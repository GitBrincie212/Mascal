use std::collections::HashSet;
use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};

pub(crate) fn check_per_variable(
    variable_type: &Box<[MascalVariableInitialDeclaration]>, mut defined_var_names: HashSet<String>,
) -> Result<HashSet<String>, MascalError> {
    for var_decl in variable_type {
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