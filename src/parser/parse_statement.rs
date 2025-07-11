use crate::define_statement_checkup;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::statements::{MascalConditionalBranch, MascalStatement};
use crate::defs::token::{Token, TokenType};
use crate::parser::TokenSequence;
use crate::parser::parse_expression::parse_expression;
use crate::parser::utils::{extract_braced_block_from_tokens, run_per_statement};

fn parse_branch(
    token_sequence: &[Token],
    is_else: bool,
) -> Result<MascalConditionalBranch, MascalError> {
    let mut open_brace_index: usize = 0;
    let condition_expression: Option<MascalExpression> = if !is_else {
        open_brace_index = 1;
        let mut condition_tokens: Vec<Token> = Vec::new();
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
            source: String::from("Expected a opening brace for a conditional branch"),
        });
    }

    let statements_parser: TokenSequence = extract_braced_block_from_tokens(
        &token_sequence[open_brace_index..],
        "Conditional",
        &[],
        &[],
    )?;

    let mut statements: Vec<MascalStatement> = Vec::new();

    let final_toks: Vec<Token> = run_per_statement(&statements_parser, |token_sequence| {
        let stmt = parse_statement(token_sequence)?;
        statements.push(stmt);
        Ok(())
    })?;

    if !final_toks.is_empty() {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: final_toks[0].line,
            character: final_toks[0].start,
            source: String::from(
                "Unexpected characters found inside conditional statement, perhaps forgot a semicolon?",
            ),
        });
    }

    Ok(MascalConditionalBranch {
        condition: condition_expression,
        statements: statements.into_boxed_slice(),
    })
}

fn parse_conditional_statement(token_sequence: &[Token]) -> Result<MascalStatement, MascalError> {
    let mut has_entered_else: bool = false;
    let mut branches: Vec<MascalConditionalBranch> = Vec::new();
    for (index, token) in token_sequence.iter().enumerate() {
        match token.token_type {
            TokenType::If => {
                branches.push(parse_branch(&token_sequence[index + 1..], false)?);
            }

            TokenType::ElseIf => {
                if has_entered_else {
                    return Err(MascalError {
                        error_type: MascalErrorType::ParserError,
                        character: token.start,
                        line: token.line,
                        source: String::from("Cannot supply an ELIF condition after an ELSE condition without opening a new IF condition"),
                    })
                }
                branches.push(parse_branch(&token_sequence[index + 1..], false)?);
            }

            TokenType::Else => {
                branches.push(parse_branch(&token_sequence[index + 1..], true)?);
                has_entered_else = true;
            }

            _ => {}
        }
    }

    Ok(MascalStatement::ConditionalStatement(branches))
}

fn locate_semicolon(tokens: &[Token]) -> Result<usize, MascalError> {
    for (index, token) in tokens.iter().enumerate() {
        if token.token_type == TokenType::Semicolon {
            return Ok(index);
        }
    }
    Err(MascalError {
        error_type: MascalErrorType::ParserError,
        line: tokens.last().unwrap().line,
        character: tokens.last().unwrap().start,
        source: String::from("Expected an ending semicolon to finish the statement"),
    })
}

fn parse_throw_statement(tokens: &[Token]) -> Result<MascalStatement, MascalError> {
    let mut index: usize = 0;
    let mut curr: &Token;
    define_statement_checkup!(
        index,
        tokens,
        curr,
        TokenType::Identifier,
        String::from("Expected a error type to throw but got nothing"),
        |curr: &Token| { format!("Expected a error type to throw but got {:?}", curr.value) }
    );
    let error_type: MascalErrorType = match curr.value {
        "TypeError" => MascalErrorType::TypeError,
        "RuntimeError" => MascalErrorType::RuntimeError,
        "OverflowError" => MascalErrorType::OverflowError,
        "UndefinedOperationError" => MascalErrorType::UndefinedOperation,
        "IndexError" => MascalErrorType::IndexError,
        "InputError" => MascalErrorType::InputError,
        "ArgumentError" => MascalErrorType::ArgumentError,
        "ValueError" => MascalErrorType::ValueError,
        _ => {
            return Err(MascalError {
                error_type: MascalErrorType::UndefinedErrorType,
                character: 0,
                line: 0,
                source: String::from(
                    "Use of an undefined usable error type in the throw statement (perhaps a typo?)",
                ),
            });
        }
    };
    index += 1;
    define_statement_checkup!(
        index,
        tokens,
        curr,
        TokenType::Colon,
        String::from("Expected a colon for the throw statement but got nothing"),
        |curr: &Token| {
            format!(
                "Expected a colon for the throw statement but got {:?}",
                curr.value
            )
        }
    );
    index += 1;
    define_statement_checkup!(
        index,
        tokens,
        curr,
        TokenType::StringLiteral,
        String::from("Expected a message for the throw statement but got nothing"),
        |curr: &Token| {
            format!(
                "Expected a message for the throw statement but got {:?}",
                curr.value
            )
        }
    );
    let message: String = curr.value.to_string();
    index += 1;
    if index < tokens.len() {
        curr = &tokens[index];
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: curr.line,
            character: curr.start,
            source: String::from(
                "Unexpected tokens found during the parsing of the throw statement",
            ),
        });
    }

    Ok(MascalStatement::Throw {
        error_type,
        message,
    })
}

