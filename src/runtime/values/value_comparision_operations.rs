use crate::{comparison_arms, uninit_cell_error};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;

macro_rules! array_equality_impl {
    ($arr1: expr, $arr2: expr, $truthful_value: expr) => {
        if $arr1.len() != $arr2.len() {return Ok(MascalValue::Boolean(!$truthful_value));}
        for (i, j) in $arr1.iter().zip($arr2.iter()) {
            if let (Some(i_unwrapped), Some(j_unwrapped)) = (&*i.borrow(), &*j.borrow()) {
                match MascalValue::equals(i_unwrapped, j_unwrapped)? {
                    MascalValue::Boolean(false) => return Ok(MascalValue::Boolean(!$truthful_value)),
                    _ => {}
                }
                continue;
            }
            uninit_cell_error!();
        }
        return Ok(MascalValue::Boolean($truthful_value));
    };
}

impl MascalValue {
    pub fn equals(
        left: &MascalValue,
        right: &MascalValue
    ) -> Result<MascalValue, MascalError> {
        comparison_arms!(left, right, eq, |v1, v2| {
            match (v1, v2) {
                (MascalValue::StaticArray(arr1), MascalValue::StaticArray(arr2)) => {
                    array_equality_impl!(arr1, arr2, true);
                },
                
                (MascalValue::DynamicArray(arr1), MascalValue::DynamicArray(arr2)) => {
                    array_equality_impl!(arr1, arr2, true);
                },
                
                (MascalValue::NULL, MascalValue::NULL) => Ok(MascalValue::Boolean(true)),

                (MascalValue::Type(t1), MascalValue::Type(t2)) => Ok(
                    MascalValue::Boolean(t1.eq(&t2))
                ),

                (MascalValue::Boolean(b1), MascalValue::Boolean(b2)) => Ok(
                    MascalValue::Boolean(b1.eq(&b2))
                ),

                _ => Ok(MascalValue::Boolean(false))
            }
        })
    }

    pub fn not_equals (
        left: &MascalValue,
        right: &MascalValue
    ) -> Result<MascalValue, MascalError> {
        comparison_arms!(left, right, ne, |v1, v2| {
            match (v1, v2) {
                (MascalValue::StaticArray(arr1), MascalValue::StaticArray(arr2)) => {
                    array_equality_impl!(arr1, arr2, false);
                },
                
                (MascalValue::DynamicArray(arr1), MascalValue::DynamicArray(arr2)) => {
                    array_equality_impl!(arr1, arr2, false);
                },
                
                (MascalValue::NULL, MascalValue::NULL) => Ok(MascalValue::Boolean(false)),

                (MascalValue::Type(t1), MascalValue::Type(t2)) => Ok(
                    MascalValue::Boolean(t1.ne(&t2))
                ),

                (MascalValue::Boolean(b1), MascalValue::Boolean(b2)) => Ok(
                    MascalValue::Boolean(b1.ne(&b2))
                ),

                _ => Ok(MascalValue::Boolean(true))
            }
        })
    }

    pub fn greater_than (
        left: &MascalValue,
        right: &MascalValue
    ) -> Result<MascalValue, MascalError> {
        comparison_arms!(left, right, gt, |v1: MascalValue, v2: MascalValue| {
            return Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                character: 0,
                line: 0,
                source: format!(
                    "Cannot operate the \"greater than(>)\" operation between the types {:?} and {:?}",
                    v1.as_type_string(),
                    v2.as_type_string()
                )
            })
        })
    }

    pub fn less_than (
        left: &MascalValue,
        right: &MascalValue
    ) -> Result<MascalValue, MascalError> {
        comparison_arms!(left, right, lt, |v1: MascalValue, v2: MascalValue| {
            return Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                character: 0,
                line: 0,
                source: format!(
                    "Cannot operate the \"less than(<)\" operation between the types {:?} and {:?}",
                    v1.as_type_string(),
                    v2.as_type_string()
                )
            })
        })
    }

    pub fn less_than_or_equal(
        left: &MascalValue,
        right: &MascalValue
    ) -> Result<MascalValue, MascalError> {
        comparison_arms!(left, right, le, |v1: MascalValue, v2: MascalValue| {
            return Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                character: 0,
                line: 0,
                source: format!(
                    "Cannot operate the \"less than or equal(<=)\" operation between the types {:?} and {:?}",
                    v1.as_type_string(),
                    v2.as_type_string()
                )
            })
        })
    }

    pub fn greater_than_or_equal(
        left: &MascalValue,
        right: &MascalValue
    ) -> Result<MascalValue, MascalError> {
        comparison_arms!(left, right, ge, |v1: MascalValue, v2: MascalValue| {
            return Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                character: 0,
                line: 0,
                source: format!(
                    "Cannot operate the \"less than or equal(>=)\" operation between the types {:?} and {:?}",
                    v1.as_type_string(),
                    v2.as_type_string()
                )
            })
        })
    }
}
