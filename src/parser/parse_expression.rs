mod parse_binary_expression;
mod parse_callable;
mod parse_indexing;
mod parse_primary;
mod utils;

use crate::define_parsing_step;
use crate::defs::binding_power::{BindingPower, get_binding_power_from_psign};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::loop_flags::LoopFlags;
use crate::defs::operators::token_type_to_unary_operator;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::parse_binary_expression::parse_binary_expression;
use crate::parser::parse_expression::parse_callable::parse_callable;
use crate::parser::parse_expression::parse_indexing::parse_indexing_expression;
use crate::parser::parse_expression::parse_primary::parse_primary;

pub fn parse_expression_internal(
    tokens: &[Token],
    pos: &mut usize,
    min_bp: BindingPower,
) -> Result<MascalExpression, MascalError> {
    if tokens.is_empty() {
        return Ok(MascalExpression::Blank);
    }
    let mut lhs: MascalExpression = parse_prefix(tokens, pos)?;

    loop {
        let curr: Option<&Token> = tokens.get(*pos);
        match curr.map(|t| &t.token_type) {
            Some(TokenType::Comma)
            | Some(TokenType::CloseBracket)
            | Some(TokenType::CloseDynamicArray)
            | Some(TokenType::CloseParen)
            | None => break,

            _ => {}
        }

        define_parsing_step!(parse_indexing_expression, tokens, pos, &min_bp, lhs);
        define_parsing_step!(parse_callable, tokens, pos, &min_bp, lhs);
        define_parsing_step!(parse_binary_expression, tokens, pos, &min_bp, lhs);
    }

    Ok(lhs)
}

pub fn parse_prefix(tokens: &[Token], pos: &mut usize) -> Result<MascalExpression, MascalError> {
    let tok = tokens.get(*pos).ok_or_else(|| MascalError {
        error_type: MascalErrorType::ParserError,
        character: tokens.last().unwrap().start,
        line: tokens.last().unwrap().line,
        source: String::from("Abrupt ending in the expression"),
    })?;

    if let Some(op) = token_type_to_unary_operator(&tok.token_type) {
        *pos += 1;
        let bp: BindingPower = get_binding_power_from_psign(op.clone());
        let rhs: MascalExpression = parse_expression_internal(tokens, pos, bp)?;
        return Ok(MascalExpression::Unary {
            operator: op,
            value: Box::new(rhs),
        });
    }

    parse_primary(tokens, pos)
}

pub fn parse_expression(token_sequence: &Vec<Token>) -> Result<MascalExpression, MascalError> {
    let mut pos: usize = 0;
    let parsed_expression: MascalExpression = parse_expression_internal(
        token_sequence,
        &mut pos,
        BindingPower {
            left_binding_power: 0,
            right_binding_power: 0,
        },
    )?;
    if pos < token_sequence.len() - 1 {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: token_sequence[pos].line,
            character: token_sequence[pos].start,
            source: String::from("Unexpected character sequences found in a supposed expression")
        })
    }
    Ok(parsed_expression)
}
