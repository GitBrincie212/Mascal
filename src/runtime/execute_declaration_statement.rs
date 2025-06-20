mod execute_index_based_decleration;
mod unwrap_index_layers;
mod extract_variable_data;
mod extract_target_area;
mod check_array_assignment;

use std::cell::{RefCell};
use std::rc::Rc;
use std::sync::Arc;
use crate::defs::blocks::ScopedBlocks;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::runtime::execute_declaration_statement::check_array_assignment::check_array_assignment;
use crate::runtime::execute_declaration_statement::execute_index_based_decleration::execute_index_based_decleration;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{VariableData, VariableTable};

pub fn execute_declaration_statement(
    variable: MascalExpression, value: MascalExpression,
    variable_table: Rc<RefCell<VariableTable>>, scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>>
) -> Result<(), MascalError> {
    match variable {
        MascalExpression::SymbolicExpression(varname) => {
            let variable_table_borrow = variable_table.borrow();
            if let Some(vardata) = variable_table_borrow.get(&varname) {
                let is_constant = vardata.is_constant;
                let is_nullable = vardata.is_nullable;
                let array_dimensions = vardata.array_dimensions.clone();
                let is_dynamic_array = vardata.is_dynamic_array.clone();
                let atomic_variable_type = Arc::clone(&vardata.atomic_variable_type);
                if is_constant {
                    return Err(MascalError {
                        line: 0,
                        character: 0,
                        error_type: MascalErrorType::RuntimeError,
                        source: format!("Cannot assign a new value to the constant variable called {:?}", varname)
                    })
                }

                drop(variable_table_borrow);
                let value: MascalValue = execute_expression(value, Rc::new(RefCell::new(ExecutionData {
                    variable_table: Some(variable_table.clone()),
                    scoped_blocks,
                })))?;

                value.is_expected_array(array_dimensions.clone(), is_dynamic_array.clone())?;

                let mut vartable_mutable_borrow = variable_table.borrow_mut();
                let owned_data = VariableData {
                    value: Some(Rc::new(RefCell::new(value))),
                    is_constant,
                    is_nullable,
                    array_dimensions,
                    is_dynamic_array,
                    atomic_variable_type,
                };

                vartable_mutable_borrow.insert(varname, owned_data);
                return Ok(());
            }

            Err(MascalError {
                line: 0,
                character: 0,
                error_type: MascalErrorType::RuntimeError,
                source: format!("Expected a variable name, however got an unknown one called {:?}", varname)
            })
        }

        MascalExpression::IndexExpression {..} => {
            execute_index_based_decleration(variable, value, variable_table, scoped_blocks)
        }

        _ => {unreachable!()}
    }
}