use crate::defs::blocks::VariableBlock;
use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::InfinityControl;
use crate::defs::token::{Token, TokenType};
use crate::parser::parse_variable_decl::parse_variable_decl;
use crate::parser::TokenSequence;
use crate::parser::utils::{extract_braced_block, locate_block_from, run_per_statement};

pub fn parse_variable_type_block<'a>(
    tokens: &Vec<Token>, token_type: TokenType, block_name: &'static str
) -> Result<Vec<MascalVariableInitialDeclaration>, MascalError> {
    let var_type_parser: TokenSequence<'_> = locate_block_from(
        tokens, token_type, block_name, &[], &[]
    )?.unwrap_or(TokenSequence::new(vec![]));

    let mut variable_initialization: Vec<MascalVariableInitialDeclaration> = Vec::new();
    let final_token_sequence: Vec<Token> = run_per_statement(&var_type_parser, |token_sequence| {
        if token_sequence.is_empty() {
            return Ok(());
        } else if token_sequence.len() == 1 {
            variable_initialization.push(MascalVariableInitialDeclaration {
                name: token_sequence.get(0).unwrap().value.to_string(),
                initial_value: None,
                is_constant: false,
                is_dynamic_array: Vec::new(),
                dimensions: Vec::new(),
                is_nullable: false,
                infinity_control: InfinityControl::DisallowInfinity
            });
            return Ok(());
        }
        let variable_decl: MascalVariableInitialDeclaration = parse_variable_decl(token_sequence)?;
        variable_initialization.push(variable_decl);
        return Ok(());
    })?;

    if !final_token_sequence.is_empty() {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: final_token_sequence.first().unwrap().line,
            character: final_token_sequence.first().unwrap().start,
            source: String::from("Unexpected characters for variable declaration, perhaps it hasn't been closed with a semicolon?")
        });
    }

    Ok(variable_initialization)
}

pub fn parse_variable_block(token_sequence: &TokenSequence) -> Result<VariableBlock, MascalError> {
    for (index, token) in (&token_sequence.tokens).iter().enumerate() {
        if token.token_type != TokenType::Variables { continue }
        let subset_parser = extract_braced_block(
            token_sequence.subsection_from(index + 1..),
            "VARIABLES",
            &[TokenType::Integer, TokenType::Float, TokenType::String, TokenType::Boolean, TokenType::Dynamic, TokenType::Type],
            &[],
        )?;

        let integer_tokens: Vec<MascalVariableInitialDeclaration> = parse_variable_type_block(
            &subset_parser.tokens, TokenType::Integer, "INTEGER"
        )?;

        let float_tokens: Vec<MascalVariableInitialDeclaration> = parse_variable_type_block(
            &subset_parser.tokens, TokenType::Float, "FLOAT"
        )?;

        let string_tokens: Vec<MascalVariableInitialDeclaration> = parse_variable_type_block(
            &subset_parser.tokens, TokenType::String, "STRING"
        )?;

        let boolean_tokens: Vec<MascalVariableInitialDeclaration> = parse_variable_type_block(
            &subset_parser.tokens, TokenType::Boolean, "BOOLEAN"
        )?;

        let dynamic_tokens: Vec<MascalVariableInitialDeclaration> = parse_variable_type_block(
            &subset_parser.tokens, TokenType::Dynamic, "DYNAMIC"
        )?;

        let type_tokens: Vec<MascalVariableInitialDeclaration> = parse_variable_type_block(
            &subset_parser.tokens, TokenType::Type, "TYPE"
        )?;

        return Ok(VariableBlock::new(
            integer_tokens,
            float_tokens,
            boolean_tokens,
            string_tokens,
            dynamic_tokens,
            type_tokens,
        ));
    }

    Ok(VariableBlock::new(
        vec![],
        vec![],
        vec![],
        vec![], 
        vec![],
        vec![],
    ))
}