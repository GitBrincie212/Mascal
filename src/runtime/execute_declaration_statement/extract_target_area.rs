use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::VariableData;
use std::cell::RefCell;
use std::rc::Rc;

pub fn extract_target_area(
    varname: &String,
    vardata: &VariableData,
    layers: &[(MascalValue, bool)],
) -> Result<Rc<RefCell<Option<MascalValue>>>, MascalError> {
    let mut target_value: Rc<RefCell<Option<MascalValue>>> = Rc::new(RefCell::new(Some(
        vardata.value.clone().unwrap().borrow().clone(),
    )));

    for (idx, (curr_index, is_dynamic)) in layers.iter().enumerate() {
        let expected_array_size = vardata.array_dimensions[idx];
        let expected_dynamic = vardata.is_dynamic_array[idx];

        if *is_dynamic != expected_dynamic {
            return Err(MascalError {
                error_type: MascalErrorType::TypeError,
                line: 0,
                character: 0,
                source: format!(
                    "Expected to index via a {} array, but indexed via a {} array instead",
                    if expected_dynamic {
                        "dynamic"
                    } else {
                        "static"
                    },
                    if *is_dynamic { "dynamic" } else { "static" },
                ),
            });
        }

        let index_value: i128 = match curr_index {
            MascalValue::Integer(i) => i.to_i128(),
            _ => {
                return Err(MascalError {
                    error_type: MascalErrorType::TypeError,
                    line: 0,
                    character: 0,
                    source: format!(
                        "Expected integer index, got {}",
                        curr_index.as_type_string()?
                    ),
                });
            }
        };

        let normalized_index: i128 = if index_value < 0 {
            expected_array_size as i128 + index_value
        } else {
            index_value
        };

        if normalized_index < 0 || normalized_index >= expected_array_size as i128 {
            return Err(MascalError {
                error_type: MascalErrorType::IndexError,
                line: 0,
                character: 0,
                source: format!("Index out of bounds for array {}", varname),
            });
        }

        let next_value_rc: Rc<RefCell<Option<MascalValue>>> = {
            let target_borrow = target_value.borrow().clone().unwrap();

            match target_borrow {
                MascalValue::DynamicArray(values) => match values.get(normalized_index as usize) {
                    Some(inner) => inner.clone(),
                    None => {
                        return Err(MascalError {
                            error_type: MascalErrorType::IndexError,
                            line: 0,
                            character: 0,
                            source: format!(
                                "Index {} out of bounds in a dynamic array {}",
                                normalized_index, varname
                            ),
                        });
                    }
                },
                MascalValue::StaticArray(values) => match values.get(normalized_index as usize) {
                    Some(inner) => inner.clone(),
                    None => {
                        return Err(MascalError {
                            error_type: MascalErrorType::IndexError,
                            line: 0,
                            character: 0,
                            source: format!(
                                "Index {} out of bounds in a static array {}",
                                normalized_index, varname
                            ),
                        });
                    }
                },
                _ => {
                    return Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        line: 0,
                        character: 0,
                        source: format!(
                            "Trying to index a non-array value (type: {}) in variable {}",
                            target_borrow.as_type_string()?,
                            varname
                        ),
                    });
                }
            }
        };

        target_value = next_value_rc;
    }

    Ok(target_value)
}
