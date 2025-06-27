use crate::defs::binding_power::BindingPower;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::loop_flags::LoopFlags;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::parse_expression_internal;

macro_rules! parse_inner_expr_of_array {
    (
        $lhs: expr, $pos: expr, $tokens: expr, $open_tok: expr,
        $close_tok: expr, $close_tok_char: expr, $is_dynamic: expr
    ) => {
        *$pos += 1;
        let mut indexing_length: usize = 0;
        let mut has_found_closing: bool = false;
        let mut depth = 1;
        if *$pos >= $tokens.len() {
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                line: $tokens.last().unwrap().line,
                character: $tokens.last().unwrap().start,
                source: format!("Expected {:?} after index expression", $close_tok_char)
            });
        }
        for tok in &$tokens[*$pos..] {
            indexing_length += 1;
            match &tok.token_type {
                tt if *tt == $open_tok => {
                    depth += 1;
                }

                tt if *tt == $close_tok => {
                    depth -= 1;
                    if depth == 0 {
                        has_found_closing = true;
                        break;
                    }
                }

                _ => {},
            }
        }
        if !has_found_closing {
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                line: $tokens[*$pos].line,
                character: $tokens[*$pos].start,
                source: format!("Expected {:?} after index expression", $close_tok_char)
            });
        }
        let idx: MascalExpression = parse_expression_internal(
            &$tokens[*$pos..*$pos + indexing_length - 1], &mut 0usize, BindingPower::new(0)
        )?;
        *$pos += indexing_length;
        $lhs = MascalExpression::Indexing {
            array: Box::new($lhs),
            index: Box::new(idx),
            is_dynamic: $is_dynamic,
        };
    };
}

pub fn parse_indexing_expression(
    tokens: &[Token],
    pos: &mut usize,
    _min_bp: &BindingPower,
    mut lhs: MascalExpression,
) -> Result<(LoopFlags, MascalExpression), MascalError> {
    loop {
        let curr: Option<&Token> = tokens.get(*pos);
        match curr.map(|t| &t.token_type) {
            Some(TokenType::OpenBracket) => {
                parse_inner_expr_of_array!(
                    lhs, pos, tokens, TokenType::OpenBracket,
                    TokenType::CloseBracket, "]", false
                );
            }

            Some(TokenType::OpenDynamicArray) => {
                parse_inner_expr_of_array!(
                    lhs, pos, tokens, TokenType::OpenDynamicArray,
                    TokenType::CloseDynamicArray, ">>", true
                );
            }

            Some(TokenType::CloseBracket) => {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    line: curr.unwrap().line,
                    character: curr.unwrap().start,
                    source: String::from("Expected an opening character \"[\" before closing an unopened static array")
                })
            }
            
            Some(TokenType::CloseDynamicArray) => {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    line: curr.unwrap().line,
                    character: curr.unwrap().start,
                    source: String::from("Expected an opening character \"<<\" before closing an unopened dynamic array")
                })
            }

            _ => { return  Ok((LoopFlags::None, lhs))},
        }
    }
}
