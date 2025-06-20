use std::cell::RefCell;
use std::rc::Rc;
use crate::defs::blocks::ScopedBlocks;
use crate::defs::errors::{MascalError};
use crate::defs::expressions::MascalExpression;
use crate::runtime::execute_declaration_statement::check_array_assignment::check_array_assignment;
use crate::runtime::execute_declaration_statement::extract_target_area::extract_target_area;
use crate::runtime::execute_declaration_statement::extract_variable_data::extract_variable_data;
use crate::runtime::execute_declaration_statement::unwrap_index_layers::unwrap_index_layers;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{VariableTable};

pub fn execute_index_based_decleration(
    variable: MascalExpression, value: MascalExpression, 
    variable_table: Rc<RefCell<VariableTable>>, scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>>
) -> Result<(), MascalError> {
    let (base, layers) = unwrap_index_layers(
        variable, variable_table.clone(), scoped_blocks.clone()
    )?;
    let (varname, vardata) = extract_variable_data(base, variable_table.clone(), &layers)?;
    
    let rhs: MascalValue = execute_expression(
        value,
        Rc::new(RefCell::new(ExecutionData {
            variable_table: Some(variable_table.clone()),
            scoped_blocks: scoped_blocks.clone(),
        })),
    )?;
    
    let layers_len: usize = layers.len();
    let target_value: Rc<RefCell<Option<MascalValue>>> = extract_target_area(&varname, &vardata, &layers)?; 
    check_array_assignment(target_value.clone(), Rc::new(RefCell::new(Some(rhs.clone()))), &vardata, layers_len)?;
    *target_value.borrow_mut() = Some(rhs);
    variable_table.borrow_mut().insert(varname, vardata);
    Ok(())
}