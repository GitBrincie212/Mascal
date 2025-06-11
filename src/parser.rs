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

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Parser<'a> {
        Parser { tokens, current: 0 }
    }

    pub fn all_tokens(&self) -> &[Token<'a>] {
        &self.tokens
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn current_index(&self) -> usize {
        self.current
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

    pub fn advance(&mut self) {
        self.current += 1;
    }

    pub fn subsection_range(&self, bounds: Range<usize>) -> Parser<'a> {
        Parser::new(self.tokens[bounds].to_vec())
    }

    pub fn subsection_from(&self, bounds: RangeFrom<usize>) -> Parser<'a> {
        Parser::new(self.tokens[bounds].to_vec())
    }

    pub fn parse(&self) -> Result<Vec<ScopedBlocks>, MascalError> {
        let mut scoped_blocks: Vec<ScopedBlocks> = Vec::new();
        for (index, token) in self.tokens.iter().enumerate() {
            match token.token_type {
                TokenType::DefineFunction => {
                    let func = parse_function(&token, self.subsection_from(index + 1..))?;
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
                    let program = parse_program(self.subsection_from(index + 1..))?;
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
}