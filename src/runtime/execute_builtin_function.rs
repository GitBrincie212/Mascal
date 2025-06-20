use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use crate::defs::builtins::builtin_functions::BuiltinFunction;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

pub fn execute_builtin_function<'a>(
    built_in_func: Arc<BuiltinFunction>, arguments: Vec<MascalExpression>,
    exec_data:  Rc<RefCell<ExecutionData>>
) -> Result<MascalValue, MascalError> {
    match built_in_func.as_ref() {
        BuiltinFunction::ValueBased {
            fixed_argument_types,
            supports_dynamic_arguments,
            execution
        } => {
            let mut args: Vec<MascalValue> = Vec::with_capacity(arguments.len());
            for (index, arg) in arguments.iter().enumerate() {
                let result: MascalValue = execute_expression(arg.clone(), exec_data.clone())?;
                if index < fixed_argument_types.len() {
                    let arg_types = &fixed_argument_types[index];
                    let mut is_atleast_one_type: bool = false;
                    for arg_type in arg_types {
                        if arg_type.is_type_of_for_value(&result) {
                            is_atleast_one_type = true;
                            break;
                        }
                    }
                    if !is_atleast_one_type {
                        if arg_types.len() == 1 {
                            return Err(MascalError {
                                error_type: MascalErrorType::TypeError,
                                line: 0,
                                character: 0,
                                source: format!(
                                    "Expected a type of {:?} but got {:?}",
                                    arg_types.first().unwrap().as_string(),
                                    result.as_type_string()
                                )
                            })
                        }
                        return Err(MascalError {
                            error_type: MascalErrorType::TypeError,
                            line: 0,
                            character: 0,
                            source: format!(
                                "Expected at least one of the types {} but got {:?}",
                                arg_types.iter()
                                    .map(|x| format!("{:?}", x.as_string()))
                                    .collect::<Vec<String>>()
                                    .join(", "),
                                result.as_type_string()
                            )
                        })
                    }
                } else {
                    if !supports_dynamic_arguments {
                        return Err(MascalError {
                            error_type: MascalErrorType::RuntimeError,
                            line: 0,
                            character: 0,
                            source: format!("Expected only {:?} parameter(s) but got {:?} parameter(s)", arguments.len(), args.len())
                        })
                    }
                }
                args.push(result);
            }
            let val: Option<MascalValue> = execution(args, exec_data.clone())?;
            Ok(val.unwrap_or(MascalValue::NULL))
        }
        
        BuiltinFunction::ExpressionBased {
            fixed_argument_types,
            supports_dynamic_arguments,
            execution,
        } => {
            let mut args: Vec<&MascalExpression> = Vec::with_capacity(arguments.len());
            for arg in arguments.iter() {
                if args.len() > fixed_argument_types.len() && !supports_dynamic_arguments {
                    return Err(MascalError {
                        error_type: MascalErrorType::RuntimeError,
                        line: 0,
                        character: 0,
                        source: format!("Expected only {:?} parameter(s) but got {:?} parameter(s)", arguments.len(), args.len())
                    })
                }
                args.push(arg);
            }
            let val: Option<MascalValue> = execution(args, exec_data.clone())?;
            Ok(val.unwrap_or(MascalValue::NULL))
        }
    }
}