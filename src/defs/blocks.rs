use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::statements::{MascalStatement};
use crate::defs::types::MascalUnprocessedType;

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
        return_type: Option<MascalUnprocessedType>,
        execution_block: ExecutionBlock,
    }
}

impl VariableBlock {
    pub fn new(
        integers: Vec<MascalVariableInitialDeclaration>,
               floats: Vec<MascalVariableInitialDeclaration>,
               booleans: Vec<MascalVariableInitialDeclaration>,
               strings: Vec<MascalVariableInitialDeclaration>,
               dynamics: Vec<MascalVariableInitialDeclaration>,
               types: Vec<MascalVariableInitialDeclaration>
    ) -> Self {
        VariableBlock {
            integers,
            floats,
            booleans,
            strings,
            dynamics,
            types
        }
    }

    pub fn iter_all(&self) -> Vec<&MascalVariableInitialDeclaration> {
        self.integers.iter()
            .chain(self.floats.iter())
            .chain(self.strings.iter())
            .chain(self.booleans.iter())
            .chain(self.dynamics.iter())
            .chain(self.types.iter())
            .collect()
    }
}