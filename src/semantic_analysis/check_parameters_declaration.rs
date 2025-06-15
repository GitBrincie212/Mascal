use std::collections::HashSet;
use crate::defs::blocks::{ExecutionBlock, MascalParameter};
use crate::defs::errors::{MascalError, MascalErrorType};

pub fn check_for_param_declaration(function_block: &ExecutionBlock, params: &Vec<MascalParameter>) -> Result<(), MascalError> {
    let varnames: HashSet<String> = HashSet::from_iter(
        function_block
            .variables
            .iter_all()
            .iter()
            .map(|var| var.name.to_string())
    );
    for param in params {
        if !varnames.contains(&param.name) {
            return Err(MascalError {
                character: 0,
                line: 0,
                error_type: MascalErrorType::ParserError,
                source: format!("Parameter of the name {:?} has not been declared in the variables block", param.name),
            })
        }
    }
    Ok(())
}