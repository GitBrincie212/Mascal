use crate::defs::blocks::ScopedBlocks;
use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::runtime::ExecutionData;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::VariableTable;
use std::cell::RefCell;
use std::rc::Rc;

#[inline(always)]
pub fn unwrap_index_layers(
    variable: MascalExpression,
    variable_table: Rc<RefCell<VariableTable>>,
    scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>>,
) -> Result<(MascalExpression, Vec<(MascalValue, bool)>), MascalError> {
    let mut layers: Vec<(MascalValue, bool)> = Vec::new();
    let mut base: MascalExpression = variable;
    while let MascalExpression::Indexing {
        array,
        index,
        is_dynamic,
    } = base
    {
        let index_val = execute_expression(
            *index,
            &mut ExecutionData {
                variable_table: Some(variable_table.clone()),
                scoped_blocks: scoped_blocks.clone(),
            },
        )?;
        layers.push((index_val, is_dynamic));
        base = *array;
    }
    layers.reverse();
    Ok((base, layers))
}
