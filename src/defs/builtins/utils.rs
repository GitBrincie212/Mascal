use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;
use crate::uninit_cell_error;
use std::cell::RefCell;
use std::rc::Rc;

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
        Some(_) => Ok(vec![target.clone()]),
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
                return if i.as_f64().$operator(f) {
                    Ok(Some(MascalValue::Float(*f)))
                } else {
                    Ok(Some(MascalValue::Integer(i.clone())))
                }
            }
            (MascalValue::Integer(i), MascalValue::Float(f)) => {
                return if i.as_f64().$operator(f) {
                    Ok(Some(MascalValue::Float(*f)))
                } else {
                    Ok(Some(MascalValue::Integer(i.clone())))
                }
            }
            (MascalValue::Float(f1), MascalValue::Float(f2)) => {
                return if (*f1).$operator(f2) {
                    Ok(Some(MascalValue::Float(*f1)))
                } else {
                    Ok(Some(MascalValue::Float(*f2)))
                }
            }
            (MascalValue::Integer(i1), MascalValue::Integer(i2)) => {
                return if i1.to_i128().$operator(&i2.to_i128()) {
                    Ok(Some(MascalValue::Integer(i1.clone())))
                } else {
                    Ok(Some(MascalValue::Integer(i2.clone())))
                }
            }
            (_, _) => unreachable!(),
        }
    };
}

#[macro_export]
macro_rules! check_boundaries {
    ($val1: expr, $val2: expr) => {
        if $val1 > $val2 {
            return Err(MascalError {
                error_type: MascalErrorType::ValueError,
                character: 0,
                line: 0,
                source: String::from("Unallowed range between maximum value and minimum value"),
            });
        }
    };
}

#[macro_export]
macro_rules! from_string_to_array_impl {
    ($val: expr) => {{
        let mut array_vec: Vec<Rc<RefCell<Option<MascalValue>>>> = Vec::with_capacity($val.len());
        for char in $val.chars() {
            array_vec.push(Rc::new(RefCell::new(Some(MascalValue::String(Arc::from(
                char.to_string(),
            ))))));
        }
        array_vec
    }};
}

pub fn sum_internal(
    value: Rc<RefCell<Option<MascalValue>>>,
    sum: &mut f64,
    encountered_float: bool,
) -> Result<bool, MascalError> {
    match &*value.borrow() {
        Some(MascalValue::Integer(i)) => {
            *sum += i.as_f64();
            Ok(encountered_float)
        }

        Some(MascalValue::Float(f)) => {
            *sum += *f;
            Ok(true)
        }

        Some(MascalValue::StaticArray(v)) => {
            let mut main_result: bool = false;
            for value in v {
                main_result = main_result || sum_internal(value.clone(), sum, encountered_float)?;
            }
            Ok(main_result)
        }

        Some(MascalValue::DynamicArray(v)) => {
            let mut main_result: bool = false;
            for value in v {
                main_result = main_result || sum_internal(value.clone(), sum, encountered_float)?;
            }
            Ok(main_result)
        }

        Some(val) => Err(MascalError {
            error_type: MascalErrorType::ArgumentError,
            line: 0,
            character: 0,
            source: format!(
                "Expected a numeric value (i.e float or integer) but got {:?}",
                val.as_string()?
            ),
        }),

        None => {
            uninit_cell_error!()
        }
    }
}

#[macro_export]
macro_rules! join_array_impl {
    ($v: expr, $sep: expr) => {
        let mut result: String = String::from("");
        for (index, value) in $v.into_iter().enumerate() {
            let val_borrow = &*value.borrow();
            match val_borrow {
                Some(val) => {
                    result += val.as_string()?.as_str();
                    if index != $v.len() - 1 {
                        result += $sep;
                    }
                    continue;
                }

                None => {
                    uninit_cell_error!()
                }
            }
        }
        return Ok(Some(MascalValue::String(Arc::from(result))));
    };
}
