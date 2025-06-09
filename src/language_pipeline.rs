use crate::defs::errors::{MascalErrorType, MascalError};
use crate::defs::token::{Token, TokenType};
use crate::lexer;

pub fn trigger_pipeline(contents: String) {
    let tokens: Vec<Token> = lexer::tokenize(&*contents);
    if tokens.is_empty() {return;}
    let last_token: &Token = tokens.last().unwrap();
    if last_token.token_type == TokenType::Unknown {
        println!("{}", MascalError {
            error_type: MascalErrorType::LexerError,
            line: last_token.line,
            character: last_token.start,
            source: format!("Unknown Character Sequence \"{}\"", last_token.value)
        });
        return;
    }
}