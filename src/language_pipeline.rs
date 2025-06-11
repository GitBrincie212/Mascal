use crate::defs::blocks::ScopedBlocks;
use crate::defs::errors::{MascalErrorType, MascalError};
use crate::defs::token::{Token, TokenType};
use crate::lexer;
use crate::parser::Parser;

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
    let lang_parser: Parser = Parser::new(tokens);
    let resulted_blocks: Result<Vec<ScopedBlocks>, MascalError> = lang_parser.parse();
    if resulted_blocks.is_err() {
        println!("{}", resulted_blocks.err().unwrap());
        return;
    }
    let blocks: Vec<ScopedBlocks> = resulted_blocks.unwrap();
    // dbg!(tokens);
}