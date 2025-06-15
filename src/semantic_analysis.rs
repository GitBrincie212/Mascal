mod variable_check_stage;
mod check_parameters_declaration;

use crate::ast::AbstractSyntaxTree;
use crate::defs::errors::MascalError;
use crate::semantic_analysis::variable_check_stage::variable_check_stage;

pub fn conduct_semantic_analysis(abstract_syntax_tree: AbstractSyntaxTree) -> Result<AbstractSyntaxTree, MascalError> {
    variable_check_stage(&abstract_syntax_tree)?;
    Ok(abstract_syntax_tree)
}