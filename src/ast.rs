use crate::defs::blocks::ScopedBlocks;

#[derive(Debug, Clone)]
pub struct AbstractSyntaxTree {
    pub blocks: Vec<ScopedBlocks>
}