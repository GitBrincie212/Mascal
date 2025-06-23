use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::statements::{MascalStatement};
use crate::defs::types::MascalUnprocessedType;

#[derive(Debug, Clone)]
pub struct VariableBlock {
    pub integers: Box<[MascalVariableInitialDeclaration]>,
    pub floats: Box<[MascalVariableInitialDeclaration]>,
    pub strings: Box<[MascalVariableInitialDeclaration]>,
    pub booleans: Box<[MascalVariableInitialDeclaration]>,
    pub dynamics: Box<[MascalVariableInitialDeclaration]>,
    pub types: Box<[MascalVariableInitialDeclaration]>,
}

#[derive(Debug, Clone)]
pub struct ExecutionBlock {
    pub variables: VariableBlock,
    pub body: Box<[MascalStatement]>
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
        parameters: Box<[MascalParameter]>,
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
            integers: integers.into_boxed_slice(),
            floats: floats.into_boxed_slice(),
            booleans: booleans.into_boxed_slice(),
            strings: strings.into_boxed_slice(),
            dynamics: dynamics.into_boxed_slice(),
            types: types.into_boxed_slice()
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