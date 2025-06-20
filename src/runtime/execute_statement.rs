use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use crate::defs::blocks::ScopedBlocks;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::statements::MascalStatement;
use crate::defs::types::MascalType;
use crate::runtime::execute_declaration_statement::execute_declaration_statement;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{VariableData, VariableTable};

fn error_check_expression(
    variable_table: Rc<RefCell<VariableTable>>, mut val: MascalExpression, variable_data: &VariableData,
    variable: &String, scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>>
) -> Result<MascalValue, MascalError> {
    if let MascalExpression::LiteralExpression(literal) = &val {
        if let MascalLiteral::Float(v) = literal {
            if *variable_data.atomic_variable_type == MascalType::Integer {
                val = MascalExpression::LiteralExpression(MascalLiteral::Integer(IntegerNum::new(v.round() as i128)));
            }
        } else if let MascalLiteral::Integer(i) = literal {
            if *variable_data.atomic_variable_type == MascalType::Float {
                val = MascalExpression::LiteralExpression(MascalLiteral::Float(i.as_f64()));
            }
        }
    }
    let val_num: MascalValue = execute_expression(val, Rc::new(RefCell::new(ExecutionData {
        variable_table: Some(variable_table),
        scoped_blocks,
    })))?;
    match &val_num {
        MascalValue::Integer(_) => {
            if *variable_data.atomic_variable_type != MascalType::Integer {
                return Err(MascalError {
                    error_type: MascalErrorType::TypeError,
                    character: 0,
                    line: 0,
                    source: format!("Variable {:?} is not assigned as an integer(dynamics not supported)", variable)
                });
            }
        }

        MascalValue::Float(_) => {
            if *variable_data.atomic_variable_type != MascalType::Float {
                return Err(MascalError {
                    error_type: MascalErrorType::TypeError,
                    character: 0,
                    line: 0,
                    source: format!("Variable {:?} is not assigned as an float(dynamics not supported)", variable)
                });
            }
        }

        _ => {
            return Err(MascalError {
                error_type: MascalErrorType::TypeError,
                character: 0,
                line: 0,
                source: format!("Unsupported type used in for loop statement(variable {:?})", variable)
            });
        }
    }

    Ok(val_num)
}

