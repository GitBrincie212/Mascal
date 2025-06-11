use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::token::Token;

pub fn parse_expression(token_sequence: &Vec<Token>) -> Result<MascalExpression, MascalError> {
    Ok(MascalExpression::LiteralExpression(MascalLiteral::NULL))
}