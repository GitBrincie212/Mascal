use std::collections::HashSet;
use crate::defs::blocks::{ExecutionBlock, MascalParameter};
use crate::defs::errors::{MascalError, MascalErrorType};

pub fn check_for_param_declaration(function_block: &ExecutionBlock, params: &Vec<MascalParameter>) -> Result<(), MascalError> {
    let mut varnames: HashSet<&String> = HashSet::new();
    for var in function_block.variables.iter_all() {
        if var.initial_value.is_some() {
            return Err(MascalError {
                character: 0,
                line: 0,
                error_type: MascalErrorType::ParserError,
                source: String::from("Cannot define an initial value for a parameter inside a function"),
            })
        }
        varnames.insert(&var.name);
    }
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