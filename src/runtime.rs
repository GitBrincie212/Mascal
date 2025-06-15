use crate::ast::AbstractSyntaxTree;
use crate::defs::blocks::{ExecutionBlock, ScopedBlocks};
use crate::defs::errors::MascalError;

pub fn interpert(abstract_syntax_tree: AbstractSyntaxTree) -> Result<(), MascalError> {
    let program_block: &ScopedBlocks = &abstract_syntax_tree.blocks[abstract_syntax_tree.program_index];
    let exec_block: &ExecutionBlock = match program_block {
        ScopedBlocks::PROGRAM(exec_block) => exec_block,
        ScopedBlocks::FUNCTION {..} => {return Ok(())}, // Bored to point out as error
    };
    Ok(())
}