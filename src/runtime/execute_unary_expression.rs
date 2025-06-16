use std::borrow::Cow;
use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::MascalUnaryOperators;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

#[allow(dead_code)]
pub fn execute_unary_expression<'a>(
    left: Box<MascalExpression>, operator: MascalUnaryOperators, exec_data: &ExecutionData<'a>
) -> Result<Cow<'a, MascalValue>, MascalError> {
    todo!()
}