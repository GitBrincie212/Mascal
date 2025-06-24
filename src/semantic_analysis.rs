mod check_parameters_declaration;
mod variable_check_stage;

use crate::ast::AbstractSyntaxTree;
use crate::defs::blocks::{ScopedBlocks, VariableBlock};
use crate::defs::errors::MascalError;
use crate::runtime::FUNCTION_HASHSET;
use crate::semantic_analysis::check_parameters_declaration::check_for_param_declaration;
use std::collections::HashSet;
use std::sync::Arc;

pub fn conduct_semantic_analysis(
    abstract_syntax_tree: AbstractSyntaxTree,
) -> Result<AbstractSyntaxTree, MascalError> {
    for block in &abstract_syntax_tree.blocks {
        let varblock: &VariableBlock = &match block {
            ScopedBlocks::Program(exec_block) => exec_block,
            ScopedBlocks::Function {
                execution_block,
                parameters,
                name,
                ..
            } => {
                let converted_name: Arc<str> = Arc::from(name.as_str());
                FUNCTION_HASHSET.lock().unwrap().insert(converted_name);
                check_for_param_declaration(execution_block, parameters)?;
                execution_block
            }
        }
        .variables;
        let mut defined_var_names: HashSet<String> = HashSet::new();
        defined_var_names =
            variable_check_stage::check_per_variable(&varblock.integers, defined_var_names)?;
        defined_var_names =
            variable_check_stage::check_per_variable(&varblock.floats, defined_var_names)?;
        defined_var_names =
            variable_check_stage::check_per_variable(&varblock.strings, defined_var_names)?;
        defined_var_names =
            variable_check_stage::check_per_variable(&varblock.booleans, defined_var_names)?;
        defined_var_names =
            variable_check_stage::check_per_variable(&varblock.dynamics, defined_var_names)?;
        variable_check_stage::check_per_variable(&varblock.types, defined_var_names)?;
    }
    Ok(abstract_syntax_tree)
}
