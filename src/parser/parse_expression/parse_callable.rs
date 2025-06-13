use crate::defs::binding_power::BindingPower;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::loop_flags::LoopFlags;
use crate::parser::parse_expression::parse_expression_internal;

pub fn parse_callable(
    tokens: &[Token], pos: &mut usize, _min_bp: &BindingPower, mut lhs: MascalExpression
) -> Result<(LoopFlags, MascalExpression), MascalError> {
    if matches!(tokens.get(*pos).map(|t| &t.token_type), Some(TokenType::OpenParen)) {
        *pos += 1;

        let mut args: Vec<MascalExpression> = Vec::new();
        if !matches!(tokens.get(*pos).map(|t| &t.token_type), Some(TokenType::CloseParen)) {
            loop {
                let arg: MascalExpression = parse_expression_internal(tokens, pos, BindingPower { left_binding_power: 0, right_binding_power: 0 })?;
                args.push(arg);

                let curr_tok: &Token = tokens.get(*pos).unwrap();
                match curr_tok.token_type {
                    TokenType::Comma => *pos += 1,
                    TokenType::CloseParen => break,
                    _ => {
                        return Err(MascalError {
                            error_type: MascalErrorType::ParserError,
                            character: curr_tok.start,
                            line: curr_tok.line,
                            source: format!(
                                "Expected a comma ',' or closing parenthesis ')' in the function call, but got {:?}", 
                                curr_tok.value
                            ),
                        });
                    }
                }
            }
        }

        if tokens.get(*pos).map(|t| &t.token_type) != Some(&TokenType::CloseParen) {
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                character: tokens.get(*pos).map_or(0, |t| t.start),
                line: tokens.get(*pos).map_or(0, |t| t.line),
                source: "Expected a closing parenthesis ')' to close the function call".into(),
            });
        }
        *pos += 1;

        lhs = MascalExpression::CallExpression {
            function: Box::new(lhs),
            arguments: args,
        };

        return Ok((LoopFlags::CONTINUE, lhs));
    }
    
    Ok((LoopFlags::NONE, lhs))
}