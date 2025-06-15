use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::types::to_processed_type;
use crate::runtime::execute_binary_expression::execute_binary_expression;
use crate::runtime::execute_function_expression::execute_function_call;
use crate::runtime::execute_inner_member_expression::execute_inner_member_expression;
use crate::runtime::execute_unary_expression::execute_unary_expression;
use crate::runtime::values::MascalValue;

#[allow(dead_code)]
pub fn execute_expression(expression: MascalExpression) -> Result<MascalValue, MascalError> {
    match expression {
        MascalExpression::LiteralExpression(value) => {
            match value {
                MascalLiteral::String(string) => { Ok(MascalValue::String(string)) },
                MascalLiteral::Integer(int) => { Ok(MascalValue::Integer(int)) },
                MascalLiteral::Float(float) => { Ok(MascalValue::Float(float)) },
                MascalLiteral::NULL => { Ok(MascalValue::NULL) },
                MascalLiteral::Boolean(bool) => { Ok(MascalValue::Boolean(bool)) },
            }
        }
        MascalExpression::SymbolicExpression(symbolic_expr) => {
            todo!()
        }
        
        MascalExpression::DynamicArrayExpression(array) => {
            let mut arr: Vec<MascalValue> = Vec::with_capacity(array.len());
            for expr in array {
                arr.push(execute_expression(expr)?);
            }
            Ok(MascalValue::DynamicArray(arr))
        }
        
        MascalExpression::StaticArrayExpression(array) => {
            let mut arr: Vec<MascalValue> = Vec::with_capacity(array.len());
            for expr in array {
                arr.push(execute_expression(expr)?);
            }
            Ok(MascalValue::DynamicArray(arr))
        }
        
        MascalExpression::TypeExpression(type_expr) => {
            Ok(MascalValue::Type(to_processed_type(*type_expr)?))
        }
        
        MascalExpression::UnaryExpression { value, operator } => {
            execute_unary_expression(value, operator)
        }
        
        MascalExpression::BinaryExpression { left, operator, right } => {
            execute_binary_expression(left, operator, right)
        }
        
        MascalExpression::InnerMemberAccessExpression { member, value } => {
            execute_inner_member_expression(*member, *value)
        }
        
        MascalExpression::CallExpression { arguments, function } => {
            execute_function_call(*function, arguments)
        }
    }
}