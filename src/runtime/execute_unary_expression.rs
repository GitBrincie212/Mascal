use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::MascalUnaryOperators;
use crate::runtime::ExecutionData;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::values::MascalValue;

#[allow(dead_code)]
#[inline(always)]
pub fn execute_unary_expression(
    target: MascalExpression,
    operator: MascalUnaryOperators,
    exec_data: &mut ExecutionData,
) -> Result<MascalValue, MascalError> {
    let target_value: MascalValue = execute_expression(target, exec_data)?;

    match operator {
        MascalUnaryOperators::Not => MascalValue::not(&target_value),
        MascalUnaryOperators::Minus => MascalValue::negate(&target_value),
        MascalUnaryOperators::Typeof => Ok(MascalValue::Type(MascalValue::as_mascal_type(
            &target_value,
        )?)),
    }
}
