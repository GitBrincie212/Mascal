use crate::defs::blocks::{ExecutionBlock, ScopedBlocks};
use crate::defs::errors::{MascalError};
use crate::defs::token::{TokenType};
use crate::parser::parse_executable_block::parse_executable;
use crate::parser::parse_variables::parse_variable_block;
use crate::parser::Parser;
use crate::parser::utils::{extract_braced_block};

pub fn parse_program(parser: Parser) -> Result<ScopedBlocks, MascalError> {
    let inner_parser = extract_braced_block(
        parser,
        "DEFINE_PROGRAM",
        &[TokenType::Variables],
        &[TokenType::Implementation],
    )?;
    let variable_block = parse_variable_block(&inner_parser)?;
    let program_body = parse_executable(inner_parser)?;
    
    Ok(ScopedBlocks::PROGRAM(ExecutionBlock {
        variables: variable_block,
        body: program_body,
    }))
}