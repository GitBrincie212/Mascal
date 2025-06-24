use crate::defs::blocks::ScopedBlocks;

#[derive(Debug, Clone)]
pub struct AbstractSyntaxTree {
    pub blocks: Vec<ScopedBlocks>,
    pub program_index: usize,
}
