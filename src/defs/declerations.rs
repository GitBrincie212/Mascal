use crate::defs::expressions::MascalExpression;

#[derive(Debug, Clone)]
pub struct MascalVariableInitialDeclaration {
    pub name: String,
    pub is_constant: bool,
    pub is_nullable: bool,
    pub dimensions: Vec<MascalExpression>,
    pub initial_value: Option<MascalExpression>,
    pub is_dynamic_array: Vec<bool>,
}