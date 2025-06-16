mod variable_table;
pub mod execute_expression;
mod execute_binary_expression;
mod execute_inner_member_expression;
mod execute_unary_expression;
mod execute_function_expression;
pub mod values;
mod value_operations;
mod execute_statement;

use std::borrow::Cow;
use std::rc::Rc;
use crate::ast::AbstractSyntaxTree;
use crate::defs::blocks::{ExecutionBlock, ScopedBlocks};
use crate::defs::errors::MascalError;
use crate::defs::InfinityControl;
use crate::runtime::execute_statement::execute_statement;
use crate::runtime::variable_table::{create_variable_table, VariableTable};

pub struct ExecutionData<'a> {
    pub variable_table: Option<&'a VariableTable>,
    pub infinity_control: InfinityControl,
    pub scoped_blocks: Rc<Vec<ScopedBlocks>>
}

pub fn interpert(mut abstract_syntax_tree: AbstractSyntaxTree) -> Result<(), MascalError> {
    let mut scoped_blocks: Vec<ScopedBlocks> = abstract_syntax_tree.blocks;
    let program_block: ScopedBlocks = scoped_blocks.remove(abstract_syntax_tree.program_index);
    let exec_block: ExecutionBlock = match program_block {
        ScopedBlocks::PROGRAM(exec_block) => exec_block,
        ScopedBlocks::FUNCTION {..} => {unreachable!()},
    };
    let mut scoped_variable_table: VariableTable = create_variable_table(&exec_block)?;
    let containerized_scoped_blocks: Rc<Vec<ScopedBlocks>> =  Rc::new(scoped_blocks);
    for statement in &exec_block.body {
        scoped_variable_table = execute_statement(
            Cow::Borrowed(statement), scoped_variable_table, containerized_scoped_blocks.clone()
        )?;
    }
    Ok(())
}