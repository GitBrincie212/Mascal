use crate::{unsupported_operation_error};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;

impl MascalValue {
    pub fn and(
        left: &MascalValue,
        right: &MascalValue,
    ) -> Result<MascalValue, MascalError> {
        match (left, right) {
            (MascalValue::Boolean(b1), MascalValue::Boolean(b2)) => Ok(
                MascalValue::Boolean(*b1 && *b2)
            ),
                
            (l, r) => unsupported_operation_error!(l, r)
        }
    }

    pub fn or(
        left: &MascalValue,
        right: &MascalValue,
    ) -> Result<MascalValue, MascalError> {
        match (left, right) {
            (MascalValue::Boolean(b1), MascalValue::Boolean(b2)) => Ok(
                MascalValue::Boolean(*b1 || *b2)
            ),

            (l, r) => unsupported_operation_error!(l, r)
        }
    }

    pub fn not(
        value: MascalValue,
    ) -> Result<MascalValue, MascalError> {
        match value {
            MascalValue::Boolean(b1) => Ok(
                MascalValue::Boolean(!b1)
            ),

            _ => Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                line: 0,
                character: 0,
                source: String::from("Cannot use the operation \"not\" on a type other than boolean")
            })
        }
    }
}
