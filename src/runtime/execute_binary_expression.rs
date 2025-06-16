use std::borrow::Cow;
use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::MascalBinaryOperators;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

pub fn execute_binary_expression<'a>(
    left: Box<MascalExpression>, operator: MascalBinaryOperators, right: Box<MascalExpression>,
    exec_data: &ExecutionData<'a>
) -> Result<Cow<'a, MascalValue>, MascalError> {
    let left_value: MascalValue = execute_expression(*left, exec_data)?.into_owned();
    let right_value: MascalValue = execute_expression(*right, exec_data)?.into_owned();
    
    Ok(Cow::Owned(match operator {
        MascalBinaryOperators::Plus => {MascalValue::add(left_value, right_value, &exec_data.infinity_control)}
        MascalBinaryOperators::Minus => {MascalValue::sub(left_value, right_value, &exec_data.infinity_control)}
        MascalBinaryOperators::Multiply => {MascalValue::mul(left_value, right_value, &exec_data.infinity_control)}
        MascalBinaryOperators::Divide => {MascalValue::div(left_value, right_value, &exec_data.infinity_control)}
        /*
        MascalBinaryOperators::Modulo => {}
        MascalBinaryOperators::Equals => {}
        MascalBinaryOperators::GreaterThan => {}
        MascalBinaryOperators::LessThan => {}
        MascalBinaryOperators::GreaterThanOrEqual => {}
        MascalBinaryOperators::LessThanOrEqual => {}
        MascalBinaryOperators::And => {}
        MascalBinaryOperators::Or => {}
        MascalBinaryOperators::NotEqual => {}
        MascalBinaryOperators::Exponentiation => {}
         */
        _ => {todo!()}
    }?))
}