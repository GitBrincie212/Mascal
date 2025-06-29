use crate::defs::expressions::MascalExpression;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MascalVariableInitialDeclaration {
    pub name: Rc<str>,
    pub is_constant: bool,
    pub is_nullable: bool,
    pub dimensions: Box<[MascalExpression]>,
    pub initial_value: Option<MascalExpression>,
    pub is_dynamic_array: Box<[bool]>,
}
