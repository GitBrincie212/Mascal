mod variable_check_stage;
mod check_parameters_declaration;

use std::collections::HashSet;
use crate::ast::AbstractSyntaxTree;
use crate::defs::blocks::{ScopedBlocks, VariableBlock};
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::semantic_analysis::check_parameters_declaration::check_for_param_declaration;

pub fn conduct_semantic_analysis(abstract_syntax_tree: AbstractSyntaxTree) -> Result<AbstractSyntaxTree, MascalError> {
    let mut has_program: bool = false;
    for block in &abstract_syntax_tree.blocks {
        let varblock: &VariableBlock = &match block {
            ScopedBlocks::PROGRAM(exec_block) => {
                has_program = true;
                exec_block
            },
            ScopedBlocks::FUNCTION {execution_block, parameters, .. } => {
                check_for_param_declaration(execution_block, parameters)?;
                execution_block
            }
        }.variables;
        let mut defined_var_names: HashSet<String> = HashSet::new();
        defined_var_names = variable_check_stage::check_per_variable(&varblock.integers, defined_var_names, true)?;
        defined_var_names = variable_check_stage::check_per_variable(&varblock.floats, defined_var_names, true)?;
        defined_var_names = variable_check_stage::check_per_variable(&varblock.strings, defined_var_names, false)?;
        defined_var_names = variable_check_stage::check_per_variable(&varblock.booleans, defined_var_names, false)?;
        defined_var_names = variable_check_stage::check_per_variable(&varblock.dynamics, defined_var_names, true)?;
        variable_check_stage::check_per_variable(&varblock.types, defined_var_names, false)?;
    }
    if !has_program {
        return Err(MascalError {
            error_type: MascalErrorType::ParserError,
            character: 0,
            line: 0,
            source: String::from("Definition of a program is required in order to execute this file")
        })
    }
    Ok(abstract_syntax_tree)
}