use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_expression::parse_expression;
use crate::parser::utils::parse_array_type;

pub fn parse_variable_decl<'a>(
    tokens: &'a Vec<Token<'a>>
) -> Result<MascalVariableInitialDeclaration, MascalError> {
    let name: String;
    let mut is_constant: bool = false;
    let mut is_nullable: bool = false;
    let mut dimensions: Vec<MascalExpression> = Vec::new();
    let mut is_dynamic_array: Vec<bool> = Vec::new();
    let mut initial_value: Option<MascalExpression> = None;
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

    curr_index = parse_array_type(tokens, curr_index, |token_sequence, is_dynamic | {
        if is_dynamic {
            if token_sequence.is_empty() {
                dimensions.push(MascalExpression::LiteralExpression(MascalLiteral::Integer(IntegerNum::I8(1))));
                is_dynamic_array.push(is_dynamic);
                return Ok(())
            }
            dimensions.push(parse_expression(&token_sequence.to_vec())?);
            is_dynamic_array.push(true);
            return Ok(());
        }
        if token_sequence.is_empty() {
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                line: 0,
                character: 0,
                source: String::from("Static arrays cannot be omitted and must have a specified size")
            })
        }
        dimensions.push(parse_expression(&token_sequence.to_vec())?);
        is_dynamic_array.push(false);
        Ok(())
    }, vec![TokenType::Semicolon, TokenType::VariableInitializer, TokenType::QuestionMark])?;

    if tokens[curr_index].token_type == TokenType::QuestionMark {
        is_nullable = true;
        curr_index += 1;
    }

    if tokens[curr_index].token_type == TokenType::VariableInitializer {
        let initial_value_tokens: &[Token] = &tokens[curr_index + 1..&tokens.len() - 1];
        initial_value = Some(parse_expression(&initial_value_tokens.to_vec())?);
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
        dimensions: dimensions.into_boxed_slice(),
        is_dynamic_array: is_dynamic_array.into_boxed_slice(),
        initial_value,
    })
}