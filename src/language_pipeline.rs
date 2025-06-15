use crate::defs::errors::{MascalErrorType, MascalError};
use crate::defs::token::{Token, TokenType};
use crate::lexer;
use crate::parser::{parse, TokenSequence};
use crate::semantic_analysis::conduct_semantic_analysis;

macro_rules! define_pipeline_step {
    ($func: expr, $($args: expr)*) => {
        match $func($($args),*) {
            Ok(val) => Some(val),
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    };
}

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
    let token_sequence: TokenSequence = TokenSequence::new(tokens);
    let Some(tree) = define_pipeline_step!(parse, token_sequence) else {return};
    let Some(tree) = define_pipeline_step!(conduct_semantic_analysis, tree) else {return};
}