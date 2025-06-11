use crate::defs::literal::MascalLiteral;
use crate::defs::operators::MascalBinaryOperators;

#[derive(Debug, Clone)]
pub enum MascalExpression {
    // === Simple Expressions ===
    LiteralExpression(MascalLiteral),
    SymbolicExpression(MascalLiteral),
    DynamicArrayExpression(Vec<MascalExpression>),
    StaticArrayExpression(Vec<MascalExpression>),


    // === Complex Expressions ===
    BinaryExpression {
        left: Box<MascalExpression>,
        operator: MascalBinaryOperators,
        right: Box<MascalExpression>,
    },

    InnerMemberAccessExpression {
        member: Box<MascalExpression>,
        operator: String,
    },

    CallExpression {
        function: Box<MascalExpression>,
        arguments: Vec<MascalExpression>,
    }
}