pub fn execute_statement(
    statement: MascalStatement, variable_table: Rc<RefCell<VariableTable>>,
    scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>>
) -> Result<(), MascalError> {
    match statement {
        MascalStatement::ConditionalStatement(branches) => {
            for branch in branches {
                let cond: bool = if let Some(cond) = branch.condition {
                    let value: MascalValue = execute_expression(cond, Rc::new(RefCell::new(ExecutionData {
                        variable_table: Some(variable_table.clone()),
                        scoped_blocks: scoped_blocks.clone(),
                    })))?;
                    match value {
                        MascalValue::Boolean(b) => Ok(b),
                        _ => Err(MascalError {
                            line: 0,
                            character: 0,
                            error_type: MascalErrorType::RuntimeError,
                            source: format!("Expected a boolean variable to indicate {:?}",value)
                        })
                    }?
                } else {true};

                if !cond {continue;}
                for stmt in branch.statements {
                    execute_statement(
                        stmt,
                        variable_table.clone(),
                        scoped_blocks.clone()
                    )?;
                }
                break;
            }
        }
        MascalStatement::While(condition) => {
            let cond_expr: MascalExpression = condition.condition.unwrap();
            while {
                /*
                 I am aware I should avoid cloning expressions (because of their tree form). 
                 However, if I try to use references or even smart pointers, it would require sweeping 
                 changes to the entire codebase which I am not willing to do
                */
                let value: MascalValue = execute_expression(cond_expr.clone(), Rc::new(RefCell::new(ExecutionData {
                    variable_table: Some(variable_table.clone()),
                    scoped_blocks: scoped_blocks.clone(),
                })))?;
                match value {
                    MascalValue::Boolean(b) => Ok(b),
                    _ => Err(MascalError {
                        line: 0,
                        character: 0,
                        error_type: MascalErrorType::RuntimeError,
                        source: format!("Expected a boolean variable to indicate {:?}",value)
                    })
                }
            }? {
                for stmt in &condition.statements {
                    execute_statement(
                        stmt.clone(),
                        variable_table.clone(),
                        scoped_blocks.clone()
                    )?;
                }
            }
        }
        MascalStatement::For {
            variable,
            from,
            to,
            step,
            statements
        } => {
            let variable_metadata = {
                variable_table.borrow_mut();
                let borrowed_vartable = variable_table.borrow();
                let variable_data = borrowed_vartable.get(&variable).ok_or_else(||
                    MascalError {
                        error_type: MascalErrorType::RuntimeError,
                        character: 0,
                        line: 0,
                        source: format!("Variable {:?} not found", variable)
                    }
                )?;
                (
                    variable_data.clone(),
                    variable_data.is_constant,
                    variable_data.is_nullable,
                    variable_data.array_dimensions.clone(),
                    variable_data.is_dynamic_array.clone(),
                    Arc::clone(&variable_data.atomic_variable_type)
                )
            };
            let (
                variable_data,
                is_constant,
                is_nullable,
                array_dimensions,
                is_dynamic_array,
                atomic_variable_type
            ) = variable_metadata;
            let from_num: MascalValue = error_check_expression(
                variable_table.clone(), from, &variable_data, &variable, scoped_blocks.clone()
            )?;

            let to_num: MascalValue = error_check_expression(
                variable_table.clone(), to, &variable_data, &variable, scoped_blocks.clone()
            )?;

            let step_num: MascalValue = error_check_expression(
                variable_table.clone(), step, &variable_data, &variable, scoped_blocks.clone()
            )?;

            match (&from_num, &to_num, &step_num) {
                (MascalValue::Integer(..), ..) => {
                    let int_to_num: i128 = to_num.extract_as_int().unwrap();
                    let int_step_num: i128 = step_num.extract_as_int().unwrap();
                    let mut curr: i128 = from_num.extract_as_int().unwrap();
                    while curr <= int_to_num {
                        {
                            let mut mutable_borrow_vartable = variable_table.borrow_mut();
                            mutable_borrow_vartable.insert(variable.clone(), VariableData {
                                value: Some(Rc::new(RefCell::new(MascalValue::Integer(IntegerNum::new(curr))))),
                                is_constant,
                                is_nullable,
                                array_dimensions: array_dimensions.clone(),
                                is_dynamic_array: is_dynamic_array.clone(),
                                atomic_variable_type: Arc::clone(&atomic_variable_type),
                            });
                        }
                        for statement in &statements {
                            execute_statement(
                                statement.clone(), variable_table.clone(), scoped_blocks.clone()
                            )?;
                        }
                        curr += int_step_num;
                    }
                }

                (MascalValue::Float(..), ..) => {
                    let float_to_num: f64 = to_num.extract_as_float().unwrap();
                    let float_step_num: f64 = step_num.extract_as_float().unwrap();
                    let mut curr: f64 = from_num.extract_as_float().unwrap();
                    while curr <= float_to_num {
                        {
                            let mut mutable_borrow_vartable = variable_table.borrow_mut();
                            mutable_borrow_vartable.insert(variable.clone(), VariableData {
                                value: Some(Rc::new(RefCell::new(MascalValue::Float(curr)))),
                                is_constant,
                                is_nullable,
                                array_dimensions: array_dimensions.clone(),
                                is_dynamic_array: is_dynamic_array.clone(),
                                atomic_variable_type: Arc::clone(&atomic_variable_type),
                            });
                        }
                        for statement in &statements {
                            execute_statement(
                                statement.clone(), variable_table.clone(), scoped_blocks.clone()
                            )?;
                        }
                        curr += float_step_num;
                    }
                }
                _ => {unreachable!()}
            }

            return Ok(());
        }
        MascalStatement::ExpressionStatement(expression) => {
            execute_expression(expression, Rc::new(RefCell::new(ExecutionData {
                variable_table: Some(variable_table.clone()),
                scoped_blocks,
            })))?;
        }
        MascalStatement::Declaration { variable, value } => {
            execute_declaration_statement(variable, value, variable_table.clone(), scoped_blocks.clone())?;
        }
        MascalStatement::Throw {
            error_type,
            message
        } => {
            let mascal_error_type: MascalErrorType = match error_type.as_str() {
                "TypeError" => MascalErrorType::TypeError,
                "RuntimeError" => MascalErrorType::RuntimeError,
                "OverflowError" => MascalErrorType::OverflowError,
                "UndefinedOperationError" => MascalErrorType::UndefinedOperation,
                "IndexError" => MascalErrorType::IndexError,
                "InputError" => MascalErrorType::InputError,
                "ArgumentError" => MascalErrorType::ArgumentError,
                "ValueError" => MascalErrorType::ValueError,
                _ => {
                    return Err(MascalError {
                        error_type: MascalErrorType::UndefinedErrorType,
                        character: 0,
                        line: 0,
                        source: String::from("Use of an undefined usable error type in the throw statement (perhaps a typo?)"),
                    })
                }
            };
            return Err(MascalError {
                error_type: mascal_error_type,
                character: 0,
                line: 0,
                source: message
            })
        }
    };

    Ok(())
}