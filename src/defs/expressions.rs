use crate::defs::literal::MascalLiteral;
use crate::defs::operators::MascalBinaryOperators;

#[derive(Debug, Clone, PartialEq)]
pub enum MascalExpression {
    // === Simple Expressions ===
    LiteralExpression(MascalLiteral),
    SymbolicExpression(MascalLiteral),


    // === Complex Expressions ===
    BinaryExpression {
        left: Box<MascalExpression>,
        operator: MascalBinaryOperators,
        right: Box<MascalExpression>,
    },

    InnerMemberExpression {
        member: Box<MascalExpression>,
        operator: String,
    },

    CallExpression {
        function: Box<MascalExpression>,
        arguments: Vec<MascalExpression>,
    }
}