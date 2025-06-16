use std::borrow::Cow;
use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

#[allow(dead_code)]
pub fn execute_function_call<'a>(
    function: MascalExpression, right: Vec<MascalExpression>, exec_data: &ExecutionData<'a>
) -> Result<Cow<'a, MascalValue>, MascalError> {
    todo!()
}