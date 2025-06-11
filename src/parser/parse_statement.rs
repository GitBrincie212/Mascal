use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::statements::{MascalConditionalBranch, MascalDeclarationStatement, MascalStatement};
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::parse_expression;
use crate::parser::TokenSequence;
use crate::parser::utils::{extract_braced_block_from_tokens, run_per_statement};

fn parse_branch(token_sequence: &[Token], is_else: bool) -> Result<MascalConditionalBranch, MascalError> {
    let mut open_brace_index: usize = 1;
    let condition_expression: Option<MascalExpression> = if !is_else {
        let mut condition_tokens: Vec<Token> =  Vec::new();
        for (index, token) in token_sequence.iter().enumerate() {
            if token.token_type == TokenType::OpenBrace {
                open_brace_index = index;
                break;
            }
            condition_tokens.push(token.clone());
        }
        Some(parse_expression(&condition_tokens)?)
    } else {
        None
    };

    if token_sequence[open_brace_index].token_type != TokenType::OpenBrace {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: token_sequence[open_brace_index].line,
            character: token_sequence[open_brace_index].start,
            source: String::from("Expected a opening brace for a conditional branch")
        })
    }

    let statements_parser: TokenSequence = extract_braced_block_from_tokens(
        &token_sequence[open_brace_index..],
        "Conditional",
        &[],
        &[],
    )?;

    let mut statements: Vec<MascalStatement> = Vec::new();

    run_per_statement(&statements_parser, |token_sequence| {
        let stmt = parse_statement(token_sequence)?;
        statements.push(stmt);
        Ok(())
    })?;

    Ok(MascalConditionalBranch {
        condition: condition_expression,
        statements,
    })
}

fn parse_conditional_statement(token_sequence: &[Token]) -> Result<MascalStatement, MascalError> {
    let mut branches: Vec<MascalConditionalBranch> = Vec::new();
    for (index, token) in token_sequence.iter().enumerate() {
        match token.token_type {
            TokenType::If => {
                branches.push(parse_branch(&token_sequence[index..], false)?);
            }

            TokenType::ElseIf => {
                branches.push(parse_branch(&token_sequence[index..], false)?);
            }

            TokenType::Else => {
                branches.push(parse_branch(&token_sequence[index..], true)?);
            }

            _ => {}
        }
    }

    Ok(MascalStatement::ConditionalStatement(branches))
}

pub fn parse_statement(token_sequence: &Vec<Token>) -> Result<MascalStatement, MascalError> {
    let last_token = token_sequence.last().unwrap();
    let first_token = token_sequence.first().unwrap();
    if last_token.token_type != TokenType::Semicolon {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: last_token.line,
            character: last_token.start,
            source: String::from("Expected an ending semicolon to finish the statement")
        })
    }

    match first_token.token_type {
        TokenType::If => {
            let if_statement: MascalStatement = parse_conditional_statement(&token_sequence)?;
            Ok(if_statement)
        }

        TokenType::ElseIf => {
            Err(MascalError {
                error_type: MascalErrorType::LexerError,
                character: first_token.start,
                line: first_token.line,
                source: String::from("Expected an IF condition before this ELIF condition"),
            })
        }

        TokenType::Else => {
            Err(MascalError {
                error_type: MascalErrorType::LexerError,
                character: first_token.start,
                line: first_token.line,
                source: String::from("Expected an IF statement before this ELSE condition"),
            })
        }

        TokenType::Identifier if token_sequence.len() >= 3
            && token_sequence[1].token_type == TokenType::VariableInitializer => {
            let name: String = first_token.value.to_string();
            Ok(MascalStatement::Declaration(MascalDeclarationStatement {
                variable: name,
                value: parse_expression(&token_sequence[2..].to_vec())?
            }))
        }

        _ => {
            let expression_statement: MascalExpression = parse_expression(token_sequence)?;
            Ok(MascalStatement::Expression(expression_statement))
        }
    }
}