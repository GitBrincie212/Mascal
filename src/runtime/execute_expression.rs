use std::borrow::Cow;
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

macro_rules! define_array_expression_exec {
    ($array: expr, $variable_table: expr, $arr_type:expr) => {
        let mut arr: Vec<MascalValue> = Vec::with_capacity($array.len());
        for expr in $array {
            match execute_expression(expr, $variable_table)? {
                Cow::Owned(v)   => arr.push(v),
                Cow::Borrowed(v) => arr.push(v.clone()),
            }
        }
        let arr = $arr_type(arr);
        return Ok(Cow::Owned(arr));
    };
}

#[allow(dead_code)]
pub fn execute_expression<'a>(expression: MascalExpression, exec_data: &ExecutionData<'a>) -> Result<Cow<'a, MascalValue>, MascalError> {
    match expression {
        MascalExpression::LiteralExpression(value) => {
            let mv = match value {
                MascalLiteral::String(s)  => MascalValue::String(s),
                MascalLiteral::Integer(i) => MascalValue::Integer(i),
                MascalLiteral::Float(f)   => MascalValue::Float(f),
                MascalLiteral::NULL       => MascalValue::NULL,
                MascalLiteral::Boolean(b) => MascalValue::Boolean(b),
            };
            Ok(Cow::Owned(mv))
        }
        
        MascalExpression::SymbolicExpression(symbolic_expr) => {
            if exec_data.variable_table.is_none() {
                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    character: 0,
                    line: 0,
                    source: format!("Variables are not allowed to be expressed in types {:?}", symbolic_expr)
                });
            }
            if let Some(variable_data) = exec_data.variable_table.unwrap().get(&symbolic_expr) {
                if let Some(unwrapped_result) = &variable_data.value {
                    return Ok(Cow::Borrowed(unwrapped_result));
                }

                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    character: 0,
                    line: 0,
                    source: format!("Could not find the variable with the name {:?}", symbolic_expr)
                });
            }
            Err(MascalError {
                error_type: MascalErrorType::RuntimeError,
                character: 0,
                line: 0,
                source: format!("Could not find the variable with the name {:?}", symbolic_expr)
            })
        }
        
        MascalExpression::DynamicArrayExpression(array) => {
            define_array_expression_exec!(array, exec_data, MascalValue::StaticArray);
        }
        
        MascalExpression::StaticArrayExpression(array) => {
            define_array_expression_exec!(array, exec_data, MascalValue::DynamicArray);
        }
        
        MascalExpression::TypeExpression(type_expr) => {
            let t = MascalValue::Type(to_processed_type(*type_expr)?);
            Ok(Cow::Owned(t))
        }
        
        MascalExpression::UnaryExpression { value, operator } => {
            execute_unary_expression(value, operator, &exec_data)
        }
        
        MascalExpression::BinaryExpression { left, operator, right } => {
            execute_binary_expression(left, operator, right, &exec_data)
        }
        
        MascalExpression::InnerMemberAccessExpression { member, value } => {
            execute_inner_member_expression(*member, *value, &exec_data)
        }
        
        MascalExpression::CallExpression { arguments, function } => {
            execute_function_call(*function, arguments, &exec_data)
        }
    }
}