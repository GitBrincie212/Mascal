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

#[macro_export]
macro_rules! min_max_common_operation {
    ($args: expr, $operator: ident) => {
        match ($args.first().unwrap(), $args.last().unwrap()) {
            (MascalValue::Float(f), MascalValue::Integer(i)) => {
                return if i.as_f64().$operator(f) {Ok(Some(MascalValue::Float(*f)))} else {
                    Ok(Some(MascalValue::Integer(i.clone())))
                }
            }
            (MascalValue::Integer(i), MascalValue::Float(f)) => {
                return if i.as_f64().$operator(f) {Ok(Some(MascalValue::Float(*f)))} else {
                    Ok(Some(MascalValue::Integer(i.clone())))
                }
            }
            (MascalValue::Float(f1), MascalValue::Float(f2)) => {
                return if (*f1).$operator(f2) {Ok(Some(MascalValue::Float(*f1)))} else {
                    Ok(Some(MascalValue::Float(*f2)))
                }
            }
            (MascalValue::Integer(i1), MascalValue::Integer(i2)) => {
                return if i1.to_i128().$operator(&i2.to_i128()) {Ok(Some(MascalValue::Integer(i1.clone())))} else {
                    Ok(Some(MascalValue::Integer(i2.clone())))
                }
            }
            (_, _) => unreachable!()
        }
    };
}