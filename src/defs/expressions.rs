use crate::defs::literal::MascalLiteral;
use crate::defs::operators::{MascalBinaryOperators, MascalUnaryOperators};
use crate::defs::types::MascalUnprocessedType;

#[derive(Debug, Clone, PartialEq)]
pub enum MascalExpression {
    // === Simple Expressions ===
    Literal(MascalLiteral),
    Symbolic(String),
    DynamicArray(Box<[MascalExpression]>),
    StaticArray(Box<[MascalExpression]>),
    Type(Box<MascalUnprocessedType>),

    // === Complex Expressions ===
    Unary {
        operator: MascalUnaryOperators,
        value: Box<MascalExpression>,
    },

    Binary {
        left: Box<MascalExpression>,
        operator: MascalBinaryOperators,
        right: Box<MascalExpression>,
    },

    Call {
        function: Box<MascalExpression>,
        arguments: Vec<MascalExpression>,
    },

    Indexing {
        array: Box<MascalExpression>,
        index: Box<MascalExpression>,
        is_dynamic: bool,
    },
}
