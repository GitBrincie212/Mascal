use crate::defs::errors::MascalError;
use crate::defs::token::Token;
use crate::parser::Parser;

pub fn parse_function<'a>(preceding_token: &Token, parser: Parser<'a>) -> Result<&'a [Token<'a>], MascalError> {
    Ok(&[])
}