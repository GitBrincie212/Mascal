use crate::defs::blocks::{ExecutionBlock, MascalParameter, ScopedBlocks};
use crate::defs::builtins::builtin_functions::BUILT_IN_FUNCTION_TABLE;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::types::{MascalType, MascalUnprocessedType, to_processed_type};
use crate::runtime::ExecutionData;
use crate::runtime::execute_builtin_function::execute_builtin_function;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::execute_statement::{SemanticContext, StatementResults, execute_statement};
use crate::runtime::execute_typecast::{execute_processed_typecast, execute_typecast};
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{VariableData, VariableTable, create_variable_table};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::Chars;

fn notify_mutable_params(
    mutable_parameters: Vec<(Rc<str>, Rc<str>)>,
    scoped_variable_table: Rc<RefCell<VariableTable>>,
    exec_data: Rc<RefCell<ExecutionData>>,
) {
    let outer_vartable = exec_data.borrow().variable_table.clone();
    if let Some(unwrapped_outer_vartable) = outer_vartable {
        let scope_borrow_vartable = scoped_variable_table.borrow();
        let mut outer_borrow_mut_vartable = unwrapped_outer_vartable.borrow_mut();
        for (param_name, var_name) in mutable_parameters {
            let mutable_vardata = outer_borrow_mut_vartable.get_mut(&var_name).unwrap();
            let parameter_vardata = scope_borrow_vartable.get(&param_name).unwrap();
            mutable_vardata.value = parameter_vardata.value.clone();
        }
    }
}

fn is_titlecase(s: &str) -> bool {
    let mut chars: Chars = s.chars();
    if let Some(first) = chars.next() {
        first.is_uppercase() && chars.clone().all(|c| c.is_lowercase())
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn execute_function_call(
    function: MascalExpression,
    arguments: Vec<MascalExpression>,
    exec_data: Rc<RefCell<ExecutionData>>,
) -> Result<MascalValue, MascalError> {
    let fn_name: String = match function {
        MascalExpression::Symbolic(target_name) => target_name,
        MascalExpression::Type(t) => {
            return execute_typecast(t, arguments, exec_data);
        }
        expr => {
            let value: MascalValue = execute_expression(expr, exec_data.clone())?;
            if value.is_type_of(&MascalType::Type) {
                let MascalValue::Type(extracted_type) = value else {
                    unreachable!()
                };
                if arguments.len() != 1 {
                    return Err(MascalError {
                        error_type: MascalErrorType::ArgumentError,
                        line: 0,
                        character: 0,
                        source: String::from("Expected one value but got none or multiple values"),
                    });
                }
                let value: MascalValue = execute_expression(arguments[0].clone(), exec_data)?;
                return execute_processed_typecast(extracted_type, value);
            }
            return Err(MascalError {
                error_type: MascalErrorType::TypeError,
                line: 0,
                character: 0,
                source: String::from(
                    "Expected an identifier for the function call but got a expression",
                ),
            });
        }
    };
    let lowercased: &String = &fn_name.to_lowercase();
    if fn_name == fn_name.to_uppercase() || &fn_name == lowercased || is_titlecase(&fn_name) {
        if let Some(built_in_func) = BUILT_IN_FUNCTION_TABLE.get(lowercased) {
            return execute_builtin_function(built_in_func, arguments, exec_data.clone());
        }
    }
    let mut func_parameters: &[MascalParameter] = &Vec::new();
    let mut func_return_type: Option<MascalUnprocessedType> = None;
    let mut wrapped_func_exec_block: Option<ExecutionBlock> = None;
    let exedata_binding: &ExecutionData = &exec_data.borrow();
    let borrowed_scoped_blocks: &Vec<ScopedBlocks> = &exedata_binding.scoped_blocks.borrow();
    for scoped_block in borrowed_scoped_blocks.iter() {
        match scoped_block {
            ScopedBlocks::Program(..) => {
                unreachable!()
            }
            ScopedBlocks::Function {
                name,
                parameters,
                return_type,
                execution_block,
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
            source: format!("Unidentified function with the name of {:?}", fn_name),
        });
    }
    if func_parameters.len() != arguments.len() {
        return Err(MascalError {
            error_type: MascalErrorType::ArgumentError,
            line: 0,
            character: 0,
            source: format!(
                "Expected {} argument(s) but got {} argument(s) instead",
                func_parameters.len(),
                arguments.len()
            ),
        });
    }
    let mut func_exec_block: ExecutionBlock = wrapped_func_exec_block.unwrap();
    let scoped_variable_table: Rc<RefCell<VariableTable>>;
    (scoped_variable_table, func_exec_block) = create_variable_table(func_exec_block)?;
    let mut borrowed_mut_vartable = scoped_variable_table.borrow_mut();
    let mut mutable_parameters: Vec<(Rc<str>, Rc<str>)> = Vec::with_capacity(func_parameters.len());
    for (index, parameter) in func_parameters.iter().enumerate() {
        let data: &mut VariableData = borrowed_mut_vartable.get_mut(&parameter.name).unwrap();
        if parameter.is_mutable {
            match &arguments[index] {
                MascalExpression::Symbolic(varname) => {
                    mutable_parameters.push((parameter.name.clone(), Rc::from(varname.as_str())));
                }

                _ => {
                    return Err(MascalError {
                        error_type: MascalErrorType::ArgumentError,
                        line: 0,
                        character: 0,
                        source: String::from(
                            "Expected a variable name in order to update to, but got something else",
                        ),
                    });
                }
            }
            continue;
        }
        let result: MascalValue = execute_expression(arguments[index].clone(), exec_data.clone())?;
        data.value = Some(Rc::new(RefCell::new(result)));
    }
    let processed_return_type: Option<MascalType> = if let Some(return_type) = func_return_type {
        Some(to_processed_type(return_type)?)
    } else {
        None
    };
    drop(borrowed_mut_vartable);
    for statement in func_exec_block.body.into_iter() {
        let statement_results: StatementResults = execute_statement(
            statement,
            Rc::new(SemanticContext {
                variable_table: scoped_variable_table.clone(),
                scoped_blocks: exec_data.borrow().scoped_blocks.clone(),
                function_name: Some(Rc::from(fn_name.clone())),
                in_loop: false,
            }),
        )?;
        if let Some(value) = statement_results.return_value {
            if processed_return_type.is_none() {
                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    character: 0,
                    line: 0,
                    source: format!(
                        "Expected no value to be returned, but returned {:?}",
                        value.as_string()?
                    ),
                });
            }
            let unwrapped_processed_return_type: MascalType =
                processed_return_type.clone().unwrap();
            if unwrapped_processed_return_type.get_atomic_type() != MascalType::Dynamic
                && !value.is_type_of(&unwrapped_processed_return_type)
            {
                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    character: 0,
                    line: 0,
                    source: format!(
                        "Expected value of type {} to be returned, but returned {}",
                        &processed_return_type.unwrap().as_string(),
                        value.as_type_string()?
                    ),
                });
            }
            notify_mutable_params(mutable_parameters, scoped_variable_table, exec_data.clone());
            return Ok(value);
        }
    }
    if processed_return_type.clone().is_some() {
        return Err(MascalError {
            error_type: MascalErrorType::RuntimeError,
            character: 0,
            line: 0,
            source: String::from("Expected a value to be returned, but nothing was returned"),
        });
    }

    notify_mutable_params(mutable_parameters, scoped_variable_table, exec_data.clone());

    Ok(MascalValue::Null)
}
