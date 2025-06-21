use crate::defs::literal::MascalLiteral;
use crate::defs::operators::{MascalBinaryOperators, MascalUnaryOperators};
use crate::defs::types::MascalUnprocessedType;

#[derive(Debug, Clone)]
pub enum MascalExpression {
    // === Simple Expressions ===
    LiteralExpression(MascalLiteral),
    SymbolicExpression(String),
    DynamicArrayExpression(Vec<MascalExpression>),
    StaticArrayExpression(Vec<MascalExpression>),
    TypeExpression(Box<MascalUnprocessedType>),

    // === Complex Expressions ===
    UnaryExpression {
        operator: MascalUnaryOperators,
        value: Box<MascalExpression>,
    },
    
    BinaryExpression {
        left: Box<MascalExpression>,
        operator: MascalBinaryOperators,
        right: Box<MascalExpression>,
    },

    CallExpression {
        function: Box<MascalExpression>,
        arguments: Vec<MascalExpression>,
    },
    
    IndexExpression {
        array: Box<MascalExpression>,
        index: Box<MascalExpression>,
        is_dynamic: bool
    }
}