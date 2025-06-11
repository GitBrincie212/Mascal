use crate::defs::errors::MascalError;
use crate::defs::statements::MascalStatement;
use crate::defs::token::TokenType;
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

    run_per_statement(&program_parser, |token_sequence| {
        let stmt: MascalStatement = parse_statement(token_sequence)?;
        statements.push(stmt);
        return Ok(());
    })?;
    
    Ok(statements)
}