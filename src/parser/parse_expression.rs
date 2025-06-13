mod parse_primary;

use crate::defs::binding_power::{get_binding_power_from_bsign, get_binding_power_from_psign, BindingPower};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::{token_type_to_binary_operator, token_type_to_unary_operator, MascalBinaryOperators};
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::parse_primary::parse_primary;

pub fn parse_expression_internal(
    tokens: &[Token], pos: &mut usize, min_bp: BindingPower
) -> Result<MascalExpression, MascalError> {
    if tokens.is_empty() {
        return Ok(MascalExpression::SymbolicExpression("".to_string()));
    }
    let mut lhs: MascalExpression = parse_prefix(tokens, pos)?;

    'pratt: loop {
        match tokens.get(*pos).map(|t| &t.token_type) {
            Some(TokenType::Comma)
            | Some(TokenType::CloseBracket)
            | Some(TokenType::CloseArrow)
            | Some(TokenType::CloseParen)
            | None  // end of input
            => break 'pratt,
            _ => {}
        }

        let op_tok = tokens.get(*pos).unwrap();
        let binop = token_type_to_binary_operator(&op_tok.token_type);
        let bp = if let Some(op) = &binop {
            get_binding_power_from_bsign(op.clone())
        } else {
            break 'pratt;
        };

        if bp.left_binding_power < min_bp.left_binding_power {
            break 'pratt;
        }

        *pos += 1;
        let op = binop.unwrap();
        let rhs = parse_expression_internal(
            tokens,
            pos,
            BindingPower {
                left_binding_power: 0,
                right_binding_power: bp.right_binding_power,
            },
        )?;

        lhs = MascalExpression::BinaryExpression {
            left:     Box::new(lhs),
            operator: op,
            right:    Box::new(rhs),
        };
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
        let bp = get_binding_power_from_psign(op.clone());
        let rhs = parse_expression_internal(&tokens.to_vec(), pos, bp)?;
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