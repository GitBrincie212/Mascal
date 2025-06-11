use std::collections::HashSet;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::token::{Token, TokenType, SCOPABLE_TOKEN_TYPES};
use crate::parser::Parser;

pub fn detect_open_brace(preceding_token: &Token, token: &Option<&Token>) -> Result<(), MascalError> {
    match token {
        None => {
            Err(MascalError {
                error_type: MascalErrorType::ParserError,
                line: preceding_token.line,
                character: preceding_token.start,
                source: String::from("Expected a opening brace but found nothing")
            })
        }

        Some(token) => {
            if token.token_type != TokenType::OpenBrace {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    line: token.line,
                    character: token.start,
                    source: format!("Expected a opening brace but found \"{}\"", token.value)
                });
            }
            
            Ok(())
        }
    }
}

pub fn token_anomalies_evaluation(
    tokens: &[Token], index: usize, depth_counter: usize, nonzero_depth_counter_msg: String
) -> Result<(), MascalError> {
    if depth_counter != 0 {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            character: tokens[index].start,
            line: tokens[index].line,
            source: nonzero_depth_counter_msg
        })
    } else if tokens[index - 1].token_type != TokenType::CloseBrace {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            character: tokens[index].start,
            line: tokens[index].line,
            source: String::from("Expected right after a closing brace but found a intermediate token")
        });
    }

    Ok(())
}

pub fn extract_braced_block<'a>(
    parser: Parser<'a>,
    block_name: &'static str,
    allow_nested: &[TokenType],
    require_inside: &[TokenType],
) -> Result<Parser<'a>, MascalError> {
    if !parser.is_of(TokenType::OpenBrace, 0) {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: parser.acquire_token(0).line,
            character: parser.acquire_token(0).start,
            source: format!("{block_name} block must start with '{{'")
        });
    }

    let mut depth: usize = 0;
    let mut found_required: HashSet<TokenType> = HashSet::new();

    for (i, token) in parser.tokens.iter().enumerate() {
        if !SCOPABLE_TOKEN_TYPES.contains(&token.token_type)
            && token.token_type != TokenType::CloseBrace
            && token.token_type != TokenType::OpenBrace {
            continue;
        }
        match token.token_type {
            TokenType::OpenBrace => depth += 1,
            TokenType::CloseBrace => {
                depth -= 1;
                if depth == 0 {
                    if !require_inside.iter().all(|r| found_required.contains(r)) {
                        let missing: Vec<_> = require_inside
                            .iter()
                            .filter(|r| !found_required.contains(r))
                            .map(|r| format!("{:?}", r))
                            .collect();
                        return Err(MascalError {
                            error_type: MascalErrorType::ParserError,
                            line: token.line,
                            character: token.start,
                            source: format!("Missing required block(s): {}", missing.join(", "))
                        });
                    }
                    return Ok(parser.subsection_range(1..i));
                }
            }

            ref tt => {
                if parser.tokens[i + 1].token_type != TokenType::OpenBrace {
                    continue;
                }
                let is_required = require_inside.contains(tt);
                let is_optional_nest = allow_nested.contains(tt);

                if is_required {
                    found_required.insert(tt.clone());
                } else if depth == 1 && !is_optional_nest {
                    dbg!(token.line);
                    return Err(MascalError {
                        error_type: MascalErrorType::ParserError,
                        line: token.line,
                        character: token.start,
                        source: format!("Token '{:?}' is not allowed in nested blocks of {block_name}", tt),
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
        character: parser.last_token().start,
        line: parser.last_token().line,
        source: format!("{block_name} block not properly closed")
    })
}

pub fn extract_braced_block_from_tokens<'a>(
    tokens: &'a [Token<'a>],
    block_name: &'static str,
    allow_nested: &[TokenType],
    require_inside: &[TokenType],
) -> Result<Parser<'a>, MascalError> {
    extract_braced_block(Parser::new(tokens.to_vec()), block_name, allow_nested, require_inside)
}

pub fn locate_block<'a>(
    parser: Parser<'a>, token_type: TokenType, block_name: &'static str,
    allow_nested: &[TokenType], require_inside: &[TokenType]
) -> Result<Option<Parser<'a>>, MascalError> {
    for (index, token) in parser.tokens.iter().enumerate() {
        if token.token_type != token_type { continue }
        let subset_tokens = extract_braced_block(
            parser.subsection_from(index + 1..),
            block_name,
            allow_nested,
            require_inside,
        )?;

        return Ok(Some(subset_tokens));
    }

    Ok(None)
}

pub fn locate_block_from<'a>(
    tokens: &'a [Token<'a>], token_type: TokenType, block_name: &'static str,
    allow_nested: &[TokenType], require_inside: &[TokenType]
) -> Result<Option<Parser<'a>>, MascalError> {
    locate_block(Parser::new(tokens.to_vec()), token_type, block_name, allow_nested, require_inside)
}

pub fn run_per_statement<'a, F>(
    parser: &'a Parser, mut func: F
) -> Result<Vec<Token<'a>>, MascalError> where F: (FnMut(&Vec<Token<'a>>) -> Result<(), MascalError>) {
    let mut token_sequence: Vec<Token> = Vec::new();
    let mut depth_counter: usize = 0;
    for token in parser.tokens.iter() {
        if token.token_type != TokenType::Comment {
            token_sequence.push(token.clone())
        }
        match token.token_type {
            TokenType::OpenBrace => {
                depth_counter += 1;
            }
            TokenType::CloseBrace => {
                depth_counter -= 1;
            }
            TokenType::Semicolon => {
                if depth_counter > 0 {
                    continue;
                }
                func(&token_sequence)?;
                token_sequence.clear();
                continue;
            }
            _ => {}
        }
    }
    
    Ok(token_sequence)
}