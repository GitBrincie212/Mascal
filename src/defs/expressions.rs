use crate::defs::literal::MascalLiteral;
use crate::defs::operators::{MascalBinaryOperators, MascalUnaryOperators};
use crate::defs::types::MascalType;

#[derive(Debug, Clone)]
pub enum MascalExpression {
    // === Simple Expressions ===
    LiteralExpression(MascalLiteral),
    SymbolicExpression(String),
    DynamicArrayExpression(Vec<MascalExpression>),
    StaticArrayExpression(Vec<MascalExpression>),
    TypeExpression(Box<MascalType>),

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

    InnerMemberAccessExpression {
        member: Box<MascalExpression>,
        value: Box<MascalExpression>,
    },

    CallExpression {
        function: Box<MascalExpression>,
        arguments: Vec<MascalExpression>,
    }
}