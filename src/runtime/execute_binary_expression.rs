use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::MascalBinaryOperators;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::values::MascalValue;

pub fn execute_binary_expression(
    left: Box<MascalExpression>, operator: MascalBinaryOperators, right: Box<MascalExpression>
) -> Result<MascalValue, MascalError> {
    let left_value: MascalValue = execute_expression(*left)?;
    let right_value: MascalValue = execute_expression(*right)?;
    
    todo!()
    /*
    match operator {
        MascalBinaryOperators::Plus => {}
        MascalBinaryOperators::Minus => {}
        MascalBinaryOperators::Multiply => {}
        MascalBinaryOperators::Divide => {}
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
    }
     */
}