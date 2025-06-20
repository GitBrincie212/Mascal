use std::cell::RefCell;
use std::rc::Rc;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;
use crate::uninit_cell_error;

pub fn flatten_impl(
    target: Rc<RefCell<Option<MascalValue>>>,
) -> Result<Vec<Rc<RefCell<Option<MascalValue>>>>, MascalError> {
    let maybe_val = target.borrow();

    match &*maybe_val {
        Some(MascalValue::DynamicArray(children)) => {
            let mut result = Vec::new();
            for child in children {
                result.extend(flatten_impl(child.clone())?);
            }
            Ok(result)
        }
        Some(MascalValue::StaticArray(children)) => {
            let mut result = Vec::new();
            for child in children.iter() {
                result.extend(flatten_impl(child.clone())?);
            }
            Ok(result)
        }
        Some(_) => {
            Ok(vec![target.clone()])
        }
        None => {
            uninit_cell_error!();
        }
    }
}