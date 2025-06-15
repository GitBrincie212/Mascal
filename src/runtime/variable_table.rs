use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::defs::blocks::ExecutionBlock;
use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::types::MascalUnprocessedType;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::values::MascalValue;

#[allow(dead_code)]
pub type VariableTable = HashMap<String, VariableData>;

pub struct VariableData {
    pub value: Option<Rc<MascalValue>>,
    pub is_constant: bool,
    pub is_nullable: bool,
    pub atomic_variable_type: Arc<MascalUnprocessedType>
}

#[allow(dead_code)]
fn create_variable_table_for_type(
    variable_type: Vec<MascalVariableInitialDeclaration>,
    mut table: VariableTable,
    target_type: Arc<MascalUnprocessedType>,
) -> Result<VariableTable, MascalError> {
    for var in variable_type {
        let value: Option<Rc<MascalValue>> = if let Some(unwrapped_val) = var.initial_value {
            let val: MascalValue = execute_expression(unwrapped_val)?;
            if val.is_atomic_type_of(&target_type) {
                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    line: 0,
                    character: 0,
                    source: String::from("Interpreted value does not match its type")
                });
            }
            Some(Rc::new(val))
        } else {None};
        table.insert(var.name, VariableData {
            value,
            is_constant: var.is_constant,
            is_nullable: var.is_nullable,
            atomic_variable_type: Arc::clone(&target_type)
        });
    }
    
    Ok(table)
}

#[allow(dead_code)]
pub fn create_variable_table(block: ExecutionBlock) -> Result<VariableTable, MascalError> {
    let mut table: VariableTable = HashMap::new();
    table = create_variable_table_for_type(block.variables.integers, table, Arc::new(MascalUnprocessedType::Integer))?;
    table = create_variable_table_for_type(block.variables.floats, table, Arc::new(MascalUnprocessedType::Float))?;
    table = create_variable_table_for_type(block.variables.strings, table, Arc::new(MascalUnprocessedType::String))?;
    table = create_variable_table_for_type(block.variables.booleans, table, Arc::new(MascalUnprocessedType::Boolean))?;
    table = create_variable_table_for_type(block.variables.dynamics, table, Arc::new(MascalUnprocessedType::Dynamic))?;
    table = create_variable_table_for_type(block.variables.types, table, Arc::new(MascalUnprocessedType::Type))?;
    Ok(table)
}