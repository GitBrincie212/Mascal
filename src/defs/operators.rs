use crate::defs::token::TokenType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MascalUnaryOperators {
    Not,
    Minus,
    Typeof
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
    Exponentiation
}

pub fn token_type_to_binary_operator(tt: &TokenType) -> Option<MascalBinaryOperators> {
    match tt {
        TokenType::Plus => Some(MascalBinaryOperators::Plus),
        TokenType::Minus => Some(MascalBinaryOperators::Minus),
        TokenType::Asterisk => Some(MascalBinaryOperators::Multiply),
        TokenType::Division => Some(MascalBinaryOperators::Divide),
        TokenType::Modulo => Some(MascalBinaryOperators::Modulo),
        TokenType::Exponentiation => Some(MascalBinaryOperators::Exponentiation),
        TokenType::Equals => Some(MascalBinaryOperators::Equals),
        TokenType::GreaterThanEqual => Some(MascalBinaryOperators::GreaterThanOrEqual),
        TokenType::LesserThanEqual => Some(MascalBinaryOperators::LessThanOrEqual),
        TokenType::And => Some(MascalBinaryOperators::And),
        TokenType::Or => Some(MascalBinaryOperators::Or),
        TokenType::NotEquals => Some(MascalBinaryOperators::NotEqual),
        TokenType::OpenArrow => Some(MascalBinaryOperators::LessThan),
        TokenType::CloseArrow => Some(MascalBinaryOperators::GreaterThan),
        _ => None
    }
}

pub fn token_type_to_unary_operator(tt: &TokenType) -> Option<MascalUnaryOperators> {
    match tt {
        TokenType::Minus =>  Some(MascalUnaryOperators::Minus),
        TokenType::Not =>  Some(MascalUnaryOperators::Not),
        TokenType::Typeof =>  Some(MascalUnaryOperators::Typeof),
        _ => None
    }
}