#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MascalUnaryOperators {
    Not,
    Minus
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MascalBinaryOperators {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    And,
    Or,
    NotEqual,
    VariableAssign,
}