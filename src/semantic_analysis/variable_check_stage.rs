use crate::defs::declerations::MascalVariableInitialDeclaration;
use crate::defs::errors::{MascalError, MascalErrorType};
use std::collections::HashSet;
use std::rc::Rc;

#[inline(always)]
pub(crate) fn check_per_variable(
    variable_type: &[MascalVariableInitialDeclaration],
    mut defined_var_names: HashSet<Rc<str>>,
) -> Result<HashSet<Rc<str>>, MascalError> {
    for var_decl in variable_type {
        let name: Rc<str> = var_decl.name.clone();
        if defined_var_names.contains(&*name) {
            return Err(MascalError {
                error_type: MascalErrorType::ParserError,
                line: 0,
                character: 0,
                source: String::from("Cannot redeclare the same variable in a variable block"),
            });
        }
        defined_var_names.insert(name);
    }
    Ok(defined_var_names)
}
