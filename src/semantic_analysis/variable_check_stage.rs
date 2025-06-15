use std::collections::HashSet;
use crate::ast::AbstractSyntaxTree;
use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::blocks::{ScopedBlocks, VariableBlock};
use crate::defs::InfinityControl;
use crate::semantic_analysis::check_parameters_declaration::check_for_param_declaration;

fn check_per_variable(
    variable_type: &Vec<MascalVariableInitialDeclaration>, mut defined_var_names: HashSet<String>,
    can_be_numeric: bool
) -> Result<HashSet<String>, MascalError> {
    for var_decl in variable_type {
        if var_decl.infinity_control != InfinityControl::DISALLOW_INFINITY && !can_be_numeric {
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                line: 0,
                character: 0,
                source: String::from(
                    "Infinity control is only allowed within numeric types(integers/floats) as well as dynamic types"
                )
            });
        }
        let name: String = var_decl.name.clone();
        if defined_var_names.contains(&name) {
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                line: 0,
                character: 0,
                source: String::from("Cannot redeclare the same variable in a variable block")
            });
        }
        defined_var_names.insert(name);
    }
    Ok(defined_var_names)
}

pub fn variable_check_stage(abstract_syntax_tree: &AbstractSyntaxTree) -> Result<(), MascalError> {
    for block in &abstract_syntax_tree.blocks {
        let varblock: &VariableBlock = &match block {
            ScopedBlocks::PROGRAM(exec_block) => exec_block,
            ScopedBlocks::FUNCTION {execution_block, parameters, .. } => {
                check_for_param_declaration(execution_block, parameters)?;
                execution_block
            }
        }.variables;
        let mut defined_var_names: HashSet<String> = HashSet::new();
        defined_var_names = check_per_variable(&varblock.integers, defined_var_names, true)?;
        defined_var_names = check_per_variable(&varblock.floats, defined_var_names, true)?;
        defined_var_names = check_per_variable(&varblock.strings, defined_var_names, false)?;
        defined_var_names = check_per_variable(&varblock.booleans, defined_var_names, false)?;
        defined_var_names = check_per_variable(&varblock.dynamics, defined_var_names, true)?;
        check_per_variable(&varblock.types, defined_var_names, false)?;
    }
    Ok(())
}