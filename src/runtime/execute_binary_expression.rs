use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::MascalBinaryOperators;
use crate::runtime::ExecutionData;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::values::MascalValue;
use std::cell::RefCell;
use std::rc::Rc;

pub fn execute_binary_expression(
    left: MascalExpression,
    operator: MascalBinaryOperators,
    right: MascalExpression,
    exec_data: Rc<RefCell<ExecutionData>>,
) -> Result<MascalValue, MascalError> {
    let left_value: MascalValue = execute_expression(left, exec_data.clone())?;
    let right_value: MascalValue = execute_expression(right, exec_data)?;

    match operator {
        MascalBinaryOperators::Plus => MascalValue::add(left_value, right_value),
        MascalBinaryOperators::Minus => MascalValue::sub(left_value, right_value),
        MascalBinaryOperators::Multiply => MascalValue::mul(left_value, right_value),
        MascalBinaryOperators::Divide => MascalValue::div(left_value, right_value),
        MascalBinaryOperators::Equals => MascalValue::equals(&left_value, &right_value),
        MascalBinaryOperators::NotEqual => MascalValue::not_equals(&left_value, &right_value),
        MascalBinaryOperators::GreaterThan => MascalValue::greater_than(&left_value, &right_value),
        MascalBinaryOperators::LessThan => MascalValue::less_than(&left_value, &right_value),
        MascalBinaryOperators::GreaterThanOrEqual => {
            MascalValue::greater_than_or_equal(&left_value, &right_value)
        }
        MascalBinaryOperators::LessThanOrEqual => {
            MascalValue::less_than_or_equal(&left_value, &right_value)
        }
        MascalBinaryOperators::Exponentiation => {
            MascalValue::exponention(&left_value, &right_value)
        }
        MascalBinaryOperators::Modulo => MascalValue::modulo(&left_value, &right_value),
        MascalBinaryOperators::And => MascalValue::and(&left_value, &right_value),
        MascalBinaryOperators::Or => MascalValue::or(&left_value, &right_value),
    }
}
