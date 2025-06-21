pub mod value_arithmetic_operations;
pub mod value_comparision_operations;
mod value_utils;
pub mod value_boolean_operations;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use crate::{
    as_mascal_type_array_impl,
    as_string_array_impl,
    as_type_string_array_impl,
    atomic_type_array_impl,
    uninit_cell_error
};
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::types::{MascalType};

#[derive(Clone, Debug)]
pub enum MascalValue {
    Integer(IntegerNum),
    Float(f64),
    String(Arc<str>),
    Boolean(bool),
    NULL,
    StaticArray(Box<[Rc<RefCell<Option<MascalValue>>>]>),
    DynamicArray(Vec<Rc<RefCell<Option<MascalValue>>>>),
    Type(MascalType)
}

fn is_expected_array_internal(
    outer_value: Rc<RefCell<Option<MascalValue>>>, sizes: Rc<[usize]>, dynamics: Rc<[bool]>, curr: usize
) -> Result<(), MascalError> {
    if let Some(unwrapped_outer_value) = &*outer_value.borrow() {
        return match unwrapped_outer_value {
            MascalValue::StaticArray(values) => {
                if curr < sizes.len()  {
                    if dynamics[curr] {
                        return Err(MascalError {
                            error_type: MascalErrorType::TypeError,
                            line: 0,
                            character: 0,
                            source: format!(
                                "Expected a dynamic array with size {} element(s) but got a static array with size {} element(s)",
                                sizes[curr],
                                values.len()
                            ),
                        })
                    } else if sizes[curr] != values.len() {
                        return Err(MascalError {
                            error_type: MascalErrorType::TypeError,
                            line: 0,
                            character: 0,
                            source: format!(
                                "Expected a static array with size {} element(s) but got a static array with size {} element(s)",
                                sizes[curr],
                                values.len()
                            ),
                        })
                    }
                } else {
                    return Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        line: 0,
                        character: 0,
                        source: String::from("The current array type is deeper than initialized to be")
                    })
                }
                for val in values.clone().into_iter() {
                    is_expected_array_internal(val.clone(), sizes.clone(), dynamics.clone(), curr + 1)?;
                }
                Ok(())
            }
            MascalValue::DynamicArray(values) => {
                if curr < sizes.len()  {
                    if !dynamics[curr] {
                        return Err(MascalError {
                            error_type: MascalErrorType::TypeError,
                            line: 0,
                            character: 0,
                            source: format!(
                                "Expected a static array with size {} element(s) but got a dynamic array with size {} element(s)",
                                sizes[curr],
                                values.len()
                            ),
                        })
                    }
                } else {
                    return Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        line: 0,
                        character: 0,
                        source: String::from("The current array type is deeper than initialized to be")
                    })
                }
                for val in values.iter() {
                    is_expected_array_internal(val.clone(), sizes.clone(), dynamics.clone(), curr + 1)?;
                }
                Ok(())
            }
            _ => {
                if curr < sizes.len() {
                    return Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        line: 0,
                        character: 0,
                        source: format!("Expected a {} but got an atomic type instead", {
                            let next_idx: usize = usize::min(curr + 1, sizes.len() - 1);
                            let current_size: usize = sizes[next_idx];
                            let is_dynamic_current: bool = dynamics[next_idx];
                            if is_dynamic_current {
                                format!("dynamic array with {} dimension(s)", current_size)
                            } else {format!("static array with {} dimension(s)", current_size)}
                        })
                    })
                }
                Ok(())
            }
        }
    }
    uninit_cell_error!();
}

impl MascalValue {
    pub fn is_equal(&self, other: &MascalValue) -> bool {
        match (self, other) {
            (MascalValue::Integer(i1), MascalValue::Integer(i2)) =>
                i1.to_i128() == i2.to_i128(),
            (MascalValue::Float(f1), MascalValue::Float(f2)) => *f1 == *f2,
            (MascalValue::Boolean(b1), MascalValue::Boolean(b2)) => *b1 == *b2,
            (MascalValue::String(s1), MascalValue::String(s2)) => s1.eq(&*s2),
            (MascalValue::Type(t1), MascalValue::Type(t2)) => *t1 == *t2,
            (MascalValue::NULL, MascalValue::NULL) => true,
            (MascalValue::DynamicArray(values1), MascalValue::DynamicArray(values2)) => {
                if values1.len() != values2.len() {return false;}
                values1
                    .iter()
                    .zip(values2.iter())
                    .all(|(v1, v2)| {
                        let v1_borrow: &Option<MascalValue> = &*v1.borrow();
                        let v2_borrow: &Option<MascalValue> = &*v2.borrow();
                        if let (Some(v1_unwrapped), Some(v2_unwrapped)) = (v1_borrow, v2_borrow) {
                            return v1_unwrapped.is_equal(v2_unwrapped);
                        }
                        if v1_borrow.is_none() && v2_borrow.is_none() {return true}
                        false
                    })
            },
            (_, _) => false
        }
    }

