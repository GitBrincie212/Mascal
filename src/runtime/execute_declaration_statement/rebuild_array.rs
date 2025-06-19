use std::sync::Arc;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;

pub fn rebuild_array(
    value: MascalValue,
    layers: &[(usize, bool)],
    new_elem: MascalValue,
) -> Result<MascalValue, MascalError> {
    match layers.split_first() {
        None => Ok(new_elem),
        Some((&(idx, is_dyn), rest)) => {
            match value {
                MascalValue::StaticArray(arc_vec) if !is_dyn => {
                    let mut vec = (*arc_vec).clone();
                    if idx >= vec.len() {
                        return Err(MascalError {
                            error_type: MascalErrorType::IndexError,
                            line: 0,
                            character: 0,
                            source: format!("Index(i.e {}) is out of bounds in static array", idx),
                        });
                    }
                    vec[idx] = rebuild_array(vec[idx].clone(), rest, new_elem)?;
                    Ok(MascalValue::StaticArray(Arc::new(vec)))
                }
                MascalValue::DynamicArray(arc_vec) if is_dyn => {
                    let mut vec = (*arc_vec).clone();
                    if idx >= vec.len() {
                        return Err(MascalError {
                            error_type: MascalErrorType::IndexError,
                            line: 0,
                            character: 0,
                            source: format!("Index(i.e {}) is out of bounds in dynamic array", idx),
                        });
                    }
                    vec[idx] = rebuild_array(vec[idx].clone(), rest, new_elem)?;
                    Ok(MascalValue::DynamicArray(Arc::new(vec)))
                }
                other => {
                    Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        line: 0,
                        character: 0,
                        source: format!("Expected an array type to index to, but an atomic type {}", other.as_type_string()),
                    })
                }
            }
        }
    }
}