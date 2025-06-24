use crate::array_check_assignment_impl;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::VariableData;
use std::cell::RefCell;
use std::rc::Rc;

pub fn check_array_assignment(
    target_value: Rc<RefCell<Option<MascalValue>>>,
    assignment_value: Rc<RefCell<Option<MascalValue>>>,
    vardata: &VariableData,
    depth_index: usize,
) -> Result<(), MascalError> {
    if let Some(unwrapped_assignment_value) = &*assignment_value.borrow() {
        if depth_index >= vardata.array_dimensions.len() {
            return match unwrapped_assignment_value {
                MascalValue::StaticArray(_) | MascalValue::DynamicArray(_) => Err(MascalError {
                    error_type: MascalErrorType::TypeError,
                    line: 0,
                    character: 0,
                    source: String::from("The current array type is deeper than initialized to be"),
                }),
                _ => Ok(()),
            };
        }
        match unwrapped_assignment_value {
            MascalValue::StaticArray(values) => {
                array_check_assignment_impl!(target_value, values, vardata, depth_index, false);
            }

            MascalValue::DynamicArray(values) => {
                array_check_assignment_impl!(target_value, values, vardata, depth_index, true);
            }
            _ => {
                return Ok(());
            }
        };
    }
    unreachable!()
}
