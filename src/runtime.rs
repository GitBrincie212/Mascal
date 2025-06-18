mod variable_table;
pub mod execute_expression;
mod execute_binary_expression;
mod execute_inner_member_expression;
mod execute_unary_expression;
mod execute_function_expression;
pub mod values;
mod execute_statement;
mod execute_builtin_function;

use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::AbstractSyntaxTree;
use crate::defs::blocks::{ExecutionBlock, ScopedBlocks};
use crate::defs::errors::MascalError;
use crate::runtime::execute_statement::execute_statement;
use crate::runtime::variable_table::{create_variable_table, VariableTable};

pub struct ExecutionData {
    pub variable_table: Option<Rc<RefCell<VariableTable>>>,
    pub scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>>
}

pub fn interpert(abstract_syntax_tree: AbstractSyntaxTree) -> Result<(), MascalError> {
    let mut scoped_blocks: Vec<ScopedBlocks> = abstract_syntax_tree.blocks;
    let program_block: ScopedBlocks = scoped_blocks.remove(abstract_syntax_tree.program_index);
    let mut exec_block: ExecutionBlock = match program_block {
        ScopedBlocks::PROGRAM(exec_block) => exec_block,
        ScopedBlocks::FUNCTION {..} => {unreachable!()},
    };
    let scoped_variable_table: Rc<RefCell<VariableTable>>;
    (scoped_variable_table, exec_block) = create_variable_table(exec_block)?;
    let containerized_scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>> =  Rc::new(RefCell::new(scoped_blocks));
    for statement in exec_block.body.into_iter() {
        execute_statement(
            statement, scoped_variable_table.clone(), containerized_scoped_blocks.clone()
        )?;
    }
    Ok(())
}