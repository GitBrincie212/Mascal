mod parse_program;
mod parse_function;
mod utils;
mod parse_variables;
mod parse_variable_decl;
mod parse_expression;
mod parse_executable_block;
mod parse_statement;


use std::ops::{Range, RangeFrom};
use crate::ast::AbstractSyntaxTree;
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

    pub fn first_token(&self) -> &Token {
        self.acquire_token(0)
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

pub fn parse(token_sequence: TokenSequence) -> Result<AbstractSyntaxTree, MascalError> {
    let mut scoped_blocks: Vec<ScopedBlocks> = Vec::new();
    for (index, token) in token_sequence.tokens.iter().enumerate() {
        match token.token_type {
            TokenType::DefineFunction => {
                let func = parse_function(token_sequence.subsection_from(index + 1..))?;
                scoped_blocks.push(func);
            }
            TokenType::DefineProgram => {
                let program = parse_program(token_sequence.subsection_from(index + 1..))?;
                scoped_blocks.push(program);
            }
            _ => {continue}
        }
    }
    let abstract_syntax_tree = AbstractSyntaxTree {blocks: scoped_blocks};
    Ok(abstract_syntax_tree)
}