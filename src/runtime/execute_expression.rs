use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::types::to_processed_type;
use crate::runtime::execute_binary_expression::execute_binary_expression;
use crate::runtime::execute_function_expression::execute_function_call;
use crate::runtime::execute_unary_expression::execute_unary_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{VariableData, VariableTable};
use crate::{define_array_expression_exec, index_array_impl, uninit_cell_error};

#[allow(dead_code)]
pub fn execute_expression(
    expression: MascalExpression, exec_data: Rc<RefCell<ExecutionData>>
) -> Result<MascalValue, MascalError> {
    match expression {
        MascalExpression::LiteralExpression(value) => {
            let mv = match value {
                MascalLiteral::String(s)  => MascalValue::String(Arc::from(s)),
                MascalLiteral::Integer(i) => MascalValue::Integer(i),
                MascalLiteral::Float(f)   => MascalValue::Float(f),
                MascalLiteral::NULL => MascalValue::NULL,
                MascalLiteral::Boolean(b) => MascalValue::Boolean(b),
            };
            Ok(mv)
        }

        MascalExpression::SymbolicExpression(symbolic_expr) => {
            let var_table_option: Option<Rc<RefCell<VariableTable>>> = exec_data.borrow().variable_table.clone();
            let unwrapped_var_table: Rc<RefCell<VariableTable>> = match var_table_option {
                Some(v) => v,
                None => return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    character: 0,
                    line: 0,
                    source: format!("Variables are unavailable for this specific context (tried accessing {:?})", symbolic_expr)
                }),
            };
            let vardata: Option<VariableData> = {
                let var_table = unwrapped_var_table.borrow();
                var_table.get(&symbolic_expr).cloned()
            };

            match vardata {
                Some(data) => {
                    data.value.as_ref()
                        .map(|v| v.borrow().clone())
                        .ok_or_else(|| MascalError {
                            error_type: MascalErrorType::RuntimeError,
                            character: 0,
                            line: 0,
                            source: format!("Variable {:?} not initialized", symbolic_expr),
                        })
                },
                None => Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    character: 0,
                    line: 0,
                    source: format!("Unknown expression {:?} not found", symbolic_expr),
                })
            }
        }

        MascalExpression::IndexExpression {
            index,
            array,
            is_dynamic
        } => {
            let arr_value: MascalValue = execute_expression(*array, exec_data.clone())?;
            if !arr_value.is_array() {
                return Err(MascalError {
                    error_type: MascalErrorType::TypeError,
                    line: 0,
                    character: 0,
                    source: String::from("Expected an array type but found instead an atomic type")
                })
            }
            let index_value: MascalValue = execute_expression(*index, exec_data.clone())?;
            let num: &IntegerNum = match &index_value {
                MascalValue::Integer(i) => Ok(i),

                _ => {
                    return Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        line: 0,
                        character: 0,
                        source: format!("Expected an index type (integer) but got {:?}", index_value.as_type_string())
                    })
                }
            }?;
            let mut num_val: i128 = num.to_i128();
            match arr_value {
                MascalValue::DynamicArray(elements) => {
                    index_array_impl!(elements, is_dynamic, num_val);
                }
                MascalValue::StaticArray(elements) => {
                    index_array_impl!(elements, is_dynamic, num_val);
                }
                _ => unreachable!()
            }
        }
        
        MascalExpression::DynamicArrayExpression(array) => {
            let arr = define_array_expression_exec!(array, exec_data.clone());
            Ok(MascalValue::DynamicArray(arr))
        }
        
        MascalExpression::StaticArrayExpression(array) => {
            let arr = define_array_expression_exec!(array, exec_data.clone());
            Ok(MascalValue::StaticArray(arr.into()))
        }
        
        MascalExpression::TypeExpression(type_expr) => {
            Ok(MascalValue::Type(to_processed_type(*type_expr)?))
        }
        
        MascalExpression::UnaryExpression { value, operator } => {
            execute_unary_expression(value, operator, exec_data)
        }
        
        MascalExpression::BinaryExpression { left, operator, right } => {
            execute_binary_expression(left, operator, right, exec_data)
        }
        
        MascalExpression::CallExpression { arguments, function } => {
            execute_function_call(*function, arguments, exec_data)
        }
    }
}