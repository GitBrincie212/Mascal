use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::statements::MascalStatement;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_statement::parse_statement;
use crate::parser::TokenSequence;
use crate::parser::utils::{locate_block, run_per_statement};

pub fn parse_executable(inner_parser: TokenSequence) -> Result<Vec<MascalStatement>, MascalError> {
    let mut statements: Vec<MascalStatement> = Vec::new();
    let program_parser = locate_block(
        inner_parser,
        TokenType::Implementation,
        "IMPLEMENTATION",
        &[],
        &[],
    )?.unwrap();

    let final_toks: Vec<Token> = run_per_statement(&program_parser, |token_sequence| {
        let stmt: MascalStatement = parse_statement(token_sequence)?;
        statements.push(stmt);
        return Ok(());
    })?;

    if !final_toks.is_empty() {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: final_toks[0].line,
            character: final_toks[0].start,
            source: String::from("Unexpected characters found inside implementation block, perhaps forgot a semicolon?")
        })
    }
    
    Ok(statements)
}