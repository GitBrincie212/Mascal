use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use crate::defs::blocks::ExecutionBlock;
use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::InfinityControl;
use crate::defs::types::{MascalType};
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

#[allow(dead_code)]
pub type VariableTable = HashMap<String, VariableData>;

pub struct VariableData {
    pub value: Option<MascalValue>,
    pub is_constant: bool,
    pub is_nullable: bool,
    pub atomic_variable_type: Arc<MascalType>
}

#[allow(dead_code)]
fn create_variable_table_for_type(
    variable_type: Vec<MascalVariableInitialDeclaration>,
    mut table: VariableTable,
    target_type: Arc<MascalType>,
) -> Result<VariableTable, MascalError> {
    for var in variable_type {
        let value: Option<MascalValue> = if let Some(unwrapped_val) = var.initial_value {
            let val: Cow<MascalValue> = execute_expression(unwrapped_val, &ExecutionData {
                variable_table: Some(&table),
                infinity_control: InfinityControl::AllowInfinity
            })?;
            if val.is_atomic_type_of(&target_type) {
                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    line: 0,
                    character: 0,
                    source: String::from("Interpreted value does not match its type")
                });
            }
            Some(val.into_owned())
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
    table = create_variable_table_for_type(block.variables.integers, table, Arc::new(MascalType::Integer))?;
    table = create_variable_table_for_type(block.variables.floats, table, Arc::new(MascalType::Float))?;
    table = create_variable_table_for_type(block.variables.strings, table, Arc::new(MascalType::String))?;
    table = create_variable_table_for_type(block.variables.booleans, table, Arc::new(MascalType::Boolean))?;
    table = create_variable_table_for_type(block.variables.dynamics, table, Arc::new(MascalType::Dynamic))?;
    table = create_variable_table_for_type(block.variables.types, table, Arc::new(MascalType::Type))?;
    Ok(table)
}