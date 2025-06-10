use crate::defs::expressions::MascalExpression;

#[derive(Debug, PartialEq, Clone)]
pub struct MascalVariableInitialDeclaration {
    name: String,
    is_constant: bool,
    is_nullable: bool,
    initial_value: MascalExpression,
}