use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;

impl MascalValue {
    pub fn add(left: MascalValue, right: MascalValue) -> Result<MascalValue, MascalError> {
        match (left, right) {
            (MascalValue::Integer(left), MascalValue::Integer(right)) => {todo!()}
            (MascalValue::Integer(left), MascalValue::Float(right)) => {todo!()}
            (MascalValue::Float(left), MascalValue::Integer(right)) => {todo!()}
            (MascalValue::String(left), MascalValue::String(right)) => {todo!()}
            (MascalValue::DynamicArray(left), MascalValue::DynamicArray(right)) => {todo!()}
            (MascalValue::Dynamic(left), right) => {
                MascalValue::add(*left, right)
            }
            (left, right) => {
                Err(MascalError {
                    character: 0,
                    line: 0,
                    error_type: MascalErrorType::UndefinedOperation,
                    source: format!(
                        "Cannot operate between the types {:?} and {:?}", 
                        left.as_string(), 
                        right.as_string()
                    ),
                })
            }
        }
    }
}