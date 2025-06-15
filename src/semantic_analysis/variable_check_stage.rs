use std::collections::HashSet;
use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::InfinityControl;

pub(crate) fn check_per_variable(
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