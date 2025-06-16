use std::borrow::Cow;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::execute_function_expression::execute_function_call;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

pub fn execute_inner_member_expression<'a>(
    member: MascalExpression, value: MascalExpression, exec_data: &ExecutionData<'a>
) -> Result<Cow<'a, MascalValue>, MascalError> {
    let val: Cow<MascalValue> = execute_expression(member, exec_data)?;
    match value {
        MascalExpression::LiteralExpression(lit_value) => {
            todo!()
        }
        
        MascalExpression::CallExpression{function, arguments} => {
            execute_function_call(*function, arguments, exec_data)
        }
        _ => {return Err(MascalError {
            error_type: MascalErrorType::RuntimeError,
            line: 0,
            character: 0,
            source: String::from(
                "Other expressions apart from literals and function calls are not allowed when accessing inner member's value"
            )
        })}
    }
}