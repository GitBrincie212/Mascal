use crate::defs::binding_power::{get_binding_power_from_bsign, BindingPower};
use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::operators::{token_type_to_binary_operator, MascalBinaryOperators};
use crate::defs::token::Token;
use crate::parser::parse_expression::loop_flags::LoopFlags;
use crate::parser::parse_expression::parse_expression_internal;

pub fn parse_binary_expression(
    tokens: &[Token], pos: &mut usize, min_bp: &BindingPower, mut lhs: MascalExpression
) -> Result<(LoopFlags, MascalExpression), MascalError> {
    let op_tok: &Token = tokens.get(*pos).unwrap();
    let binop: Option<MascalBinaryOperators> = token_type_to_binary_operator(&op_tok.token_type);
    let bp: BindingPower = if let Some(op) = &binop {
        get_binding_power_from_bsign(op.clone())
    } else {
        return Ok((LoopFlags::BREAK, lhs));
    };

    if bp.left_binding_power <= min_bp.left_binding_power {
        return Ok((LoopFlags::BREAK, lhs));
    }

    *pos += 1;
    let op: MascalBinaryOperators = binop.unwrap();
    let rhs: MascalExpression = parse_expression_internal(
        tokens,
        pos,
        BindingPower {
            right_binding_power: 0,
            left_binding_power: bp.right_binding_power,
        },
    )?;

    lhs = MascalExpression::BinaryExpression {
        left: Box::new(lhs),
        operator: op,
        right: Box::new(rhs),
    };
    
    Ok((LoopFlags::NONE, lhs))
}