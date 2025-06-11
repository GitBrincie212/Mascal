use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::token::{Token, TokenType};

pub fn parse_variable_decl<'a>(
    tokens: &'a Vec<Token<'a>>
) -> Result<MascalVariableInitialDeclaration, MascalError> {
    let name: String;
    let mut is_constant: bool = false;
    let mut is_nullable: bool = false;
    let mut dimensions: Vec<MascalExpression> = Vec::new();
    let mut is_dynamic_array: Vec<bool> = Vec::new();
    let mut initial_value: Option<MascalExpression> = Some(MascalExpression::LiteralExpression(MascalLiteral::NULL));
    let mut curr_index: usize = 0;

    if curr_index < tokens.len() && tokens[curr_index].token_type == TokenType::Const {
        is_constant = true;
        curr_index += 1;
    }

    if curr_index < tokens.len() && tokens[curr_index].token_type == TokenType::Identifier {
        name = tokens[curr_index].value.to_string();
        curr_index += 1;
    } else {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: tokens[0].line,
            character: tokens[0].start,
            source: String::from("Expected a variable name"),
        });
    }

    let mut bracket_depth: usize = 0;
    let mut arrow_depth: usize = 0;
    let mut last_token: &Token = &tokens[curr_index];
    let first_token: &Token = &tokens[curr_index];
    while curr_index < tokens.len() && first_token.token_type == TokenType::OpenBracket {
        let mut token_sequence: Vec<&Token> = Vec::new();
        let token: &Token = &tokens[curr_index];
        match tokens[curr_index].token_type {
            TokenType::OpenBracket => {
                bracket_depth += 1;
            }
            
            TokenType::CloseBracket => {
                // TODO: Parse expression
                dimensions.push(MascalExpression::LiteralExpression(MascalLiteral::NULL));
                is_dynamic_array.push(false);
                bracket_depth -= 1;
            }

            TokenType::OpenArrow => {
                arrow_depth += 1;
            }

            TokenType::CloseArrow => {
                // TODO: Parse expression
                dimensions.push(MascalExpression::LiteralExpression(MascalLiteral::NULL));
                is_dynamic_array.push(true);
                arrow_depth -= 1;
            }

            TokenType::Semicolon | TokenType::VariableInitializer | TokenType::QuestionMark => {
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
            source: String::from("Bracket has not been closed for array type")
        })
    } else if arrow_depth != 0 {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: last_token.line,
            character: last_token.start,
            source: String::from("Arrow has not been closed for dynamic array type")
        })
    }

    if tokens[curr_index].token_type == TokenType::QuestionMark {
        is_nullable = true;
        curr_index += 1;
    }

    if tokens[curr_index].token_type == TokenType::VariableInitializer {
        // TODO: Parse the expression provided
        // let initial_value_tokens: &[&Token] = &tokens[curr_index + 1..];
        initial_value = Some(MascalExpression::LiteralExpression(MascalLiteral::NULL));
        curr_index += tokens.len() - curr_index - 1;
    }

    if tokens[curr_index].token_type != TokenType::Semicolon {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: tokens[curr_index].line,
            character: tokens[curr_index].start,
            source: String::from("Unexpected characters found during parsing of variable initialization")
        });
    }

    Ok(MascalVariableInitialDeclaration {
        name,
        is_constant,
        is_nullable,
        dimensions,
        is_dynamic_array,
        initial_value,
    })
}