use std::borrow::Cow;
use std::rc::Rc;
use crate::defs::blocks::ScopedBlocks;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::InfinityControl;
use crate::defs::literal::MascalLiteral;
use crate::defs::statements::MascalStatement;
use crate::defs::types::MascalType;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{VariableData, VariableTable};

fn error_check_expression(
    variable_table: &VariableTable, mut val: MascalExpression, variable_data: &VariableData,
    variable: &String, scoped_blocks: Rc<Vec<ScopedBlocks>>
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
    let val_num: MascalValue = execute_expression(val, &ExecutionData {
        variable_table: Some(&variable_table),
        infinity_control: InfinityControl::DisallowInfinity,
        scoped_blocks,
    })?.into_owned();
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

pub fn execute_statement<'a>(
    statement: Cow<MascalStatement>, mut variable_table: VariableTable, scoped_blocks: Rc<Vec<ScopedBlocks>>
) -> Result<VariableTable, MascalError> {
    match statement.into_owned() {
        MascalStatement::ConditionalStatement(branches) => {
            for branch in branches {
                let cond: bool = if let Some(cond) = branch.condition {
                    let cond_expr = Cow::Borrowed(&cond).into_owned();
                    let value: MascalValue = execute_expression(cond_expr, &ExecutionData {
                        variable_table: Some(&variable_table),
                        infinity_control: InfinityControl::AllowInfinity,
                        scoped_blocks: scoped_blocks.clone(),
                    })?.into_owned();
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

                if !cond {return Ok(variable_table)}
                for stmt in branch.statements {
                    variable_table = execute_statement(
                        Cow::Borrowed(&stmt),
                        variable_table,
                        scoped_blocks.clone()
                    )?;
                }
            }
        }
        MascalStatement::While(condition) => {
            while {
                let cond_expr = Cow::Borrowed(&condition).into_owned().condition.unwrap();
                let value: MascalValue = execute_expression(cond_expr, &ExecutionData {
                    variable_table: Some(&variable_table),
                    infinity_control: InfinityControl::AllowInfinity,
                    scoped_blocks: scoped_blocks.clone(),
                })?.into_owned();
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
                    variable_table = execute_statement(
                        Cow::Borrowed(stmt),
                        variable_table,
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
            let wrapped_variable_data: Option<VariableData> = variable_table.get(&variable)
                .map(|x| x.clone());
            if wrapped_variable_data.is_none() {
                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    character: 0,
                    line: 0,
                    source: format!("Could not find the variable with the name {:?}", variable)
                });
            }
            let variable_data: VariableData = wrapped_variable_data.unwrap();
            let from_num: MascalValue = error_check_expression(
                &variable_table, from, &variable_data, &variable, scoped_blocks.clone()
            )?;
            let to_num: MascalValue = error_check_expression(
                &variable_table, to, &variable_data, &variable, scoped_blocks.clone()
            )?;
            let step_num: MascalValue = error_check_expression(
                &variable_table, step, &variable_data, &variable, scoped_blocks.clone()
            )?;
            match (&from_num, &to_num, &step_num) {
                (MascalValue::Integer(..), ..) => {
                    let int_to_num: i128 = to_num.extract_as_int().unwrap();
                    let int_step_num: i128 = step_num.extract_as_int().unwrap();
                    let mut curr: i128 = from_num.extract_as_int().unwrap();
                    while curr <= int_to_num {
                        variable_table.insert(variable.clone(), variable_data.clone_with_new_value(Some(
                            MascalValue::Integer(IntegerNum::new(curr))
                        )));
                        for statement in &statements {
                            variable_table = execute_statement(
                                Cow::Borrowed(statement), variable_table, scoped_blocks.clone()
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
                        variable_table.insert(variable.clone(), variable_data.clone_with_new_value(Some(MascalValue::Float(curr))));
                        for statement in &statements {
                            variable_table = execute_statement(
                                Cow::Borrowed(statement), variable_table, scoped_blocks.clone()
                            )?;
                        }
                        curr += float_step_num;
                    }
                }
                _ => {}
            }

        }
        MascalStatement::ExpressionStatement(expression) => {
            execute_expression(expression, &ExecutionData {
                variable_table: Some(&variable_table),
                infinity_control: InfinityControl::AllowInfinity,
                scoped_blocks,
            })?;
        }
        MascalStatement::Declaration { variable, value } => {
            let variable_data: Option<&VariableData> = variable_table.get(&variable);
            if variable_data.is_none() {
                return Err(MascalError {
                    line: 0,
                    character: 0,
                    error_type: MascalErrorType::RuntimeError,
                    source: format!("Expected a variable name, however got an unknown one called {:?}",variable)
                })
            }
            let unwrapped_data: Cow<VariableData> = Cow::Borrowed(variable_data.unwrap());
            if unwrapped_data.is_constant {
                return Err(MascalError {
                    line: 0,
                    character: 0,
                    error_type: MascalErrorType::RuntimeError,
                    source: format!("Cannot assign a new value to the constant variable called {:?}",variable)
                })
            }
            let mut owned_data: VariableData = unwrapped_data.into_owned();
            let value: Cow<MascalValue> = execute_expression(value, &ExecutionData {
                variable_table: Some(&variable_table),
                infinity_control: owned_data.infinity_control.clone().into_owned(),
                scoped_blocks,
            })?;
            owned_data.value = Some(value.into_owned());
            variable_table.insert(variable, owned_data);
        }
        MascalStatement::Throw { .. } => {}
    };

    Ok(variable_table)
}