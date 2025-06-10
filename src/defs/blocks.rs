use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::statements::MascalStatement;
use crate::defs::types::MascalType;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableBlock {
    integers: Vec<MascalVariableInitialDeclaration>,
    floats: Vec<MascalVariableInitialDeclaration>,
    strings: Vec<MascalVariableInitialDeclaration>,
    booleans: Vec<MascalVariableInitialDeclaration>,
    dynamics: Vec<MascalVariableInitialDeclaration>,
    types: Vec<MascalVariableInitialDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionBlock {
    variables: VariableBlock,
    body: Vec<MascalStatement>
}

#[derive(Debug, Clone, PartialEq)]
pub struct MascalParameter {
    name: String,
    mascal_type: MascalType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBlock {
    parameters: Vec<MascalParameter>,
    name: Option<String>,
    return_type: Option<MascalType>,
    execution_block: ExecutionBlock,
}