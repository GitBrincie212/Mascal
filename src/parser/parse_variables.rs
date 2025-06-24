use std::rc::Rc;
use crate::defs::blocks::VariableBlock;
use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::token::{Token, TokenType};
use crate::parser::TokenSequence;
use crate::parser::parse_variable_decl::parse_variable_decl;
use crate::parser::utils::{extract_braced_block, run_per_statement};

pub fn parse_variable_type_block(
    subsection_tokens: TokenSequence,
    var_inits: &mut Vec<MascalVariableInitialDeclaration>,
) -> Result<(), MascalError> {
    let final_token_sequence: Vec<Token> =
        run_per_statement(&subsection_tokens, |token_sequence| {
            if token_sequence.is_empty() {
                return Ok(());
            } else if token_sequence.len() == 1 {
                var_inits.push(MascalVariableInitialDeclaration {
                    name: Rc::from(token_sequence[0].value),
                    initial_value: None,
                    is_constant: false,
                    is_dynamic_array: Box::new([]),
                    dimensions: Box::new([]),
                    is_nullable: false,
                });
                return Ok(());
            }
            let variable_decl: MascalVariableInitialDeclaration =
                parse_variable_decl(token_sequence)?;
            var_inits.push(variable_decl);
            Ok(())
        })?;

    if !final_token_sequence.is_empty() {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            line: final_token_sequence.first().unwrap().line,
            character: final_token_sequence.first().unwrap().start,
            source: String::from(
                "Unexpected characters for variable declaration, perhaps it hasn't been closed with a semicolon?",
            ),
        });
    }

    Ok(())
}

pub fn parse_variable_block(token_sequence: &TokenSequence) -> Result<VariableBlock, MascalError> {
    for (index, token) in token_sequence.tokens.iter().enumerate() {
        if token.token_type != TokenType::Variables {
            continue;
        }
        let subset_token_sequence: TokenSequence = extract_braced_block(
            token_sequence.subsection_from(index + 1..),
            "VARIABLES",
            &[
                TokenType::Integer,
                TokenType::Float,
                TokenType::String,
                TokenType::Boolean,
                TokenType::Dynamic,
                TokenType::Type,
            ],
            &[],
        )?;

        let mut pos: usize = index;

        let mut integers: Vec<MascalVariableInitialDeclaration> = vec![];
        let mut floats: Vec<MascalVariableInitialDeclaration> = vec![];
        let mut strings: Vec<MascalVariableInitialDeclaration> = vec![];
        let mut booleans: Vec<MascalVariableInitialDeclaration> = vec![];
        let mut dynamics: Vec<MascalVariableInitialDeclaration> = vec![];
        let mut types: Vec<MascalVariableInitialDeclaration> = vec![];

        let mut vartype_blocks: Vec<(TokenType, &str, &mut Vec<MascalVariableInitialDeclaration>)> = vec![
            (TokenType::Integer, "INTEGER", &mut integers),
            (TokenType::Float, "FLOAT", &mut floats),
            (TokenType::String, "STRING", &mut strings),
            (TokenType::Boolean, "BOOLEAN", &mut booleans),
            (TokenType::Dynamic, "DYNAMIC", &mut dynamics),
            (TokenType::Type, "TYPE", &mut types),
        ];

        let mut already_assigned: Vec<usize> = Vec::with_capacity(vartype_blocks.len());
        while pos < subset_token_sequence.tokens.len() {
            let curr: &Token = &subset_token_sequence.tokens[pos];
            let index: Option<usize> = vartype_blocks
                .iter()
                .position(|(t, _, _)| t == &curr.token_type);

            if let Some(vartype_block_index) = index {
                pos += 1;
                let extracted_vartype_block: TokenSequence = extract_braced_block(
                    subset_token_sequence.subsection_from(pos..),
                    vartype_blocks[vartype_block_index].1,
                    &[],
                    &[],
                )?;
                if already_assigned.contains(&vartype_block_index) {
                    return Err(MascalError {
                        error_type: MascalErrorType::ParserError,
                        line: curr.line,
                        character: curr.start,
                        source: format!(
                            "Found redefinition of the same variable type block {} declared before",
                            curr.value
                        ),
                    });
                }
                pos += extracted_vartype_block.tokens.len() + 2;
                parse_variable_type_block(
                    extracted_vartype_block,
                    vartype_blocks[vartype_block_index].2,
                )?;
                already_assigned.push(vartype_block_index);
                continue;
            }
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                line: curr.line,
                character: curr.start,
                source: String::from(
                    "Expected to define a variable type inside the variable block but got an unknown expression",
                ),
            });
        }

        return Ok(VariableBlock::new(
            integers, floats, booleans, strings, dynamics, types,
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
