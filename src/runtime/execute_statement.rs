use crate::defs::blocks::ScopedBlocks;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::loop_flags::LoopFlags;
use crate::defs::statements::MascalStatement;
use crate::defs::types::MascalType;
use crate::runtime::ExecutionData;
use crate::runtime::execute_declaration_statement::execute_declaration_statement;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{VariableData, VariableTable};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

pub struct SemanticContext {
    pub variable_table: Rc<RefCell<VariableTable>>,
    pub scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>>,
    pub function_name: Option<Rc<str>>,
    pub in_loop: bool,
}

impl SemanticContext {
    pub fn create_from(
        semantic_context: Rc<SemanticContext>,
        function_name: Option<Rc<str>>,
    ) -> Rc<Self> {
        Rc::new(SemanticContext {
            function_name,
            variable_table: semantic_context.variable_table.clone(),
            scoped_blocks: semantic_context.scoped_blocks.clone(),
            in_loop: semantic_context.in_loop,
        })
    }

    pub fn create_for_loop_from(
        semantic_context: Rc<SemanticContext>,
        function_name: Option<Rc<str>>,
    ) -> Rc<Self> {
        Rc::new(SemanticContext {
            function_name,
            variable_table: semantic_context.variable_table.clone(),
            scoped_blocks: semantic_context.scoped_blocks.clone(),
            in_loop: true,
        })
    }
}

pub struct StatementResults {
    pub return_value: Option<MascalValue>,
    pub loop_flag: LoopFlags,
}

#[inline(always)]
fn error_check_expression(
    variable_table: Rc<RefCell<VariableTable>>,
    mut val: MascalExpression,
    variable_data: &VariableData,
    variable: &String,
    scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>>,
) -> Result<MascalValue, MascalError> {
    if let MascalExpression::Literal(literal) = &val {
        if let MascalLiteral::Float(v) = literal {
            if *variable_data.atomic_variable_type == MascalType::Integer {
                val = MascalExpression::Literal(MascalLiteral::Integer(IntegerNum::new(
                    v.round() as i128
                )));
            }
        } else if let MascalLiteral::Integer(i) = literal {
            if *variable_data.atomic_variable_type == MascalType::Float {
                val = MascalExpression::Literal(MascalLiteral::Float(i.as_f64()));
            }
        }
    }
    let val_num: MascalValue = execute_expression(
        val,
        &mut ExecutionData {
            variable_table: Some(variable_table),
            scoped_blocks,
        },
    )?;
    match &val_num {
        MascalValue::Integer(_) => {
            if *variable_data.atomic_variable_type != MascalType::Integer {
                return Err(MascalError {
                    error_type: MascalErrorType::TypeError,
                    character: 0,
                    line: 0,
                    source: format!(
                        "Variable {:?} is not assigned as an integer(dynamics not supported)",
                        variable
                    ),
                });
            }
        }

        MascalValue::Float(_) => {
            if *variable_data.atomic_variable_type != MascalType::Float {
                return Err(MascalError {
                    error_type: MascalErrorType::TypeError,
                    character: 0,
                    line: 0,
                    source: format!(
                        "Variable {:?} is not assigned as an float(dynamics not supported)",
                        variable
                    ),
                });
            }
        }

        _ => {
            return Err(MascalError {
                error_type: MascalErrorType::TypeError,
                character: 0,
                line: 0,
                source: format!(
                    "Unsupported type used in for loop statement(variable {:?})",
                    variable
                ),
            });
        }
    }

    Ok(val_num)
}