    pub fn is_expected_array(&self, sizes: Rc<[usize]>, dynamics: Rc<[bool]>) -> Result<(), MascalError> {
        match self {
            MascalValue::StaticArray(v) => {
                for values in v.iter() {
                    is_expected_array_internal(values.clone(), sizes.clone(), dynamics.clone(), 1)?;
                }
                Ok(())
            }

            MascalValue::DynamicArray(v) => {
                for values in v.iter() {
                    is_expected_array_internal(values.clone(), sizes.clone(), dynamics.clone(), 1)?;
                }
                Ok(())
            }

            _ => {
                if !sizes.is_empty() {
                    return Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        line: 0,
                        character: 0,
                        source: format!("Expected a {} but got an atomic type instead", {
                            let current_size: usize = *sizes.first().unwrap();
                            let is_dynamic_current: bool = *dynamics.first().unwrap();
                            if is_dynamic_current {
                                format!("dynamic array with {} dimension(s)", current_size)
                            } else {format!("static array with {} dimension(s)", current_size)}
                        })
                    });
                }
                Ok(())
            }
        }
    }

    pub fn as_mascal_type(&self) -> Result<MascalType, MascalError> {
        match self {
            MascalValue::String(_) => Ok(MascalType::String),
            MascalValue::Integer(_) => Ok(MascalType::Integer),
            MascalValue::Float(_) => Ok(MascalType::Float),
            MascalValue::Boolean(_) => Ok(MascalType::Boolean),
            MascalValue::Type(_) => Ok(MascalType::Type),
            MascalValue::StaticArray(values) => {
                as_mascal_type_array_impl!(values);
            }
            MascalValue::DynamicArray(values) => {
                as_mascal_type_array_impl!(values);
            }

            MascalValue::NULL => Err(MascalError {
                error_type: MascalErrorType::TypeError,
                line: 0,
                character: 0,
                source: String::from("NULL is not a type in of itself")
            }),
        }
    }

    pub fn as_string(&self) -> Result<String, MascalError> {
        self.as_string_inner(false)
    }

    pub fn as_string_inner(&self, quote_string: bool) -> Result<String, MascalError> {
        match self {
            MascalValue::String(s) => Ok(if quote_string {format!("{:?}", s.deref().to_string())} else {s.deref().to_string()}),
            MascalValue::Integer(i) => {Ok(i.as_string())}
            MascalValue::Float(f) => {Ok(if f.floor() == *f {format!("{}.0", f)} else {f.to_string()})}
            MascalValue::Boolean(b) => {if *b {Ok(String::from("TRUE"))} else {Ok(String::from("FALSE"))}}
            MascalValue::NULL => {Ok(String::from("NULL"))}
            MascalValue::DynamicArray(values) => {
                as_string_array_impl!(values, "<", ">");
            }
            MascalValue::StaticArray(values) => {
                as_string_array_impl!(values, "[", "]");
            }
            MascalValue::Type(t) => {
                Ok(t.as_string())
            }
        }
    }

    pub fn as_type_string(&self) -> Result<String, MascalError> {
        match self {
            MascalValue::String(_) => Ok(String::from("STRING")),
            MascalValue::Integer(_) => Ok(String::from("INTEGER")),
            MascalValue::Float(_) => Ok(String::from("FLOAT")),
            MascalValue::Boolean(_) => Ok(String::from("BOOLEAN")),
            MascalValue::NULL => Ok(String::from("NULL")),
            MascalValue::DynamicArray(values) => {
                as_type_string_array_impl!(values);
            }
            MascalValue::StaticArray(values) => {
                as_type_string_array_impl!(values);
            }
            MascalValue::Type(t) => {
                Ok(t.as_string())
            }
        }
    }

    pub fn extract_as_float(&self) -> Option<f64> {
        match self {
            MascalValue::Integer(i) => Some(i.as_f64()),
            MascalValue::Float(f) => Some(*f),
            _ => {None}
        }
    }

    pub fn extract_as_int(&self) -> Option<i128> {
        match self {
            MascalValue::Integer(i) => Some(i.to_i128()),
            MascalValue::Float(f) => Some(f.round() as i128),
            _ => {None}
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            MascalValue::DynamicArray(_) => true,
            MascalValue::StaticArray(_) => true,
            _ => false
        }
    }
    
    pub fn is_type_of(&self, value_type: &MascalType) -> bool {
        match (self, value_type) {
            (MascalValue::Integer {..}, MascalType::Integer) => true,
            (MascalValue::Float{..}, MascalType::Float) => true,
            (MascalValue::String(..), MascalType::String) => true,
            (MascalValue::Boolean(..), MascalType::Boolean) => true,
            (MascalValue::NULL, _) => true,
            (MascalValue::Type(..), MascalType::Type) => true,
            (MascalValue::StaticArray(..), MascalType::StaticArray {array_type, ..}) => {
                self.is_type_of(array_type)
            }
            (MascalValue::DynamicArray(..), MascalType::DynamicArray {array_type, ..}) => {
                self.is_type_of(array_type)
            }
            _ => false
        }
    }

    pub fn is_atomic_type_of(&self, value_type: &MascalType) -> Result<bool, MascalError> {
        match (self, value_type) {
            (MascalValue::Integer {..}, MascalType::Integer {..}) => Ok(true),
            (MascalValue::Float {..}, MascalType::Float) => Ok(true),
            (MascalValue::String(..), MascalType::String) => Ok(true),
            (MascalValue::Boolean(..), MascalType::Boolean) => Ok(true),
            (MascalValue::NULL, _) => Ok(true),
            (_, MascalType::Dynamic) => Ok(true),
            (MascalValue::Type(..), MascalType::Type) => Ok(true),
            (MascalValue::StaticArray(values), _) => {
                atomic_type_array_impl!(values, &value_type);
            }
            (MascalValue::DynamicArray(values), _) => {
                atomic_type_array_impl!(values, &value_type);
            }
            _ => Ok(false)
        }
    }
}