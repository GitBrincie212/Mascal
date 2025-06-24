use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::token::{SCOPABLE_TOKEN_TYPES, Token, TokenType};
use crate::parser::TokenSequence;
use std::collections::HashSet;

pub fn extract_braced_block<'a>(
    token_sequence: TokenSequence<'a>,
    block_name: &'static str,
    allow_nested: &[TokenType],
    require_inside: &[TokenType],
) -> Result<TokenSequence<'a>, MascalError> {
    if !token_sequence.is_of(TokenType::OpenBrace, 0) {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: token_sequence.acquire_token(0).line,
            character: token_sequence.acquire_token(0).start,
            source: format!("{block_name} block must start with '{{'"),
        });
    }

    let mut depth: usize = 0;
    let mut found_required: HashSet<TokenType> = HashSet::new();

    for (i, token) in token_sequence.tokens.iter().enumerate() {
        if !SCOPABLE_TOKEN_TYPES.contains(&token.token_type)
            && token.token_type != TokenType::CloseBrace
            && token.token_type != TokenType::OpenBrace
        {
            continue;
        }
        match token.token_type {
            TokenType::OpenBrace => depth += 1,
            TokenType::CloseBrace => {
                depth -= 1;
                if depth == 0 {
                    let missing: Vec<String> = require_inside
                        .iter()
                        .filter(|r| !found_required.contains(r))
                        .map(|r| format!("{:?}", r))
                        .collect();
                    
                    if !missing.is_empty() {
                        return Err(MascalError {
                            error_type: MascalErrorType::ParserError,
                            line: token.line,
                            character: token.start,
                            source: format!("Missing required block(s): {}", missing.join(", ")),
                        });
                    }
                    return Ok(token_sequence.subsection_range(1..i));
                }
            }

            ref tt => {
                if token_sequence.tokens[i + 1].token_type != TokenType::OpenBrace {
                    continue;
                }
                let is_required = require_inside.contains(tt);
                let is_optional_nest = allow_nested.contains(tt);

                if is_required {
                    found_required.insert(tt.clone());
                } else if depth == 1 && !is_optional_nest {
                    return Err(MascalError {
                        error_type: MascalErrorType::ParserError,
                        line: token.line,
                        character: token.start,
                        source: format!(
                            "Token '{:?}' is not allowed in nested blocks of {block_name}",
                            tt
                        ),
                    });
                } else if !is_required && (!is_optional_nest && depth == 1) {
                    return Err(MascalError {
                        error_type: MascalErrorType::ParserError,
                        line: token.line,
                        character: token.start,
                        source: format!("Token '{:?}' is not allowed in {block_name} block", tt),
                    });
                }
            }
        }
    }

    Err(MascalError {
        error_type: MascalErrorType::ParserError,
        character: token_sequence.last_token().start,
        line: token_sequence.last_token().line,
        source: format!("{block_name} block not properly closed"),
    })
}

pub fn extract_braced_block_from_tokens<'a>(
    tokens: &'a [Token<'a>],
    block_name: &'static str,
    allow_nested: &[TokenType],
    require_inside: &[TokenType],
) -> Result<TokenSequence<'a>, MascalError> {
    extract_braced_block(
        TokenSequence::new(tokens.to_vec()),
        block_name,
        allow_nested,
        require_inside,
    )
}

#[inline(always)]
pub fn locate_block<'a>(
    token_sequence: TokenSequence<'a>,
    token_type: TokenType,
    block_name: &'static str,
    allow_nested: &[TokenType],
    require_inside: &[TokenType],
) -> Result<Option<TokenSequence<'a>>, MascalError> {
    for (index, token) in token_sequence.tokens.iter().enumerate() {
        if token.token_type != token_type {
            continue;
        }
        let subset_tokens: TokenSequence = extract_braced_block(
            token_sequence.subsection_from(index + 1..),
            block_name,
            allow_nested,
            require_inside,
        )?;

        return Ok(Some(subset_tokens));
    }

    Ok(None)
}

