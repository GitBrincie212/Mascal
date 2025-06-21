use std::cell::RefCell;
use std::rc::Rc;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::MascalError;
use crate::runtime::values::MascalValue;

pub fn make_array(
    dims: &[usize],
    dyns: &[bool],
) -> MascalValue {
    if dims.len() == 1 {
        let len = dims[0];
        let slots = (0..len)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>();

        if dyns[0] {
            return MascalValue::DynamicArray(slots);
        }
        return MascalValue::StaticArray(slots.into_boxed_slice());
    }
    let len = dims[0];
    let tail_dims = &dims[1..];
    let tail_dyns = &dyns[1..];

    let slots = (0..len).map(|_| {
        Rc::new(RefCell::new(Some(make_array(tail_dims, tail_dyns))))
    })
        .collect::<Vec<_>>();

    if dyns[0] {
        return MascalValue::DynamicArray(slots);
    }
    MascalValue::StaticArray(slots.into_boxed_slice())
}

pub fn get_dimensions(val: &Option<MascalValue>, dimension: usize) -> Result<Option<MascalValue>, MascalError> {
    if let Some(unwrapped_val) = val {
        match unwrapped_val {
            MascalValue::StaticArray(val) => {
                return get_dimensions(&*val.first().unwrap().borrow(), dimension + 1);
            }
            MascalValue::DynamicArray(val) => {
                return get_dimensions(&*val.first().unwrap().borrow(), dimension + 1);
            }
            _ => {}
        };
    }
    Ok(Some(MascalValue::Integer(IntegerNum::new(dimension as i128))))
}

pub fn get_sizes(val: &Option<MascalValue>, mut sizes: Vec<usize>) -> Result<Option<MascalValue>, MascalError> {
    if let Some(unwrapped_val) = val {
        match unwrapped_val {
            MascalValue::StaticArray(val) => {
                sizes.push(val.len());
                return get_sizes(&*val.first().unwrap().borrow(), sizes);
            }
            MascalValue::DynamicArray(val) => {
                sizes.push(val.len());
                return get_sizes(&*val.first().unwrap().borrow(), sizes);
            }
            _ => {}
        };
    }
    let converted_size = sizes.iter()
            .map(|x| Rc::new(RefCell::new(Some(MascalValue::Integer(IntegerNum::new(*x as i128))))))
            .collect::<Vec<_>>()
            .into_boxed_slice();
    
    Ok(Some(MascalValue::StaticArray(converted_size)))
}

#[macro_export]
macro_rules! uninit_cell_error {
    () => {
        return Err(MascalError {
            error_type: MascalErrorType::ValueError,
            line: 0,
            character: 0,
            source: String::from("Uninitialized cell in an array has been detected")
        })
    };
}

#[macro_export]
macro_rules! as_mascal_atomic_type_array_impl {
    ($values: expr) => {
        let mut mascal_type: MascalType = MascalType::Dynamic;
        let mut has_run_once: bool = false;
        for value in $values.iter() {
            if let Some(unwrapped_value) = &*value.borrow() {
                let val_type: MascalType = unwrapped_value.as_mascal_type()?;
                if has_run_once && mascal_type != val_type {
                    return Ok(MascalType::Dynamic);
                }
                mascal_type = val_type;
                has_run_once = true;
                continue;
            }
            uninit_cell_error!();
        }
        return Ok(mascal_type)
    };
}

#[macro_export]
macro_rules! as_mascal_type_array_impl {
    ($values: expr, $is_dynamic: expr) => {
        let mut mascal_type: MascalType = MascalType::Dynamic;
        let mut has_run_once: bool = false;
        for value in $values.iter() {
            if let Some(unwrapped_value) = &*value.borrow() {
                let val_type: MascalType = unwrapped_value.as_atomic_mascal_type()?;
                if has_run_once && mascal_type != val_type {
                    mascal_type = MascalType::Dynamic;
                    break;
                }
                mascal_type = val_type;
                has_run_once = true;
                continue;
            }
            uninit_cell_error!();
        }
        if $is_dynamic {
            return Ok(MascalType::DynamicArray(Box::new(mascal_type)))
        }
        return Ok(MascalType::StaticArray(Box::new(mascal_type)))
    };
}

#[macro_export]
macro_rules! as_string_array_impl {
    ($values: expr, $open: expr, $close: expr) => {
        let mut target_string = String::from($open);
        if $values.is_empty() {
            return Ok(target_string + $close);
        }
        for v in $values[..$values.len() - 1].iter() {
            if let Some(unwrapped_value) = &*v.borrow() {
                target_string += format!("{}, ", unwrapped_value.as_string_inner(true)?).as_str();
                continue;
            }
            uninit_cell_error!();
        }
        if let Some(last_val) = $values.last() {
            let extracted_final_value = (&*last_val.borrow());
            if let Some(unwrapped_value) = extracted_final_value {
                target_string += format!("{}", unwrapped_value.as_string_inner(true)?).as_str();
                return Ok(target_string + $close);
            }
            uninit_cell_error!();
        }
        return Ok(target_string + $close);
    };
}