pub fn execute_statement(
    statement: MascalStatement,
    semantic_context: Rc<SemanticContext>,
) -> Result<StatementResults, MascalError> {
    match statement {
        MascalStatement::ConditionalStatement(branches) => {
            for branch in branches {
                let cond: bool = if let Some(cond) = branch.condition {
                    let value: MascalValue = execute_expression(
                        cond,
                        &mut ExecutionData {
                            variable_table: Some(semantic_context.variable_table.clone()),
                            scoped_blocks: semantic_context.scoped_blocks.clone(),
                        },
                    )?;
                    match value {
                        MascalValue::Boolean(b) => Ok(b),
                        _ => Err(MascalError {
                            line: 0,
                            character: 0,
                            error_type: MascalErrorType::RuntimeError,
                            source: format!(
                                "Expected a boolean variable on the condition but got {:?}",
                                value.as_string()?
                            ),
                        }),
                    }?
                } else {
                    true
                };

                if !cond {
                    continue;
                }
                for stmt in branch.statements {
                    let statement_results: StatementResults = execute_statement(
                        stmt,
                        SemanticContext::create_from(
                            semantic_context.clone(),
                            semantic_context.function_name.clone(),
                        ),
                    )?;
                    if statement_results.return_value.is_some() {
                        return Ok(statement_results);
                    }
                    if statement_results.loop_flag != LoopFlags::None {
                        return Ok(statement_results);
                    }
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
                let value: MascalValue = execute_expression(
                    cond_expr.clone(),
                    &mut ExecutionData {
                        variable_table: Some(semantic_context.variable_table.clone()),
                        scoped_blocks: semantic_context.scoped_blocks.clone(),
                    },
                )?;
                match value {
                    MascalValue::Boolean(b) => Ok(b),
                    _ => Err(MascalError {
                        line: 0,
                        character: 0,
                        error_type: MascalErrorType::RuntimeError,
                        source: format!(
                            "Expected a boolean variable on the condition but got {:?}",
                            value
                        ),
                    }),
                }
            }? {
                for stmt in &condition.statements {
                    let statement_results: StatementResults = execute_statement(
                        stmt.clone(),
                        SemanticContext::create_for_loop_from(
                            semantic_context.clone(),
                            semantic_context.function_name.clone(),
                        ),
                    )?;
                    if statement_results.loop_flag == LoopFlags::Break {
                        return Ok(StatementResults {
                            return_value: None,
                            loop_flag: LoopFlags::None,
                        });
                    } else if statement_results.loop_flag == LoopFlags::Continue {
                        break;
                    }
                    if statement_results.return_value.is_some() {
                        return Ok(statement_results);
                    }
                }
            }
        }
        MascalStatement::For {
            variable,
            from,
            to,
            step,
            statements,
        } => {
            let variable_metadata = {
                let borrowed_vartable = semantic_context.variable_table.borrow();
                let variable_data =
                    borrowed_vartable
                        .get(variable.as_str())
                        .ok_or_else(|| MascalError {
                            error_type: MascalErrorType::RuntimeError,
                            character: 0,
                            line: 0,
                            source: format!("Variable {:?} not found", variable),
                        })?;
                (
                    variable_data.clone(),
                    variable_data.is_constant,
                    variable_data.is_nullable,
                    variable_data.array_dimensions.clone(),
                    variable_data.is_dynamic_array.clone(),
                    Arc::clone(&variable_data.atomic_variable_type),
                )
            };
            let (
                variable_data,
                is_constant,
                is_nullable,
                array_dimensions,
                is_dynamic_array,
                atomic_variable_type,
            ) = variable_metadata;
            let from_num: MascalValue = error_check_expression(
                semantic_context.variable_table.clone(),
                from,
                &variable_data,
                &variable,
                semantic_context.scoped_blocks.clone(),
            )?;

            let to_num: MascalValue = error_check_expression(
                semantic_context.variable_table.clone(),
                to,
                &variable_data,
                &variable,
                semantic_context.scoped_blocks.clone(),
            )?;

            let step_num: MascalValue = error_check_expression(
                semantic_context.variable_table.clone(),
                step,
                &variable_data,
                &variable,
                semantic_context.scoped_blocks.clone(),
            )?;

            match (&from_num, &to_num, &step_num) {
                (MascalValue::Integer(..), ..) => {
                    let int_to_num: i128 = to_num.extract_as_int().unwrap();
                    let int_step_num: i128 = step_num.extract_as_int().unwrap();
                    let mut curr: i128 = from_num.extract_as_int().unwrap();
                    let varname: Rc<str> = Rc::from(variable);
                    while curr <= int_to_num {
                        {
                            let mut mutable_borrow_vartable =
                                semantic_context.variable_table.borrow_mut();
                            mutable_borrow_vartable.insert(
                                varname.clone(),
                                VariableData {
                                    value: Some(Rc::new(RefCell::new(MascalValue::Integer(
                                        IntegerNum::new(curr),
                                    )))),
                                    is_constant,
                                    is_nullable,
                                    array_dimensions: array_dimensions.clone(),
                                    is_dynamic_array: is_dynamic_array.clone(),
                                    atomic_variable_type: Arc::clone(&atomic_variable_type),
                                },
                            );
                        }
                        for statement in &statements {
                            let statement_results: StatementResults = execute_statement(
                                statement.clone(),
                                SemanticContext::create_for_loop_from(
                                    semantic_context.clone(),
                                    semantic_context.function_name.clone(),
                                ),
                            )?;
                            if statement_results.loop_flag == LoopFlags::Break {
                                return Ok(StatementResults {
                                    return_value: None,
                                    loop_flag: LoopFlags::None,
                                });
                            } else if statement_results.loop_flag == LoopFlags::Continue {
                                break;
                            }
                            if statement_results.return_value.is_some() {
                                return Ok(statement_results);
                            };
                        }
                        curr += int_step_num;
                    }
                }

                (MascalValue::Float(..), ..) => {
                    let float_to_num: f64 = to_num.extract_as_float().unwrap();
                    let float_step_num: f64 = step_num.extract_as_float().unwrap();
                    let mut curr: f64 = from_num.extract_as_float().unwrap();
                    let varname: Rc<str> = Rc::from(variable);
                    while curr <= float_to_num {
                        {
                            let mut mutable_borrow_vartable =
                                semantic_context.variable_table.borrow_mut();
                            mutable_borrow_vartable.insert(
                                varname.clone(),
                                VariableData {
                                    value: Some(Rc::new(RefCell::new(MascalValue::Float(curr)))),
                                    is_constant,
                                    is_nullable,
                                    array_dimensions: array_dimensions.clone(),
                                    is_dynamic_array: is_dynamic_array.clone(),
                                    atomic_variable_type: Arc::clone(&atomic_variable_type),
                                },
                            );
                        }
                        for statement in &statements {
                            let statement_results: StatementResults = execute_statement(
                                statement.clone(),
                                SemanticContext::create_for_loop_from(
                                    semantic_context.clone(),
                                    semantic_context.function_name.clone(),
                                ),
                            )?;
                            if statement_results.return_value.is_some() {
                                return Ok(statement_results);
                            };
                        }
                        curr += float_step_num;
                    }
                }
                _ => {
                    unreachable!()
                }
            }

            return Ok(StatementResults {
                return_value: None,
                loop_flag: LoopFlags::None,
            });
        }
        MascalStatement::ExpressionStatement(expression) => {
            execute_expression(
                expression,
                &mut ExecutionData {
                    variable_table: Some(semantic_context.variable_table.clone()),
                    scoped_blocks: semantic_context.scoped_blocks.clone(),
                },
            )?;
        }
        MascalStatement::Declaration { variable, value } => {
            return execute_declaration_statement(
                variable,
                value,
                semantic_context.variable_table.clone(),
                semantic_context.scoped_blocks.clone(),
            );
        }
        MascalStatement::Throw {
            error_type,
            message,
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
                        source: String::from(
                            "Use of an undefined usable error type in the throw statement (perhaps a typo?)",
                        ),
                    });
                }
            };
            return Err(MascalError {
                error_type: mascal_error_type,
                character: 0,
                line: 0,
                source: message,
            });
        }
        MascalStatement::Break => {
            if !semantic_context.in_loop {
                return Err(MascalError {
                    error_type: MascalErrorType::ContextError,
                    character: 0,
                    line: 0,
                    source: String::from(
                        "Break statement is not allowed outside of a loop statement",
                    ),
                });
            }
            return Ok(StatementResults {
                return_value: None,
                loop_flag: LoopFlags::Break,
            });
        }

        MascalStatement::Continue => {
            if !semantic_context.in_loop {
                return Err(MascalError {
                    error_type: MascalErrorType::ContextError,
                    character: 0,
                    line: 0,
                    source: String::from(
                        "Continue statement is not allowed outside of a loop statement",
                    ),
                });
            }
            return Ok(StatementResults {
                return_value: None,
                loop_flag: LoopFlags::Continue,
            });
        }
    };

    Ok(StatementResults {
        return_value: None,
        loop_flag: LoopFlags::None,
    })
}
