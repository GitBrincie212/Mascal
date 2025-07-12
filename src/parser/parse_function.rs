use crate::defs::blocks::{ExecutionBlock, MascalParameter, ScopedBlocks};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::token::{Token, TokenType};
use crate::defs::types::MascalUnprocessedType;
use crate::parser::TokenSequence;
use crate::parser::parse_executable_block::parse_executable;
use crate::parser::parse_variables::parse_variable_block;
use crate::parser::utils::{extract_braced_block, parse_array_type};
use std::rc::Rc;

#[inline(always)]
fn get_parameters_of_func(
    token_sequence: &TokenSequence,
    mut curr_index: usize,
) -> Result<(Vec<MascalParameter>, usize), MascalError> {
    if !token_sequence.is_of(TokenType::OpenParen, curr_index) {
        return Ok((Vec::new(), curr_index));
    } else if token_sequence.is_of(TokenType::CloseParen, curr_index + 1) {
        return Ok((Vec::new(), curr_index + 2));
    }
    let mut is_mutable: bool = false;
    let mut parameter_name: Option<&str> = None;
    let mut parameters: Vec<MascalParameter> = Vec::new();
    while !token_sequence.is_of(TokenType::CloseParen, curr_index) {
        if curr_index >= token_sequence.tokens.len() {
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                line: token_sequence.first_token().line,
                character: token_sequence.first_token().start,
                source: String::from("Parameters have not been closed with a parenthesis"),
            });
        }
        let curr_token = token_sequence.acquire_token(curr_index);
        let next_token_type = &token_sequence
            .get_token(curr_index + 1)
            .unwrap_or(curr_token)
            .token_type;
        if curr_token.token_type == TokenType::Identifier {
            parameter_name = Some(curr_token.value);
        } else if curr_token.token_type == TokenType::Mutable {
            if parameter_name.is_some() {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    line: curr_token.line,
                    character: curr_token.start,
                    source: String::from(
                        "the keyword MUT should be followed before the variable's name",
                    ),
                });
            } else if is_mutable {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    line: curr_token.line,
                    character: curr_token.start,
                    source: String::from("the MUT keyword cannot stack on top of one variable"),
                });
            }
            is_mutable = true;
        }
        if next_token_type == &TokenType::CloseParen || curr_token.token_type == TokenType::Comma {
            if parameter_name.is_none() {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    line: curr_token.line,
                    character: curr_token.start,
                    source: String::from("expected to define parameter name"),
                });
            }
            parameters.push(MascalParameter {
                name: Rc::from(parameter_name.unwrap()),
                is_mutable,
            });
            parameter_name = None;
            is_mutable = false;
        }
        curr_index += 1;
    }

    Ok((parameters, curr_index + 1))
}

pub fn parse_function(token_sequence: TokenSequence) -> Result<ScopedBlocks, MascalError> {
    let mut curr_index: usize = 0;
    let name: String;
    if token_sequence.is_of(TokenType::Identifier, curr_index) {
        name = token_sequence.first_token().value.to_string();
        curr_index += 1;
    } else {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: token_sequence.first_token().line,
            character: token_sequence.first_token().start,
            source: String::from("Expected a identifier for the function name"),
        });
    }

    let returned_tuple = get_parameters_of_func(&token_sequence, curr_index)?;
    let parameters: Vec<MascalParameter> = returned_tuple.0;
    curr_index = returned_tuple.1;

    let mut return_type: Option<MascalUnprocessedType> = None;
    if token_sequence.is_of(TokenType::ReturnIndicator, curr_index) {
        curr_index += 1;
        let curr_token: &Token = token_sequence.acquire_token(curr_index);
        return_type = Some(match curr_token.token_type {
            TokenType::Integer => MascalUnprocessedType::Integer,
            TokenType::Float => MascalUnprocessedType::Float,
            TokenType::Boolean => MascalUnprocessedType::Boolean,
            TokenType::Dynamic => MascalUnprocessedType::Dynamic,
            TokenType::String => MascalUnprocessedType::String,
            TokenType::Type => MascalUnprocessedType::Type,
            _ => {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    line: curr_token.line,
                    character: curr_token.start,
                    source: String::from("Expected a specific type to be returned and got something else"),
                });
            }
        });
        curr_index += 1;
        let mut is_dynamics: Vec<bool> = Vec::new();
        curr_index = parse_array_type(
            &token_sequence.tokens,
            curr_index,
            |_tokens, is_dynamic| {
                is_dynamics.push(is_dynamic);
                Ok(())
            },
            vec![TokenType::OpenBrace],
        )?;
        if !is_dynamics.is_empty() {
            for is_dynamic in is_dynamics {
                if is_dynamic {
                    return_type = Some(MascalUnprocessedType::DynamicArray(Box::new(
                        return_type.unwrap(),
                    )));
                    continue;
                }
                return_type = Some(MascalUnprocessedType::StaticArray(Box::new(
                    return_type.unwrap(),
                )));
            }
        }
    }

    let inner_token_sequence = extract_braced_block(
        token_sequence.subsection_from(curr_index..),
        "DEFINE_FUNCTION",
        &[TokenType::Variables],
        &[TokenType::Implementation],
    )?;
    let variable_block = parse_variable_block(&inner_token_sequence)?;
    let program_body = parse_executable(inner_token_sequence)?;

    Ok(ScopedBlocks::Function {
        parameters: parameters.into_boxed_slice(),
        name,
        return_type,
        execution_block: ExecutionBlock {
            variables: variable_block,
            body: program_body.into_boxed_slice(),
        },
    })
}
