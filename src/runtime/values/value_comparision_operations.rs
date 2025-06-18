use crate::{comparison_arms};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;

impl MascalValue {
    fn are_arrays_equal(arr1: &Vec<MascalValue>, arr2: &Vec<MascalValue>) -> Result<MascalValue, MascalError> {
        if arr1.len() != arr2.len() {return Ok(MascalValue::Boolean(false));}
        for (i, j) in arr1.iter().zip(arr2.iter()) {
            match MascalValue::equals(i, j)? {
                MascalValue::Boolean(false) => return Ok(MascalValue::Boolean(false)),
                _ => {}
            }
        }
        Ok(MascalValue::Boolean(true))
    }
    
    fn are_arrays_not_equal(arr1: &Vec<MascalValue>, arr2: &Vec<MascalValue>) -> Result<MascalValue, MascalError> {
        Self::are_arrays_equal(arr1, arr2).map(|x| match x {
            MascalValue::Boolean(true) => MascalValue::Boolean(false),
            MascalValue::Boolean(false) => MascalValue::Boolean(true),
            _ => {unreachable!()}
        })
    }

    fn is_first_array_gt(_arr1: &Vec<MascalValue>, _arr2: &Vec<MascalValue>) -> Result<MascalValue, MascalError> {
        Err(MascalError {
            error_type: MascalErrorType::UndefinedOperation,
            line: 0,
            character: 0,
            source: String::from("Cannot use the \"greater than(>)\" operator in array types")
        })
    }

    fn is_first_array_lt(_arr1: &Vec<MascalValue>, _arr2: &Vec<MascalValue>) -> Result<MascalValue, MascalError> {
        Err(MascalError {
            error_type: MascalErrorType::UndefinedOperation,
            line: 0,
            character: 0,
            source: String::from("Cannot use the \"less than(<)\" operator in array types")
        })
    }

    fn is_first_array_le(_arr1: &Vec<MascalValue>, _arr2: &Vec<MascalValue>) -> Result<MascalValue, MascalError> {
        Err(MascalError {
            error_type: MascalErrorType::UndefinedOperation,
            line: 0,
            character: 0,
            source: String::from("Cannot use the \"less than or equal(<=)\" operator in array types")
        })
    }

    fn is_first_array_ge(_arr1: &Vec<MascalValue>, _arr2: &Vec<MascalValue>) -> Result<MascalValue, MascalError> {
        Err(MascalError {
            error_type: MascalErrorType::UndefinedOperation,
            line: 0,
            character: 0,
            source: String::from("Cannot use the \"greater than or equal(>=)\" operator in array types")
        })
    }

    pub fn equals(
        left: &MascalValue,
        right: &MascalValue
    ) -> Result<MascalValue, MascalError> {
        comparison_arms!(left, right, Self::are_arrays_equal, eq, |v1, v2| {
            match (v1, v2) {
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
        comparison_arms!(left, right, Self::are_arrays_not_equal, ne, |v1, v2| {
            match (v1, v2) {
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
        comparison_arms!(left, right, Self::is_first_array_gt, gt, |v1: MascalValue, v2: MascalValue| {
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
        comparison_arms!(left, right, Self::is_first_array_lt, lt, |v1: MascalValue, v2: MascalValue| {
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
        comparison_arms!(left, right, Self::is_first_array_le, le, |v1: MascalValue, v2: MascalValue| {
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
        comparison_arms!(left, right, Self::is_first_array_ge, ge, |v1: MascalValue, v2: MascalValue| {
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
