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

    pub fn get_variable_type(&self, var_type: MascalUnprocessedType) -> Option<&Vec<MascalVariableInitialDeclaration>> {
        match var_type {
            MascalUnprocessedType::Integer => Some(&self.integers),
            MascalUnprocessedType::Float => Some(&self.floats),
            MascalUnprocessedType::Boolean => Some(&self.booleans),
            MascalUnprocessedType::String => Some(&self.strings),
            MascalUnprocessedType::Dynamic => Some(&self.dynamics),
            MascalUnprocessedType::Type => Some(&self.types),
            _ => None
        }
    }

    pub fn get_variable_type_by_index(&self, type_index: usize) -> Option<&Vec<MascalVariableInitialDeclaration>> {
        self.get_variable_type(self.type_index_to_mascal_type(type_index)?)
    }

    pub fn type_index_to_mascal_type(&self, type_index: usize) -> Option<MascalUnprocessedType> {
        match type_index {
            0 => Some(MascalUnprocessedType::Integer),
            1 => Some(MascalUnprocessedType::Float),
            2 => Some(MascalUnprocessedType::Boolean),
            3 => Some(MascalUnprocessedType::String),
            4 => Some(MascalUnprocessedType::Dynamic),
            5 => Some(MascalUnprocessedType::Type),
            _ => None
        }
    }
}