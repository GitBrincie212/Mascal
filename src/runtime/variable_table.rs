use std::borrow::Cow;
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
    pub value: Option<MascalValue>,
    pub is_constant: bool,
    pub is_nullable: bool,
    pub atomic_variable_type: Arc<MascalType>,
}

impl VariableData {
    pub fn clone_with_new_value(&self, new_value: Option<MascalValue>) -> VariableData {
        VariableData {
            value: new_value,
            is_constant: self.is_constant,
            is_nullable: self.is_nullable,
            atomic_variable_type: self.atomic_variable_type.clone(),
        }
    }
}

#[allow(dead_code)]
fn create_variable_table_for_type(
    variable_type: Cow<Vec<MascalVariableInitialDeclaration>>,
    mut table: VariableTable,
    target_type: Arc<MascalType>,
) -> Result<VariableTable, MascalError> {
    for var in variable_type.into_owned() {
        let value: Option<MascalValue> = if let Some(unwrapped_val) = var.initial_value {
            let val: Cow<MascalValue> = execute_expression(unwrapped_val, &ExecutionData {
                variable_table: Some(&table),
                scoped_blocks: Rc::new(Vec::new())
            })?;
            if !val.is_atomic_type_of(&target_type) {
                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    line: 0,
                    character: 0,
                    source: format!("Interpreted value does not match its atomic type which is {:?}", &target_type)
                });
            }
            Some(val.into_owned())
        } else {None};
        table.insert(var.name, VariableData {
            value,
            is_constant: var.is_constant,
            is_nullable: var.is_nullable,
            atomic_variable_type: Arc::clone(&target_type),
        });
    }
    
    Ok(table)
}

#[allow(dead_code)]
pub fn create_variable_table(block: &ExecutionBlock) -> Result<VariableTable, MascalError> {
    let mut table: VariableTable = HashMap::new();

    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.integers), table, Arc::new(MascalType::Integer))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.floats), table, Arc::new(MascalType::Float))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.strings), table, Arc::new(MascalType::String))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.booleans), table, Arc::new(MascalType::Boolean))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.dynamics), table, Arc::new(MascalType::Dynamic))?;
    table = create_variable_table_for_type(Cow::Borrowed(&block.variables.types), table, Arc::new(MascalType::Type))?;
    Ok(table)
}