pub fn run_per_statement<'a, F>(
    token_sequence: &'a TokenSequence,
    mut func: F,
) -> Result<Vec<Token<'a>>, MascalError>
where
    F: (FnMut(&Vec<Token<'a>>) -> Result<(), MascalError>),
{
    let mut statement_token_seq: Vec<Token> = Vec::new();
    let mut depth_counter: usize = 0;
    let mut entered_conditional_stmt: bool = false;
    for (index, token) in token_sequence.tokens.iter().enumerate() {
        statement_token_seq.push(token.clone());
        match token.token_type {
            TokenType::If => {
                if depth_counter == 0 {
                    entered_conditional_stmt = true;
                }
            }

            TokenType::OpenBrace => {
                depth_counter += 1;
            }

            TokenType::CloseBrace => {
                depth_counter -= 1;
                if depth_counter > 0 {
                    continue;
                }
                let lookahead = token_sequence
                    .tokens
                    .get(index + 1)
                    .map(|x| &x.token_type)
                    .unwrap_or(&TokenType::Null);
                if !entered_conditional_stmt {
                    func(&statement_token_seq)?;
                    statement_token_seq.clear();
                    continue;
                }
                if *lookahead == TokenType::ElseIf || *lookahead == TokenType::Else {
                    continue;
                }
                entered_conditional_stmt = false;
                func(&statement_token_seq)?;
                statement_token_seq.clear();
            }

            TokenType::Semicolon => {
                if depth_counter > 0 {
                    continue;
                }
                func(&statement_token_seq)?;
                statement_token_seq.clear();
                continue;
            }
            _ => {}
        }
    }

    Ok(statement_token_seq)
}

pub fn parse_array_type<F>(
    tokens: &[Token],
    mut curr_index: usize,
    mut on_creation: F,
    terminator_types: Vec<TokenType>,
) -> Result<usize, MascalError>
where
    F: (FnMut(&[Token], bool) -> Result<(), MascalError>),
{
    let mut bracket_depth: usize = 0;
    let mut arrow_depth: usize = 0;
    let mut last_token: &Token = &tokens[curr_index];
    let first_token: &Token = &tokens[curr_index];

    let mut open_index: usize = 0;
    let mut is_curr_array_dynamic: Option<bool> = None;
    let is_array_open: bool = first_token.token_type == TokenType::OpenBracket
        || first_token.token_type == TokenType::OpenDynamicArray;
    let mut token_sequence: Vec<&Token> = Vec::new();
    while curr_index < tokens.len() && is_array_open {
        let token: &Token = &tokens[curr_index];
        match tokens[curr_index].token_type {
            TokenType::OpenBracket => {
                if bracket_depth == 0 && is_curr_array_dynamic.is_none() {
                    open_index = curr_index + 1;
                    is_curr_array_dynamic = Some(false);
                }
                bracket_depth += 1;
            }

            TokenType::CloseBracket => {
                if bracket_depth == 1 && !is_curr_array_dynamic.unwrap_or(true) {
                    let tokens_inside = &tokens[open_index..curr_index];
                    on_creation(tokens_inside, false)?;
                    is_curr_array_dynamic = None;
                }
                bracket_depth -= 1;
            }

            TokenType::OpenDynamicArray => {
                if bracket_depth == 0 && is_curr_array_dynamic.is_none() {
                    open_index = curr_index + 1;
                    is_curr_array_dynamic = Some(true);
                }
                arrow_depth += 1;
            }

            TokenType::CloseDynamicArray => {
                if arrow_depth == 1 && is_curr_array_dynamic.unwrap_or(false) {
                    let tokens_inside = &tokens[open_index..curr_index];
                    on_creation(tokens_inside, true)?;
                    is_curr_array_dynamic = None;
                }
                arrow_depth -= 1;
            }

            ref tt if terminator_types.contains(tt) => {
                last_token = token;
                break;
            }

            _ => {
                if bracket_depth > 0 || arrow_depth > 0 {
                    token_sequence.push(token);
                }
            }
        }
        last_token = token;
        curr_index += 1;
    }

    if bracket_depth != 0 {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: last_token.line,
            character: last_token.start,
            source: String::from("Bracket has not been closed for array type"),
        });
    } else if arrow_depth != 0 {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: last_token.line,
            character: last_token.start,
            source: String::from("Arrow has not been closed for dynamic array type"),
        });
    }

    Ok(curr_index)
}

#[macro_export]
macro_rules! define_statement_checkup {
    ( $index: expr, $tokens: expr, $curr: expr, $target: expr, $message_for_nothing: expr, $message_for_wrong: expr) => {{
        if $index >= $tokens.len() {
            let first_token: &Token = $tokens.first().unwrap();
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                character: first_token.start,
                line: first_token.line,
                source: $message_for_nothing,
            });
        }
        $curr = &$tokens[$index];
        if $curr.token_type != $target {
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                character: $curr.start,
                line: $curr.line,
                source: $message_for_wrong($curr),
            });
        }
    }};
}
