mod variable_table;
pub mod execute_expression;
mod execute_binary_expression;
mod execute_inner_member_expression;
mod execute_unary_expression;
mod execute_function_expression;
pub mod values;
mod value_operations;
mod execute_statement;

use crate::ast::AbstractSyntaxTree;
use crate::defs::blocks::{ExecutionBlock, ScopedBlocks};
use crate::defs::errors::MascalError;
use crate::defs::InfinityControl;
use crate::runtime::variable_table::VariableTable;

pub struct ExecutionData<'a> {
    pub variable_table: Option<&'a VariableTable>,
    pub infinity_control: InfinityControl,
}

pub fn interpert(abstract_syntax_tree: AbstractSyntaxTree) -> Result<(), MascalError> {
    let program_block: &ScopedBlocks = &abstract_syntax_tree.blocks[abstract_syntax_tree.program_index];
    let exec_block: &ExecutionBlock = match program_block {
        ScopedBlocks::PROGRAM(exec_block) => exec_block,
        ScopedBlocks::FUNCTION {..} => {return Ok(())}, // Bored to point out as error
    };
    Ok(())
}