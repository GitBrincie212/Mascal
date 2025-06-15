use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::MascalUnaryOperators;
use crate::runtime::values::MascalValue;

#[allow(dead_code)]
pub fn execute_unary_expression(
    left: Box<MascalExpression>, operator: MascalUnaryOperators
) -> Result<MascalValue, MascalError> {
    todo!()
}