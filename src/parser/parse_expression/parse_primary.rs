use crate::defs::binding_power::BindingPower;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::token::{Token, TokenType};
use crate::defs::types::{token_type_to_atom_mascal_type};
use crate::parser::parse_expression::{parse_expression_internal};

fn parse_delimited_array(
    tokens: &[Token],
    pos: &mut usize,
    open_tok: TokenType,
    open_tok_chars: &str,
    close_tok: TokenType,
    close_tok_chars: &str,
) -> Result<Vec<MascalExpression>, MascalError> {
    let start: &Token = tokens.get(*pos)
        .ok_or_else(|| MascalError {
            error_type: MascalErrorType::ParserError,
            line: tokens.first().unwrap().line,
            character: tokens.first().unwrap().start,
            source: format!("Abrupt end of array, was expecting a opening character {:?} but got nothing", open_tok_chars)
        })?;
    if start.token_type != open_tok {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: start.line,
            character: start.start,
            source: format!("Expected a opening character {:?} but got {:?}", open_tok_chars, start.value)
        });
    }
    
    let mut elems: Vec<MascalExpression> = Vec::new();
    if tokens.get(*pos + 1).map(|t| &t.token_type) == Some(&close_tok) {
        *pos += 1;
    }
    while tokens.get(*pos).map(|t| &t.token_type) != Some(&close_tok) {
        *pos += 1;
        let e: MascalExpression = parse_expression_internal(
            tokens,
            pos,
            BindingPower { left_binding_power: 0, right_binding_power: 0 },
        )?;
        elems.push(e);
    }

    let closing: &Token = tokens.get(*pos)
        .ok_or_else(|| MascalError {
            error_type: MascalErrorType::ParserError,
            line: tokens.last().unwrap().line,
            character: tokens.last().unwrap().start,
            source: format!("Abrupt end of array, was expecting a closing character {:?} but got nothing", close_tok_chars)
        })?;
    if closing.token_type != close_tok {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: closing.line,
            character: closing.start,
            source: format!("Expected a closing character {:?} but got {:?}", close_tok_chars, closing.value)
        });
    }
    *pos += 1;

    Ok(elems)
}

pub fn parse_primary(tokens: &[Token], pos: &mut usize) -> Result<MascalExpression, MascalError> {
    let tok: &Token =  tokens.get(*pos).ok_or_else(|| MascalError {
        error_type: MascalErrorType::ParserError,
        character: tokens.last().unwrap().start,
        line: tokens.last().unwrap().line,
        source: String::from("Abrupt ending in a primary expression")
    })?;

    if let Some(next_tok) = tokens.get(*pos + 1) {
        match (&tok.token_type, &next_tok.token_type) {
            (TokenType::IntegerLiteral | TokenType::FloatLiteral, TokenType::Identifier) => {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    character: tok.start,
                    line: tok.line,
                    source: format!("Numbers cannot be directly followed by identifiers (found '{} {}')",
                                    tok.value, next_tok.value),
                });
            }
            _ => {}
        }
    }

    match &tok.token_type {
        TokenType::IntegerLiteral => {
            *pos += 1;
            Ok(MascalExpression::LiteralExpression(
                MascalLiteral::Integer(IntegerNum::new(tok.value.parse::<i128>().unwrap())),
            ))
        }

        TokenType::FloatLiteral => {
            *pos += 1;
            Ok(MascalExpression::LiteralExpression(
                MascalLiteral::Float(tok.value.parse::<f64>().unwrap()),
            ))
        }

        TokenType::True => {
            *pos += 1;
            Ok(MascalExpression::LiteralExpression(
                MascalLiteral::Boolean(true),
            ))
        }

        TokenType::False => {
            *pos += 1;
            Ok(MascalExpression::LiteralExpression(
                MascalLiteral::Boolean(false),
            ))
        }

        TokenType::NULL => {
            *pos += 1;
            Ok(MascalExpression::LiteralExpression(
                MascalLiteral::NULL,
            ))
        }

        tt if tt == &TokenType::Integer ||
            tt ==  &TokenType::String ||
            tt ==  &TokenType::Float ||
            tt == &TokenType::Boolean ||
            tt == &TokenType::Type ||
            tt == &TokenType::Dynamic => {
            *pos += 1;
            Ok(MascalExpression::TypeExpression(Box::new(token_type_to_atom_mascal_type(tt).unwrap())))
        }

        TokenType::StringLiteral => {
            *pos += 1;
            Ok(MascalExpression::LiteralExpression(
                MascalLiteral::String(tok.value.to_string()),
            ))
        }

        TokenType::Identifier => {
            *pos += 1;
            Ok(MascalExpression::SymbolicExpression(tok.value.to_string()))
        }

        TokenType::OpenParen => {
            *pos += 1;
            let expr: MascalExpression = parse_expression_internal(tokens, pos, BindingPower {
                left_binding_power: 0, right_binding_power: 0
            })?;
            let closing: &Token = tokens.get(*pos).ok_or_else(|| MascalError {
                error_type: MascalErrorType::ParserError,
                character: tok.start,
                line:      tok.line,
                source:    "Unclosed parenthesis".into(),
            })?;
            if closing.token_type != TokenType::CloseParen {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    character: closing.start,
                    line:      closing.line,
                    source:    "Expected a closing parenthesis ')'".into(),
                });
            }
            *pos += 1;
            Ok(expr)
        }

        TokenType::OpenBracket => {
            let elems: Vec<MascalExpression> = parse_delimited_array(
                tokens, pos, TokenType::OpenBracket, "[",
                TokenType::CloseBracket, "]"
            )?;
            Ok(MascalExpression::StaticArrayExpression(elems))
        }

        TokenType::OpenArrow => {
            let elems: Vec<MascalExpression> = parse_delimited_array(
                tokens, pos, TokenType::OpenArrow, "<",
                TokenType::CloseArrow, ">"
            )?;
            Ok(MascalExpression::DynamicArrayExpression(elems))
        }

        TokenType::CloseArrow => Err(MascalError {
            error_type: MascalErrorType::ParserError,
            character: tok.start,
            line: tok.line,
            source: String::from("Expected an opening arrow '<' in dynamic array literal"),
        }),

        TokenType::CloseBracket => Err(MascalError {
            error_type: MascalErrorType::ParserError,
            character: tok.start,
            line: tok.line,
            source: String::from("Expected an opening bracket '[' in static array literal"),
        }),

        _ => Err(MascalError {
            error_type: MascalErrorType::ParserError,
            character: tok.start,
            line: tok.line,
            source: format!("Unexpected characters in primary expression: {:?}", &tok.value),
        }),
    }
}