use crate::defs::errors::MascalError;
use crate::defs::token::Token;
use crate::parser::TokenSequence;

pub fn parse_function<'a>(preceding_token: &Token, parser: TokenSequence<'a>) -> Result<&'a [Token<'a>], MascalError> {
    Ok(&[])
}