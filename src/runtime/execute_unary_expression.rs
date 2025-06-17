use std::borrow::Cow;
use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::{MascalUnaryOperators};
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

#[allow(dead_code)]
pub fn execute_unary_expression<'a>(
    target: Box<MascalExpression>, operator: MascalUnaryOperators, exec_data: &ExecutionData<'a>
) -> Result<Cow<'a, MascalValue>, MascalError> {
    let target_value: MascalValue = execute_expression(*target, exec_data)?.into_owned();

    Ok(Cow::Owned(match operator {
        MascalUnaryOperators::Not => {MascalValue::not(target_value)}
        MascalUnaryOperators::Minus => {MascalValue::negate(&target_value)}
        MascalUnaryOperators::Typeof => {
            Ok(MascalValue::Type(target_value.as_mascal_type()?))
        }
    }?))
}