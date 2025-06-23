use std::cell::RefCell;
use rustc_hash::FxHashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::defs::blocks::ExecutionBlock;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::types::{MascalType};
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;
use crate::runtime::utils::make_array;


/*
 I am highly aware that this could be severally optimized by using a stack and a heap. However,
 this lookup table offers more simplicity compared to making a full on stack/heap; the goal is to
 just make a toy language, investing in deep system level design will be way harder
*/
#[allow(dead_code)]
pub type VariableTable = FxHashMap<String, VariableData>;

#[derive(Debug, Clone)]
pub struct VariableData {
    pub value: Option<Rc<RefCell<MascalValue>>>,
    pub is_constant: bool,
    pub is_nullable: bool,
    pub array_dimensions: Rc<[usize]>,
    pub is_dynamic_array: Rc<[bool]>,
    pub atomic_variable_type: Arc<MascalType>,
}

macro_rules! create_variable_table_for_type {
    ($variable_type: expr, $table: expr, $target_type: expr) => {
        for var in $variable_type {
            let has_defined_value = var.initial_value.is_some();
            let mut value: Option<Rc<RefCell<MascalValue>>> = if let Some(unwrapped_val) = var.initial_value {
                let val: MascalValue = execute_expression(unwrapped_val, Rc::new(RefCell::new(ExecutionData {
                    variable_table: Some($table.clone()),
                    scoped_blocks: Rc::new(RefCell::new(Vec::new()))
                })))?;
                if !(val.is_atomic_type_of(&*$target_type.clone())?) {
                    return Err(MascalError {
                        error_type: MascalErrorType::RuntimeError,
                        line: 0,
                        character: 0,
                        source: format!("Evaluated value does not match its atomic type which is {:?}", $target_type.clone())
                    });
                }
                Some(Rc::new(RefCell::new(val)))
            } else {None};
            let mut dimensions_val: Vec<usize> = Vec::new();
            for dimension in var.dimensions {
                let val: MascalValue = execute_expression(dimension, Rc::new(RefCell::new(ExecutionData {
                    variable_table: Some($table.clone()),
                    scoped_blocks: Rc::new(RefCell::new(Vec::new()))
                })))?;

                let size = match val {
                    MascalValue::Integer(i) => {
                        if i.is_negative_or_zero() {
                            return Err(MascalError {
                                error_type: MascalErrorType::TypeError,
                                line: 0,
                                character: 0,
                                source: String::from("Evaluated expression is not a positive non-zero integer to be used in specifying an array size")
                            });
                        }
                        i.to_i128() as usize
                    }
                    _ => {return Err(MascalError {
                        error_type: MascalErrorType::TypeError,
                        line: 0,
                        character: 0,
                        source: String::from("Evaluated expression is not of type integer to be used in specifying an array size")
                    })}
                };

                dimensions_val.push(size);
            }

            value = if !dimensions_val.is_empty() && !has_defined_value  {
                let dyns: Vec<bool> = var.is_dynamic_array.to_vec();
                let arr = make_array(&dimensions_val, &dyns);
                Some(Rc::new(RefCell::new(arr)))
            } else {value};

            let is_dynamic_array: Rc<[bool]> = var.is_dynamic_array.into();
            let array_dimensions: Rc<[usize]> = Rc::from(dimensions_val);
            /*
            if let Some(unwrapped_value) = value.clone() {
                unwrapped_value.borrow().is_expected_array(
                    array_dimensions.clone(),
                    is_dynamic_array.clone()
                )?;
            }
             */

            $table.borrow_mut().insert(var.name, VariableData {
                value,
                is_dynamic_array,
                array_dimensions,
                is_constant: var.is_constant,
                is_nullable: var.is_nullable,
                atomic_variable_type: $target_type.clone(),
            });
        }
    };
}

#[allow(dead_code)]
pub fn create_variable_table(mut block: ExecutionBlock) -> Result<(Rc<RefCell<VariableTable>>, ExecutionBlock), MascalError> {
    let table: Rc<RefCell<VariableTable>> = Rc::new(RefCell::new(FxHashMap::default()));

    let integers = std::mem::take(&mut block.variables.integers);
    let floats = std::mem::take(&mut block.variables.floats);
    let strings = std::mem::take(&mut block.variables.strings);
    let booleans = std::mem::take(&mut block.variables.booleans);
    let dynamics = std::mem::take(&mut block.variables.dynamics);
    let types = std::mem::take(&mut block.variables.types);

    create_variable_table_for_type!(integers, Rc::clone(&table), Arc::new(MascalType::Integer));
    create_variable_table_for_type!(floats, Rc::clone(&table), Arc::new(MascalType::Float));
    create_variable_table_for_type!(strings, Rc::clone(&table), Arc::new(MascalType::String));
    create_variable_table_for_type!(booleans, Rc::clone(&table), Arc::new(MascalType::Boolean));
    create_variable_table_for_type!(dynamics, Rc::clone(&table), Arc::new(MascalType::Dynamic));
    create_variable_table_for_type!(types, Rc::clone(&table), Arc::new(MascalType::Type));

    Ok((table, block))
}