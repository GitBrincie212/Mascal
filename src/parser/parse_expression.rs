mod parse_primary;
mod loop_flags;
mod parse_binary_expression;
mod parse_callable;
mod parse_inner_member;
mod utils;
mod parse_indexing;

use crate::{define_parsing_step};
use crate::defs::binding_power::{get_binding_power_from_psign, BindingPower};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::{token_type_to_unary_operator};
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::loop_flags::LoopFlags;
use crate::parser::parse_expression::parse_binary_expression::parse_binary_expression;
use crate::parser::parse_expression::parse_callable::parse_callable;
use crate::parser::parse_expression::parse_indexing::parse_indexing_expression;
use crate::parser::parse_expression::parse_inner_member::parse_inner_member;
use crate::parser::parse_expression::parse_primary::parse_primary;

pub fn parse_expression_internal(
    tokens: &[Token], pos: &mut usize, min_bp: BindingPower
) -> Result<MascalExpression, MascalError> {
    if tokens.is_empty() {
        return Ok(MascalExpression::SymbolicExpression("ligma".to_string()));
    }
    let mut lhs: MascalExpression = parse_prefix(tokens, pos)?;

    'pratt: loop {
        match tokens.get(*pos).map(|t| &t.token_type) {
            Some(TokenType::Comma)
            | Some(TokenType::CloseBracket)
            | Some(TokenType::CloseArrow)
            | Some(TokenType::CloseParen)
            | None
            => break 'pratt,
            _ => {}
        }

        define_parsing_step!(parse_indexing_expression, tokens, pos, &min_bp, lhs);
        define_parsing_step!(parse_inner_member, tokens, pos, &min_bp, lhs);
        define_parsing_step!(parse_callable, tokens, pos, &min_bp, lhs);
        define_parsing_step!(parse_binary_expression, tokens, pos, &min_bp, lhs);
    }

    Ok(lhs)
}

pub fn parse_prefix(
    tokens: &[Token], pos: &mut usize,
) -> Result<MascalExpression, MascalError> {
    let tok =  tokens.get(*pos).ok_or_else(|| MascalError {
        error_type: MascalErrorType::ParserError,
        character: tokens.last().unwrap().start,
        line: tokens.last().unwrap().line,
        source: String::from("Abrupt ending in the expression")
    })?;

    if let Some(op) = token_type_to_unary_operator(&tok.token_type) {
        *pos += 1;
        let bp: BindingPower = get_binding_power_from_psign(op.clone());
        let rhs: MascalExpression = parse_expression_internal(&tokens.to_vec(), pos, bp)?;
        return Ok(MascalExpression::UnaryExpression {
            operator: op,
            value: Box::new(rhs),
        });
    }
    
    parse_primary(tokens, pos)
}

pub fn parse_expression(token_sequence: &Vec<Token>) -> Result<MascalExpression, MascalError> {
    let mut pos: usize = 0;
    let parsed_expression: MascalExpression = parse_expression_internal(token_sequence, &mut pos, BindingPower {
        left_binding_power: 0,
        right_binding_power: 0
    })?;
    Ok(parsed_expression)
}