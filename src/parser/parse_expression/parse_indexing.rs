use crate::defs::binding_power::{BindingPower};
use crate::defs::errors::{MascalError};
use crate::defs::expressions::MascalExpression;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::loop_flags::LoopFlags;
use crate::parser::parse_expression::parse_expression_internal;

pub fn parse_indexing_expression(
    tokens: &[Token], pos: &mut usize, _min_bp: &BindingPower, lhs: MascalExpression
) -> Result<(LoopFlags, MascalExpression), MascalError> {
    let tok = tokens.get(*pos).unwrap();
    let (open_tt, close_tt, is_dynamic) = match &tok.token_type {
        TokenType::OpenBracket => (TokenType::OpenBracket, TokenType::CloseBracket, false),
        TokenType::OpenArrow => (TokenType::OpenArrow, TokenType::CloseArrow, true),
        _ => return Ok((LoopFlags::NONE, lhs)),
    };
    *pos += 1;
    let mut depth: usize = 1;
    let mut indices: Vec<Token> = Vec::new();
    while *pos < tokens.len() {
        let token: &Token = &tokens[*pos];
        *pos += 1;
        if token.token_type == close_tt {
            depth -= 1;
        } else if token.token_type == open_tt {
            depth += 1;
        }
        if depth == 0 {break;}
        indices.push(token.clone());
    }

    let index: MascalExpression = parse_expression_internal(
        indices.as_slice(),
        &mut 0usize, 
        BindingPower {left_binding_power: 0, right_binding_power: 0 }
    )?;
    
    Ok((LoopFlags::CONTINUE, MascalExpression::IndexExpression {
        array: Box::new(lhs),
        index: Box::new(index),
        is_dynamic
    }))
}