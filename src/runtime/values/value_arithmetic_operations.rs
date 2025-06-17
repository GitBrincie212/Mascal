use crate::{define_arithmetic_fn, unsupported_operation_error, error_float_overflow};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;

impl MascalValue {
    pub fn add(
        left: MascalValue,
        right: MascalValue,
    ) -> Result<MascalValue, MascalError> {
        define_arithmetic_fn!(Self::add, left, right, add, +,
            (MascalValue::String(l), MascalValue::String(r)) => {
                Ok(MascalValue::String(l + r.as_str()))
            },

            (MascalValue::DynamicArray(l), MascalValue::DynamicArray(r)) => {
                Ok(MascalValue::DynamicArray(l.into_iter().chain(r.into_iter()).collect()))
            }
        )
    }

    pub fn sub(
        left: MascalValue,
        right: MascalValue,
    ) -> Result<MascalValue, MascalError> {
        define_arithmetic_fn!(Self::sub, left, right, sub, -)
    }

    pub fn mul(
        left: MascalValue,
        right: MascalValue,
    ) -> Result<MascalValue, MascalError> {
        define_arithmetic_fn!(Self::mul, left, right, mul, *)
    }

    pub fn div(
        left: MascalValue,
        right: MascalValue,
    ) -> Result<MascalValue, MascalError> {
        match (left, right) {
            (MascalValue::Integer(l), MascalValue::Integer(r)) => {
                Ok(MascalValue::Integer(l.div(r)?))
            }

            (l, MascalValue::Integer(r)) => {
                MascalValue::div(l, MascalValue::Float(r.as_f64()))
            }

            (MascalValue::Integer(l), r) => {
                MascalValue::div(MascalValue::Float(l.as_f64()), r)
            }

            (MascalValue::Float(l), MascalValue::Float(r)) => {
                if r == 0f64 {
                    return Err(MascalError {
                        character: 0,
                        line: 0,
                        error_type: MascalErrorType::UndefinedOperation,
                        source: String::from("Cannot divide by zero")
                    })
                }

                Ok(MascalValue::Float(l / r))
            },

            (l, r) => unsupported_operation_error!(l, r),
        }
    }

    pub fn exponention(
        left: &MascalValue,
        right: &MascalValue,
    ) -> Result<MascalValue, MascalError> {
        match (left, right) {
            (MascalValue::Integer(l), MascalValue::Integer(r)) => {
                Ok(MascalValue::Integer(l.exponentation(&r)?))
            }

            (l, MascalValue::Integer(r)) => {
                MascalValue::exponention(l, &MascalValue::Float(r.as_f64()))
            }

            (MascalValue::Integer(l), r) => {
                MascalValue::exponention(&MascalValue::Float(l.as_f64()), r)
            }

            (MascalValue::Float(l), MascalValue::Float(r)) => {
                if *l <= 0f64 {
                    return Err(MascalError {
                        character: 0,
                        line: 0,
                        error_type: MascalErrorType::UndefinedOperation,
                        source: String::from("Cannot perform exponentation with a negative or zero base")
                    })
                }
                if *r < 0f64 {
                    return Ok(MascalValue::Float(1f64 / num_traits::pow(*l, r.abs().round() as usize)));
                }
                Ok(MascalValue::Float(num_traits::pow(*l, *r as usize)))
            },

            (l, r) => unsupported_operation_error!(l, r),
        }
    }

    pub fn modulo(
        left: &MascalValue,
        right: &MascalValue,
    ) -> Result<MascalValue, MascalError> {
        match (left, right) {
            (MascalValue::Integer(l), MascalValue::Integer(r)) => {
                Ok(MascalValue::Integer(l.modulo(r.clone())?))
            }

            (l, MascalValue::Integer(r)) => {
                MascalValue::modulo(l, &MascalValue::Float(r.as_f64()))
            }

            (MascalValue::Integer(l), r) => {
                MascalValue::modulo(&MascalValue::Float(l.as_f64()), r)
            }

            (MascalValue::Float(l), MascalValue::Float(r)) => {
                if *r == 0f64 {
                    return Err(MascalError {
                        error_type: MascalErrorType::UndefinedOperation,
                        line: 0,
                        character: 0,
                        source: String::from("Cannot modulo by zero")
                    });
                }
                Ok(MascalValue::Float(*l % *r))
            },

            (l, r) => unsupported_operation_error!(l, r),
        }
    }

    pub fn negate(target: &MascalValue) -> Result<MascalValue, MascalError> {
        match target {
            MascalValue::Integer(i) => Ok(MascalValue::Integer(i.neg()?)),
            MascalValue::Float(f) => { Ok(MascalValue::Float(-f)) },

            _ => {Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                line: 0,
                character: 0,
                source: String::from("Cannot perform the negation operation on a non-numeric type")
            })}
        }
    }
}
