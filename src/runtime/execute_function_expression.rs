use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::runtime::values::MascalValue;

#[allow(dead_code)]
pub fn execute_function_call(
    function: MascalExpression, right: Vec<MascalExpression>
) -> Result<MascalValue, MascalError> {
    todo!()
}