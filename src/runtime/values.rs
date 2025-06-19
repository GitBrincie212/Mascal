pub mod value_arithmetic_operations;
pub mod value_comparision_operations;
mod value_utils;
pub mod value_boolean_operations;

use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
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
    DynamicArray(Arc<Vec<MascalValue>>),
    StaticArray(Arc<Vec<MascalValue>>),
    Type(MascalType)
}

impl MascalValue {
    fn is_expected_array_internal(&self, sizes: Rc<[usize]>, dynamics: Rc<[bool]>, curr: usize) -> Result<(), MascalError> {
        match self {
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
                for val in values.iter() {
                    val.is_expected_array_internal(sizes.clone(), dynamics.clone(), curr + 1)?;
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
                    val.is_expected_array_internal(sizes.clone(), dynamics.clone(), curr + 1)?;
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
    
    pub fn is_expected_array(&self, sizes: Rc<[usize]>, dynamics: Rc<[bool]>) -> Result<(), MascalError> {
        match self {
            MascalValue::StaticArray(_) | MascalValue::DynamicArray(_) => {
                self.is_expected_array_internal(sizes, dynamics, 0)
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
            MascalValue::DynamicArray(values) |  MascalValue::StaticArray(values) => {
                let mut mascal_type: MascalType = MascalType::Dynamic;
                let mut has_run_once: bool = false;
                for value in values.iter() {
                    let val_type: MascalType = value.as_mascal_type()?;
                    if has_run_once && mascal_type != val_type {
                        return Ok(MascalType::Dynamic);
                    }
                    mascal_type = val_type;
                    has_run_once = true;
                }
                Ok(mascal_type)
            }

            MascalValue::NULL => Err(MascalError {
                error_type: MascalErrorType::TypeError,
                line: 0,
                character: 0,
                source: String::from("NULL is not a type in of itself")
            }),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            MascalValue::String(s) => s.deref().to_string(),
            MascalValue::Integer(i) => {i.as_string()}
            MascalValue::Float(f) => {f.to_string()}
            MascalValue::Boolean(b) => {if *b {String::from("True")} else {String::from("False")}}
            MascalValue::NULL => {String::from("NULL")}
            MascalValue::DynamicArray(values) => {
                String::from("<") + &*values.iter().map(|v| v.as_string())
                    .collect::<Vec<String>>()
                    .join(", ") + ">"
            }
            MascalValue::StaticArray(values) => {
                String::from("[") + &*values.iter().map(|v| v.as_string())
                    .collect::<Vec<String>>()
                    .join(", ") + "]"
            }
            MascalValue::Type(t) => {
                t.as_string()
            }
        }
    }

    pub fn as_type_string(&self) -> String {
        match self {
            MascalValue::String(_) => String::from("STRING"),
            MascalValue::Integer(_) => String::from("INTEGER"),
            MascalValue::Float(_) => String::from("FLOAT"),
            MascalValue::Boolean(_) => String::from("BOOLEAN"),
            MascalValue::NULL => String::from("NULL"),
            MascalValue::DynamicArray(values) => {
                if let Some(first) = values.first() {
                    return first.as_string() + format!("<{}>", values.len()).as_str();
                }
                String::from("ANY")
            }
            MascalValue::StaticArray(values) => {
                if let Some(first) = values.first() {
                    return first.as_string() + format!("[{}]", values.len()).as_str();
                }
                String::from("ANY")
            }
            MascalValue::Type(t) => {
                t.as_string()
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

    pub fn is_atomic_type_of(&self, value_type: &MascalType) -> bool {
        match (self, value_type) {
            (MascalValue::Integer {..}, MascalType::Integer {..}) => true,
            (MascalValue::Float {..}, MascalType::Float) => true,
            (MascalValue::String(..), MascalType::String) => true,
            (MascalValue::Boolean(..), MascalType::Boolean) => true,
            (MascalValue::NULL, _) => true,
            (_, MascalType::Dynamic) => true,
            (MascalValue::Type(..), MascalType::Type) => true,
            (MascalValue::StaticArray(values), _) => {
                values.iter().all(|val| val.is_atomic_type_of(value_type))
            }
            (MascalValue::DynamicArray(values), _) => {
                values.iter().all(|val| val.is_atomic_type_of(value_type))
            }
            _ => false
        }
    }
}