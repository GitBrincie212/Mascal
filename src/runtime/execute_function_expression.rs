use std::cell::RefCell;
use std::rc::Rc;
use crate::defs::blocks::{ExecutionBlock, MascalParameter, ScopedBlocks};
use crate::defs::builtins::builtin_functions::{BUILT_IN_FUNCTION_TABLE};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::statements::MascalStatement;
use crate::defs::types::{to_processed_type, MascalType, MascalUnprocessedType};
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::{ExecutionData};
use crate::runtime::execute_builtin_function::execute_builtin_function;
use crate::runtime::execute_statement::execute_statement;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{create_variable_table, VariableData, VariableTable};

#[allow(dead_code)]
pub fn execute_function_call(
    function: MascalExpression, arguments: Vec<MascalExpression>, exec_data: Rc<RefCell<ExecutionData>>
) -> Result<MascalValue, MascalError> {
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
        return execute_builtin_function(built_in_func.clone(), arguments, exec_data.clone());
    }
    let mut func_parameters: &Vec<MascalParameter> = &Vec::new();
    let mut func_return_type: Option<MascalUnprocessedType> = None;
    let mut wrapped_func_exec_block: Option<ExecutionBlock> = None;
    let exedata_binding = exec_data.borrow();
    let borrowed_scoped_blocks = exedata_binding.scoped_blocks.borrow();
    for scoped_block in borrowed_scoped_blocks.iter() {
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
                    func_parameters = &parameters;
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
    let mut func_exec_block: ExecutionBlock = wrapped_func_exec_block.unwrap();
    let scoped_variable_table: Rc<RefCell<VariableTable>>;
    (scoped_variable_table, func_exec_block) = create_variable_table(func_exec_block)?;
    let mut borrowed_mut_vartable = scoped_variable_table.borrow_mut();
    for (index, parameter) in func_parameters.into_iter().enumerate() {
        let data: &mut VariableData = borrowed_mut_vartable
            .get_mut(&parameter.name)
            .unwrap();
        let result: MascalValue = execute_expression(arguments[index].clone(), exec_data.clone())?;
        data.value = Some(Rc::new(RefCell::new(result)));
    }
    let processed_return_type: Option<MascalType> = if let Some(return_type) = func_return_type {
        Some(to_processed_type(return_type)?)
    } else {None};
    for statement in func_exec_block.body.into_iter() {
        match &statement {
            MascalStatement::Declaration {
                variable,
                value
            } => {
                match variable {
                    MascalExpression::SymbolicExpression(name) => {
                        if name == &fn_name {
                            let computed_value: MascalValue = execute_expression(
                                value.clone(),
                                Rc::new(RefCell::new(ExecutionData {
                                    variable_table: Some(scoped_variable_table.clone()),
                                    scoped_blocks: exec_data.borrow().scoped_blocks.clone()
                                }
                                )))?;
                            if processed_return_type.is_none() {
                                return Err(MascalError {
                                    error_type: MascalErrorType::RuntimeError,
                                    character: 0,
                                    line: 0,
                                    source: format!(
                                        "Expected no value to be returned, but returned {:?}",
                                        computed_value
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
                            return Ok(computed_value);
                        }
                    }
                    _ => {
                        return Err(MascalError {
                            error_type: MascalErrorType::RuntimeError,
                            character: 0,
                            line: 0,
                            source: String::from("Callables do not support using expressions for the name other than an identifier")
                        })
                    }
                }
            }

            _ => {}
        }
        execute_statement(
            statement, scoped_variable_table.clone(), exec_data.borrow().scoped_blocks.clone()
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

    Ok(MascalValue::NULL)
}