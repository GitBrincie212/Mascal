mod parse_program;
mod parse_function;
mod utils;
mod parse_variables;
mod parse_variable_decl;
mod parse_expression;
mod parse_executable_block;
mod parse_statement;

use std::ops::{Range, RangeFrom};
use crate::defs::blocks::{ScopedBlocks};
use crate::defs::errors::MascalError;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_function::parse_function;
use crate::parser::parse_program::parse_program;

pub struct TokenSequence<'a> {
    pub tokens: Vec<Token<'a>>,
}

impl<'a> TokenSequence<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> TokenSequence<'a> {
        TokenSequence { tokens }
    }

    pub fn get_token(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }

    pub fn last_token(&self) -> &Token {
        self.acquire_token(self.tokens.len() - 1)
    }

    pub fn acquire_token(&self, index: usize) -> &Token {
        &self.tokens[index]
    }

    pub fn is_of(&self, target: TokenType, index: usize) -> bool {
        matches!(self.tokens.get(index).map(|t| &t.token_type), Some(t) if t == &target)
    }

    pub fn subsection_range(&self, bounds: Range<usize>) -> TokenSequence<'a> {
        TokenSequence::new(self.tokens[bounds].to_vec())
    }

    pub fn subsection_from(&self, bounds: RangeFrom<usize>) -> TokenSequence<'a> {
        TokenSequence::new(self.tokens[bounds].to_vec())
    }
}

pub fn parse(token_sequence: TokenSequence) -> Result<Vec<ScopedBlocks>, MascalError> {
    let mut scoped_blocks: Vec<ScopedBlocks> = Vec::new();
    for (index, token) in token_sequence.tokens.iter().enumerate() {
        match token.token_type {
            TokenType::DefineFunction => {
                let func = parse_function(&token, token_sequence.subsection_from(index + 1..))?;
                /*
                scoped_blocks.push(ScopedBlocks::FUNCTION {
                    name: ,
                    return_type: ,
                    parameters: ,
                    execution_block: ExecutionBlock {
                        variables:,
                        body:
                    }
                });
                 */
            }
            TokenType::DefineProgram => {
                let program = parse_program(token_sequence.subsection_from(index + 1..))?;
                dbg!(program);
                /*
                scoped_blocks.push(ScopedBlocks::PROGRAM(ExecutionBlock {
                    variables:,
                    body:
                }));
                 */
            }
            _ => {continue}
        }
    }
    Ok(scoped_blocks)
}