#[macro_export]
macro_rules! as_type_string_array_impl {
    ($values: expr) => {
        if let Some(first) = $values.first() {
            if let Some(extracted_first) = &*first.clone().borrow() {
                let string = extracted_first.as_string()?;
                return Ok(string + format!("<{}>", $values.len()).as_str());
            }
            uninit_cell_error!();
        }
        return Ok(String::from("ANY"));
    };
}

#[macro_export]
macro_rules! atomic_type_array_impl {
    ($values: expr, $value_type: expr) => {
        for val in $values {
            if let Some(unwrapped_value) = &*val.borrow() {
                if !unwrapped_value.is_atomic_type_of($value_type)? {
                    return Ok(false);
                }
                continue;
            }
            uninit_cell_error!();
        }
        return Ok(true);
    };
}

#[macro_export]
macro_rules! index_array_impl {
    ($values: expr, $is_dynamic: expr, $num_val: expr) => {
        if $is_dynamic {
            return Err(MascalError {
                error_type: MascalErrorType::IndexError,
                line: 0,
                character: 0,
                source: format!(
                    "Attempting to access a {} array when it is a {} one",
                    if $is_dynamic {"dynamic"} else {"static"},
                    if $is_dynamic {"static"} else {"dynamic"}
                )
            })
        }

        if $num_val < 0 {
            $num_val = ($values.len() as i128) + $num_val;
        }
        if $num_val < 0 || $num_val >= $values.len() as i128 {
            return Err(MascalError {
                error_type: MascalErrorType::IndexError,
                line: 0,
                character: 0,
                source: format!("Index is out of bounds for array size of {}", $values.len())
            })
        }
        if let Some(extracted_value) = &*$values[$num_val as usize].clone().borrow() {
            return Ok(extracted_value.clone());
        }
        uninit_cell_error!()
    }
}

#[macro_export]
macro_rules! define_array_expression_exec {
    ($array: expr, $variable_table: expr) => {{
        let mut arr: Vec<Rc<RefCell<Option<MascalValue>>>> = Vec::with_capacity($array.len());
        for expr in $array {
            let evaluated_val = Some(execute_expression(expr, $variable_table)?);
            arr.push(Rc::new(RefCell::new(evaluated_val)));
        }
        arr
    }};
}

#[macro_export]
macro_rules! array_check_assignment_impl {
    ($target_val: expr, $values: expr, $vardata: expr, $depth_index: expr, $is_dynamic: expr) => {
        let expected_array_size: usize = $vardata.array_dimensions[$depth_index];
        let expected_dynamic: bool = $vardata.is_dynamic_array[$depth_index];
        if $values.len() != expected_array_size {
            return Err(MascalError {
                error_type: MascalErrorType::TypeError,
                line: 0,
                character: 0,
                source: format!(
                    "Mismatch between element size, expected an array of {} element(s) but got an array of {} element(s)",
                    expected_array_size,
                    $values.len()
                )
            })
        }
        if expected_dynamic != $is_dynamic {
            return Err(MascalError {
                error_type: MascalErrorType::TypeError,
                line: 0,
                character: 0,
                source: format!(
                    "Expected a {} array, but got a {} array instead",
                    if expected_dynamic {"dynamic"} else {"static"},
                    if $is_dynamic {"dynamic"} else {"static"},
                )
            })
        }
        for val in $values.iter()  {
            check_array_assignment($target_val.clone(), val.clone(), $vardata, $depth_index + 1)?;
        }
        return Ok(());
    };
}

#[macro_export]
macro_rules! type_cast_array_impl {
    ($values: expr, $array_type: expr, $value_type_init: expr) => {{
        for val in $values.clone() {
            let mut val_mut_borrow = val.borrow_mut();
            if let Some(unwrapped_val) = &*val_mut_borrow {
                let deref_array_type = *$array_type.clone();
                let is_dynamic_type = &deref_array_type == &MascalType::Dynamic;
                if !is_dynamic_type && unwrapped_val.is_type_of(&deref_array_type) {
                    return Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        character: 0,
                        line: 0,
                        source: format!(
                            "Expected an array with type {:?} but got {:?}",
                            $array_type.as_string(), unwrapped_val.as_string()?
                        )
                    })
                }
                if is_dynamic_type {
                    continue;
                }
                *val_mut_borrow = Some(
                    execute_processed_typecast(deref_array_type, unwrapped_val.clone())?
                );
                continue;
            }
        }
        return Ok($value_type_init($values));
    }};
}