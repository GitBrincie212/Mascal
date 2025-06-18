use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::types::to_processed_type;
use crate::runtime::execute_binary_expression::execute_binary_expression;
use crate::runtime::execute_function_expression::execute_function_call;
use crate::runtime::execute_inner_member_expression::execute_inner_member_expression;
use crate::runtime::execute_unary_expression::execute_unary_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{VariableData, VariableTable};

macro_rules! define_array_expression_exec {
    ($array: expr, $variable_table: expr, $arr_type:expr) => {
        let mut arr: Vec<MascalValue> = Vec::with_capacity($array.len());
        for expr in $array {
            arr.push(execute_expression(expr, $variable_table)?);
        }
        let arr = $arr_type(Arc::new(arr));
        return Ok(arr);
    };
}

#[allow(dead_code)]
pub fn execute_expression(
    expression: MascalExpression, exec_data: Rc<RefCell<ExecutionData>>
) -> Result<MascalValue, MascalError> {
    match expression {
        MascalExpression::LiteralExpression(value) => {
            let mv = match value {
                MascalLiteral::String(s)  => MascalValue::String(Arc::new(s)),
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
                    source: format!("Variable {:?} not found", symbolic_expr),
                })
            }
        }
        
        MascalExpression::DynamicArrayExpression(array) => {
            define_array_expression_exec!(array, exec_data.clone(), MascalValue::StaticArray);
        }
        
        MascalExpression::StaticArrayExpression(array) => {
            define_array_expression_exec!(array, exec_data.clone(), MascalValue::DynamicArray);
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
        
        MascalExpression::InnerMemberAccessExpression { member, value } => {
            execute_inner_member_expression(*member, *value, exec_data)
        }
        
        MascalExpression::CallExpression { arguments, function } => {
            execute_function_call(*function, arguments, exec_data)
        }
    }
}