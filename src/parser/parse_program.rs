use crate::defs::blocks::{ExecutionBlock, ScopedBlocks};
use crate::defs::errors::{MascalError};
use crate::defs::token::{TokenType};
use crate::parser::parse_executable_block::parse_executable;
use crate::parser::parse_variables::parse_variable_block;
use crate::parser::TokenSequence;
use crate::parser::utils::{extract_braced_block};

pub fn parse_program(token_sequence: TokenSequence) -> Result<ScopedBlocks, MascalError> {
    let inner_token_sequence = extract_braced_block(
        token_sequence,
        "DEFINE_PROGRAM",
        &[TokenType::Variables],
        &[TokenType::Implementation],
    )?;
    let variable_block = parse_variable_block(&inner_token_sequence)?;
    let program_body = parse_executable(inner_token_sequence)?;
    
    Ok(ScopedBlocks::PROGRAM(ExecutionBlock {
        variables: variable_block,
        body: program_body,
    }))
}