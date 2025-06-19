use std::cell::RefCell;
use std::rc::Rc;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::VariableData;

pub fn check_array_assignment(
    target_value: Rc<RefCell<MascalValue>>,
    assignment_value: Rc<MascalValue>,
    vardata: &VariableData,
    depth_index: usize,
) -> Result<(), MascalError> {
    if depth_index >= vardata.array_dimensions.len() {
        return match &*assignment_value {
            MascalValue::StaticArray(_) | MascalValue::DynamicArray(_) => {
                Err(MascalError {
                    error_type: MascalErrorType::TypeError,
                    line: 0,
                    character: 0,
                    source: String::from("The current array type is deeper than initialized to be")
                })
            }
            _ => Ok(()),
        };
    }
    let (arr_assign_values, is_dynamic) = match &*assignment_value {
        MascalValue::StaticArray(values) => ((*values).clone(), false),
        MascalValue::DynamicArray(values) => ((*values).clone(), true),
        _ => { return Ok(()) }
    };
    let expected_array_size: usize = vardata.array_dimensions[depth_index];
    let expected_dynamic: bool = vardata.is_dynamic_array[depth_index];
    if arr_assign_values.len() != expected_array_size {
        return Err(MascalError {
            error_type: MascalErrorType::TypeError,
            line: 0,
            character: 0,
            source: format!(
                "Mismatch between element size, expected an array of {} element(s) but got an array of {} element(s)",
                expected_array_size,
                arr_assign_values.len()
            )
        })
    }
    if expected_dynamic != is_dynamic {
        return Err(MascalError {
            error_type: MascalErrorType::TypeError,
            line: 0,
            character: 0,
            source: format!(
                "Expected a {} array, but got a {} array instead",
                if expected_dynamic {"dynamic"} else {"static"},
                if is_dynamic {"dynamic"} else {"static"},
            )
        })
    }
    for val in arr_assign_values.iter()  {
        check_array_assignment(target_value.clone(), Rc::from(val.clone()), vardata, depth_index + 1)?;
    }
    Ok(())
}