macro_rules! parse_expression_in_statement {
    ($tokens: expr, $index: expr, $terminator_tokens: expr) => {{
        let mut expression_tokens: Vec<Token> = Vec::new();
        for token in $tokens[$index..].iter() {
            if $terminator_tokens.contains(&token.token_type) {
                break;
            }
            expression_tokens.push(token.clone());
            $index += 1;
        }
        parse_expression(&expression_tokens)?
    }};
}

fn parse_for_loop_statement(tokens: &[Token]) -> Result<MascalStatement, MascalError> {
    let mut index: usize = 0;
    let mut curr: &Token;
    define_statement_checkup!(
        index,
        tokens,
        curr,
        TokenType::Identifier,
        String::from("Expected a variable identifier to use but got nothing"),
        |curr: &Token| {
            format!(
                "Expected a variable identifier to use but got {:?}",
                curr.value
            )
        }
    );
    let variable_name: String = curr.value.to_string();
    index += 1;
    define_statement_checkup!(
        index,
        tokens,
        curr,
        TokenType::From,
        String::from("Expected FROM but got nothing"),
        |curr: &Token| { format!("Expected FROM but got {:?}", curr.value) }
    );
    index += 1;

    let from: MascalExpression = parse_expression_in_statement!(tokens, index, [TokenType::To]);

    define_statement_checkup!(
        index,
        tokens,
        curr,
        TokenType::To,
        String::from("Expected TO but got nothing"),
        |curr: &Token| { format!("Expected TO but got {:?}", curr.value) }
    );
    index += 1;

    let to: MascalExpression =
        parse_expression_in_statement!(tokens, index, [TokenType::WithStep, TokenType::OpenBrace]);

    let with_step: MascalExpression = if tokens[index].token_type == TokenType::WithStep {
        index += 1;

        parse_expression_in_statement!(tokens, index, [TokenType::OpenBrace])
    } else {
        MascalExpression::Literal(MascalLiteral::Integer(IntegerNum::I8(1)))
    };

    if tokens[index].token_type != TokenType::OpenBrace {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: tokens[index].line,
            character: tokens[index].start,
            source: String::from("Expected a opening brace for a for loop block"),
        });
    }

    let statements_parser: TokenSequence =
        extract_braced_block_from_tokens(&tokens[index..], "For loop", &[], &[])?;

    let mut statements: Vec<MascalStatement> = Vec::new();

    let final_toks: Vec<Token> = run_per_statement(&statements_parser, |token_sequence| {
        let stmt = parse_statement(token_sequence)?;
        statements.push(stmt);
        Ok(())
    })?;

    if !final_toks.is_empty() {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: final_toks[0].line,
            character: final_toks[0].start,
            source: String::from(
                "Unexpected characters found inside for loop statement, perhaps forgot a semicolon?",
            ),
        });
    }

    Ok(MascalStatement::For {
        variable: variable_name,
        from,
        to,
        step: with_step,
        statements,
    })
}

fn parse_while_loop_statement(tokens: &[Token]) -> Result<MascalStatement, MascalError> {
    let mut index: usize = 0;

    let condition_expression: MascalExpression =
        parse_expression_in_statement!(tokens, index, [TokenType::OpenBrace]);

    if tokens[index].token_type != TokenType::OpenBrace {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: tokens[index].line,
            character: tokens[index].start,
            source: String::from("Expected a opening brace for a while loop block"),
        });
    }

    let statements_parser: TokenSequence =
        extract_braced_block_from_tokens(&tokens[index..], "While loop", &[], &[])?;

    let mut statements: Vec<MascalStatement> = Vec::new();

    let final_toks: Vec<Token> = run_per_statement(&statements_parser, |token_sequence| {
        let stmt = parse_statement(token_sequence)?;
        statements.push(stmt);
        Ok(())
    })?;

    if !final_toks.is_empty() {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: final_toks[0].line,
            character: final_toks[0].start,
            source: String::from(
                "Unexpected characters found inside while loop statement, perhaps forgot a semicolon?",
            ),
        });
    }

    Ok(MascalStatement::While(MascalConditionalBranch {
        condition: Some(condition_expression),
        statements: statements.into_boxed_slice(),
    }))
}

