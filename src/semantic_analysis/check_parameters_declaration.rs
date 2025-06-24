use crate::defs::blocks::{ExecutionBlock, MascalParameter};
use crate::defs::errors::{MascalError, MascalErrorType};
use std::collections::HashSet;
use std::rc::Rc;

pub fn check_for_param_declaration(
    function_block: &ExecutionBlock,
    params: &[MascalParameter],
) -> Result<(), MascalError> {
    let param_names: HashSet<Rc<str>> = params.iter().map(|p| p.name.clone()).collect();

    if function_block
        .variables
        .iter_all()
        .iter()
        .any(|v| param_names.contains(&v.name) && v.initial_value.is_some())
    {
        return Err(MascalError {
            character: 0,
            line: 0,
            error_type: MascalErrorType::ParserError,
            source: String::from(
                "Cannot define an initial value for a parameter inside a function",
            ),
        });
    }

    let declared: HashSet<Rc<str>> = function_block
        .variables
        .iter_all()
        .iter()
        .map(|v| v.name.clone())
        .collect();

    for param in params {
        if !declared.contains(&param.name) {
            return Err(MascalError {
                character: 0,
                line: 0,
                error_type: MascalErrorType::ParserError,
                source: format!(
                    "Parameter named {:?} has not been declared in the variables block",
                    param.name
                ),
            });
        }
    }

    Ok(())
}
