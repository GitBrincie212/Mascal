use std::borrow::Cow;
use std::ops::Deref;
use crate::defs::blocks::{ExecutionBlock, MascalParameter, ScopedBlocks};
use crate::defs::builtins::builtin_functions::BUILT_IN_FUNCTION_TABLE;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::InfinityControl;
use crate::defs::statements::MascalStatement;
use crate::defs::types::{to_processed_type, MascalType, MascalUnprocessedType};
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::{ExecutionData};
use crate::runtime::execute_statement::execute_statement;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{create_variable_table, VariableData, VariableTable};

#[allow(dead_code)]
pub fn execute_function_call<'a>(
    function: MascalExpression, arguments: Vec<MascalExpression>, exec_data: &ExecutionData<'a>
) -> Result<Cow<'a, MascalValue>, MascalError> {
    let fn_name: String;
    match function {
        MascalExpression::SymbolicExpression(name) => {fn_name = name}
        _ => {return Err(MascalError {
            error_type: MascalErrorType::TypeError,
            line: 0,
            character: 0,
            source: String::from("Expected an identifier for the function call but got a expression")
        });}
    }
    if let Some(built_in_func) = BUILT_IN_FUNCTION_TABLE.get(&fn_name)  {
        let mut args: Vec<MascalValue> = Vec::with_capacity(arguments.len());
        for (index, arg) in arguments.iter().enumerate() {
            let result: MascalValue = execute_expression(arg.clone(), exec_data)?
                .into_owned();
            if args.len() > arguments.len() {
                let arg_type = &(*built_in_func).fixed_argument_types[index];
                if !result.is_type_of(arg_type) {
                    return Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        line: 0,
                        character: 0,
                        source: format!("Expected a type of \"{:?}\" but got \"{:?}\"", arg_type, result)
                    })
                }
            } else {
                if !built_in_func.supports_dynamic_arguments {
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
        let val: Option<MascalValue> = (built_in_func.execution)(args)?;
        return Ok(Cow::Owned(val.unwrap_or(MascalValue::NULL)));
    }
    let mut func_parameters: &Vec<MascalParameter> = &Vec::new();
    let mut func_return_type: Option<MascalUnprocessedType> = None;
    let mut wrapped_func_exec_block: Option<ExecutionBlock> = None;
    for scoped_block in exec_data.scoped_blocks.deref().iter() {
        match scoped_block {
            ScopedBlocks::PROGRAM(..) => {unreachable!()}
            ScopedBlocks::FUNCTION {
                name,
                parameters,
                return_type,
                execution_block
            } => {
                if name == &fn_name {
                    func_return_type = return_type.clone();
                    func_parameters = parameters;
                    wrapped_func_exec_block = Some(execution_block.clone());
                    break;
                }
            }
        }
    }
    if wrapped_func_exec_block.is_none() {
        return Err(MascalError {
            error_type: MascalErrorType::RuntimeError,
            character: 0,
            line: 0,
            source: format!("Unidentified function with the name of {:?}", fn_name)
        })
    }
    let func_exec_block: ExecutionBlock = wrapped_func_exec_block.unwrap();
    let mut scoped_variable_table: VariableTable = create_variable_table(&func_exec_block)?;
    for (index, parameter) in func_parameters.iter().enumerate() {
        let data: &mut VariableData = scoped_variable_table
            .get_mut(&parameter.name)
            .unwrap();
        let result: MascalValue = execute_expression(arguments[index].clone(), exec_data)?.into_owned();
        data.value = Some(result);
    }
    let processed_return_type: Option<MascalType> = if let Some(return_type) = func_return_type {
        Some(to_processed_type(return_type)?)
    } else {None};
    for statement in &func_exec_block.body {
        match &statement {
            &MascalStatement::Declaration {
                variable,
                value
            } => {
                if variable == &fn_name {
                    let computed_value: Cow<MascalValue> = execute_expression(value.clone(), &ExecutionData {
                        variable_table: Some(&scoped_variable_table),
                        infinity_control: InfinityControl::DisallowInfinity,
                        scoped_blocks: exec_data.scoped_blocks.clone()
                    })?;
                    if processed_return_type.is_none() {
                        return Err(MascalError {
                            error_type: MascalErrorType::RuntimeError,
                            character: 0,
                            line: 0,
                            source: format!(
                                "Expected no value to be returned, but returned {:?}",
                                computed_value.into_owned()
                            )
                        })
                    }
                    if !computed_value.is_type_of(&processed_return_type.clone().unwrap()) {
                        return Err(MascalError {
                            error_type: MascalErrorType::RuntimeError,
                            character: 0,
                            line: 0,
                            source: format!(
                                "Expected value of type {:?} to be returned, but returned {:?}",
                                &processed_return_type.unwrap(),
                                computed_value.as_type_string()
                            )
                        })
                    }
                    return Ok(Cow::Owned(computed_value.into_owned()));
                }
            }

            _ => {}
        }
        scoped_variable_table = execute_statement(
            Cow::Borrowed(statement), scoped_variable_table, exec_data.scoped_blocks.clone()
        )?;
    }
    if processed_return_type.clone().is_some() {
        return Err(MascalError {
            error_type: MascalErrorType::RuntimeError,
            character: 0,
            line: 0,
            source: String::from("Expected a value to be returned, but nothing was returned")
        });
    }

    Ok(Cow::Owned(MascalValue::NULL))
}