pub fn parse_statement(token_sequence: &Vec<Token>) -> Result<MascalStatement, MascalError> {
    let first_token: &Token = token_sequence.first().unwrap();

    match first_token.token_type {
        TokenType::Throw => {
            let index: usize = locate_semicolon(token_sequence)?;
            let throw_statement: MascalStatement =
                parse_throw_statement(&token_sequence[1..index])?;
            Ok(throw_statement)
        }

        TokenType::If => {
            let if_statement: MascalStatement = parse_conditional_statement(token_sequence)?;
            Ok(if_statement)
        }

        TokenType::For => {
            let for_statement: MascalStatement = parse_for_loop_statement(&token_sequence[1..])?;
            Ok(for_statement)
        }

        TokenType::While => {
            let for_statement: MascalStatement = parse_while_loop_statement(&token_sequence[1..])?;
            Ok(for_statement)
        }

        TokenType::ElseIf => Err(MascalError {
            error_type: MascalErrorType::ParserError,
            character: first_token.start,
            line: first_token.line,
            source: String::from("Expected an IF condition before this ELIF condition"),
        }),

        TokenType::Else => Err(MascalError {
            error_type: MascalErrorType::ParserError,
            character: first_token.start,
            line: first_token.line,
            source: String::from("Expected an IF condition before this ELSE condition"),
        }),

        TokenType::Break => {
            let index: usize = locate_semicolon(token_sequence)?;
            if token_sequence[..index].len() > 1 {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    character: 0,
                    line: 0,
                    source: String::from(
                        "Expected nothing else to be supplied but contents inside the break statement",
                    ),
                });
            }
            Ok(MascalStatement::Break)
        }

        TokenType::Continue => {
            let index: usize = locate_semicolon(token_sequence)?;
            if token_sequence[..index].len() > 1 {
                return Err(MascalError {
                    error_type: MascalErrorType::ParserError,
                    character: 0,
                    line: 0,
                    source: String::from(
                        "Expected nothing else to be supplied but contents inside the continue statement",
                    ),
                });
            }
            Ok(MascalStatement::Continue)
        }

        _ => {
            let index: usize = locate_semicolon(token_sequence)?;
            let mut assignment_index: Option<usize> = None;
            let trunucated_token_seq: Vec<Token> = token_sequence[..index].to_vec();
            for (index, tok) in trunucated_token_seq.iter().enumerate() {
                if tok.token_type == TokenType::VariableInitializer {
                    if assignment_index.is_some() {
                        return Err(MascalError {
                            error_type: MascalErrorType::ParserError,
                            line: tok.line,
                            character: tok.start,
                            source: String::from("Cannot use more than one variable initializer per statement")
                        })
                    }
                    assignment_index = Some(index);
                }
            }
            if let Some(unwrapped_assign_index) = assignment_index {
                let target_assigne: MascalExpression =
                    parse_expression(&trunucated_token_seq[..unwrapped_assign_index].to_vec())?;
                match target_assigne {
                    MascalExpression::Indexing { .. } | MascalExpression::Symbolic(_) => {}

                    _ => {
                        return Err(MascalError {
                            error_type: MascalErrorType::ParserError,
                            line: trunucated_token_seq.first().unwrap().line,
                            character: trunucated_token_seq.first().unwrap().line,
                            source: String::from(
                                "Expected either a index-based variable modification or variable assignment but got something else",
                            ),
                        });
                    }
                }
                let value_assigned: MascalExpression =
                    parse_expression(&trunucated_token_seq[unwrapped_assign_index + 1..].to_vec())?;
                return Ok(MascalStatement::Declaration {
                    variable: target_assigne,
                    value: value_assigned,
                });
            }
            let expression_statement: MascalExpression = parse_expression(&trunucated_token_seq)?;
            Ok(MascalStatement::ExpressionStatement(expression_statement))
        }
    }
}
