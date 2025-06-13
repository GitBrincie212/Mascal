use crate::defs::binding_power::BindingPower;
use crate::defs::errors::{MascalError};
use crate::defs::expressions::MascalExpression;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::loop_flags::LoopFlags;
use crate::parser::parse_expression::{parse_expression_internal};

pub fn parse_inner_member(
    tokens: &[Token], pos: &mut usize, min_bp: &BindingPower, lhs: MascalExpression
) -> Result<(LoopFlags, MascalExpression), MascalError> {
    if matches!(tokens.get(*pos).map(|t| &t.token_type), Some(TokenType::Dot)) {
        *pos += 1;
        return Ok((LoopFlags::BREAK, MascalExpression::InnerMemberAccessExpression {
            member: Box::new(lhs),
            value: Box::new(parse_expression_internal(tokens, pos, min_bp.clone())?),
        }));
    }
    Ok((LoopFlags::NONE, lhs))
}