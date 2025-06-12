use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::statements::MascalStatement;
use crate::defs::types::MascalType;

#[derive(Debug, Clone)]
pub struct VariableBlock {
    pub integers: Vec<MascalVariableInitialDeclaration>,
    pub floats: Vec<MascalVariableInitialDeclaration>,
    pub strings: Vec<MascalVariableInitialDeclaration>,
    pub booleans: Vec<MascalVariableInitialDeclaration>,
    pub dynamics: Vec<MascalVariableInitialDeclaration>,
    pub types: Vec<MascalVariableInitialDeclaration>,
}

#[derive(Debug, Clone)]
pub struct ExecutionBlock {
    pub variables: VariableBlock,
    pub body: Vec<MascalStatement>
}

#[derive(Debug, Clone, PartialEq)]
pub struct MascalParameter {
    pub name: String,
    pub is_mutable: bool,
}

#[derive(Debug, Clone)]
pub enum ScopedBlocks {
    PROGRAM(ExecutionBlock),
    FUNCTION {
        parameters: Vec<MascalParameter>,
        name: String,
        return_type: Option<MascalType>,
        execution_block: ExecutionBlock,
    }
}