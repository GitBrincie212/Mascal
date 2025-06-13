use crate::defs::expressions::MascalExpression;
use crate::defs::token::TokenType;

#[derive(Debug, Clone)]
pub enum MascalType {
    Integer,
    Float,
    Boolean,
    String,
    Dynamic,
    Type,
    DynamicArray {
        array_type: Box<MascalType>,
        initial_size: MascalExpression,
    },
    StaticArray {
        array_type: Box<MascalType>,
        size: MascalExpression,
    },
}

pub fn token_type_to_atom_mascal_type(tt: &TokenType) -> Option<MascalType> {
    match tt {
        TokenType::Integer => Some(MascalType::Integer),
        TokenType::Float => Some(MascalType::Float),
        TokenType::String => Some(MascalType::String),
        TokenType::Dynamic => Some(MascalType::Dynamic),
        TokenType::Type => Some(MascalType::Type),
        _ => None
    }
}