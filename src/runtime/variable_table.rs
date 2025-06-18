use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::defs::blocks::ExecutionBlock;
use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::types::{MascalType};
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;


/*
 I am highly aware that this could be severally optimized by using a stack and a heap. However,
 this lookup table offers more simplicity compared to making a full on stack/heap; the goal is to
 just make a toy language, investing in deep system level design will be way harder
*/
#[allow(dead_code)]
pub type VariableTable = HashMap<String, VariableData>;

#[derive(Debug, Clone)]
pub struct VariableData {
    pub value: Option<Rc<RefCell<MascalValue>>>,
    pub is_constant: bool,
    pub is_nullable: bool,
    pub array_dimensions: Arc<Vec<usize>>,
    pub is_dynamic_array: Arc<Vec<bool>>,
    pub atomic_variable_type: Arc<MascalType>,
}

#[allow(dead_code)]
fn create_variable_table_for_type(
    variable_type: Cow<Vec<MascalVariableInitialDeclaration>>,
    table: Rc<RefCell<VariableTable>>,
    target_type: Arc<MascalType>,
) -> Result<Rc<RefCell<VariableTable>>, MascalError> {
    for var in variable_type.into_owned() {
        let value: Option<Rc<RefCell<MascalValue>>> = if let Some(unwrapped_val) = var.initial_value {
            let val: MascalValue = execute_expression(unwrapped_val, Rc::new(RefCell::new(ExecutionData {
                variable_table: Some(table.clone()),
                scoped_blocks: Rc::new(RefCell::new(Vec::new()))
            })))?;
            if !val.is_atomic_type_of(&target_type) {
                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    line: 0,
                    character: 0,
                    source: format!("Evaluated value does not match its atomic type which is {:?}", &target_type)
                });
            }
            Some(Rc::new(RefCell::new(val)))
        } else {None};
        let mut dimensions_val: Vec<usize> = Vec::new();
        for dimension in var.dimensions {
            let val: MascalValue = execute_expression(dimension, Rc::new(RefCell::new(ExecutionData {
                variable_table: Some(table.clone()),
                scoped_blocks: Rc::new(RefCell::new(Vec::new()))
            })))?;
            
            dimensions_val.push(match val {
                MascalValue::Integer(i) => {
                    if i.is_negative_or_zero() {
                        return Err(MascalError {
                            error_type: MascalErrorType::TypeError,
                            line: 0,
                            character: 0,
                            source: format!("Evaluated expression is not of type integer to be used {:?}", &target_type)
                        });
                    }
                    i.to_i128() as usize
                }
                _ => {return Err(MascalError {
                    error_type: MascalErrorType::TypeError,
                    line: 0,
                    character: 0,
                    source: format!("Evaluated expression is not of type integer to be used {:?}", &target_type)
                })}
            });
        }
        
        table.borrow_mut().insert(var.name, VariableData {
            value,
            is_dynamic_array: Arc::new(var.is_dynamic_array),
            array_dimensions: Arc::new(dimensions_val),
            is_constant: var.is_constant,
            is_nullable: var.is_nullable,
            atomic_variable_type: Arc::clone(&target_type),
        });
    }
    
    Ok(table)
}

#[allow(dead_code)]
pub fn create_variable_table(block: &ExecutionBlock) -> Result<Rc<RefCell<VariableTable>>, MascalError> {
    let mut table: Rc<RefCell<VariableTable>> = Rc::new(RefCell::new(HashMap::new()));

    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.integers), table, Arc::new(MascalType::Integer))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.floats), table, Arc::new(MascalType::Float))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.strings), table, Arc::new(MascalType::String))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.booleans), table, Arc::new(MascalType::Boolean))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.dynamics), table, Arc::new(MascalType::Dynamic))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.types), table, Arc::new(MascalType::Type))?;
    Ok(table)
}