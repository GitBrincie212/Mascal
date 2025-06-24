use crate::defs::binding_power::BindingPower;
use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::loop_flags::LoopFlags;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::parse_expression_internal;

pub fn parse_indexing_expression(
    tokens: &[Token],
    pos: &mut usize,
    _min_bp: &BindingPower,
    lhs: MascalExpression,
) -> Result<(LoopFlags, MascalExpression), MascalError> {
    let tok = tokens.get(*pos).map(|t| &t.token_type);
    let is_dynamic_array = tok == Some(&TokenType::OpenDynamicArray);
    if !is_dynamic_array && tok != Some(&TokenType::OpenBracket) {
        return Ok((LoopFlags::None, lhs));
    }

    let mut depth = 0;
    let mut match_idx = None;
    for (i, tok) in tokens.iter().enumerate().skip(*pos) {
        match tok.token_type {
            TokenType::OpenDynamicArray | TokenType::OpenBracket => depth += 1,
            TokenType::CloseDynamicArray | TokenType::CloseBracket => {
                depth -= 1;
                if depth == 0 {
                    match_idx = Some(i);
                    break;
                }
            }
            _ => {}
        }
    }
    let end = if let Some(i) = match_idx {
        i
    } else {
        return Ok((LoopFlags::None, lhs));
    };

    if tokens
        .get(end + 1)
        .map(|t| {
            matches!(
                t.token_type,
                TokenType::Identifier
                    | TokenType::IntegerLiteral
                    | TokenType::FloatLiteral
                    | TokenType::StringLiteral
                    | TokenType::OpenParen
                    | TokenType::OpenBracket
            )
        })
        .unwrap_or(false)
    {
        return Ok((LoopFlags::None, lhs));
    }

    let open_tt = if is_dynamic_array {
        TokenType::OpenDynamicArray
    } else {
        TokenType::OpenBracket
    };
    let close_tt = if is_dynamic_array {
        TokenType::CloseDynamicArray
    } else {
        TokenType::CloseBracket
    };
    let is_dynamic = true;

    *pos += 1;
    let mut inner_depth = 1;
    let mut indices = Vec::new();
    while *pos < tokens.len() {
        let token = &tokens[*pos];
        *pos += 1;
        match &token.token_type {
            tt if tt == &close_tt => {
                inner_depth -= 1;
                if inner_depth == 0 {
                    break;
                }
            }
            tt if tt == &open_tt => inner_depth += 1,
            _ => {}
        }
        if inner_depth > 0 {
            indices.push(token.clone());
        }
    }

    let mut inner_pos: usize = 0;
    let idx_expr: MascalExpression = parse_expression_internal(
        &indices,
        &mut inner_pos,
        BindingPower {
            left_binding_power: 0,
            right_binding_power: 0,
        },
    )?;

    Ok((
        LoopFlags::Continue,
        MascalExpression::Indexing {
            array: Box::new(lhs),
            index: Box::new(idx_expr),
            is_dynamic,
        },
    ))
}
