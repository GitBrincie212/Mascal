use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::runtime::values::MascalValue;
use crate::runtime::variable_table::{VariableData, VariableTable};
use std::cell::RefCell;
use std::rc::Rc;

pub fn extract_variable_data(
    base: MascalExpression,
    variable_table: Rc<RefCell<VariableTable>>,
    layers: &[(MascalValue, bool)],
) -> Result<(String, VariableData), MascalError> {
    let vartable_borrow = variable_table.borrow();

    let varname: String = match base {
        MascalExpression::Symbolic(name) => name,
        other => {
            return Err(MascalError {
                error_type: MascalErrorType::RuntimeError,
                line: 0,
                character: 0,
                source: format!("Left hand-side must be a variable, but got {:?}", other),
            });
        }
    };

    let vardata = vartable_borrow
        .get(&varname)
        .ok_or_else(|| MascalError {
            error_type: MascalErrorType::RuntimeError,
            line: 0,
            character: 0,
            source: format!(
                "Expected a defined variable but got an unknown called {}",
                varname
            ),
        })?
        .clone();

    if vardata.is_constant {
        return Err(MascalError {
            error_type: MascalErrorType::RuntimeError,
            character: 0,
            line: 0,
            source: format!("Cannot assign an array to a constant variable {}", varname),
        });
    }

    if vardata.value.is_none() && !layers.is_empty() {
        return Err(MascalError {
            error_type: MascalErrorType::RuntimeError,
            character: 0,
            line: 0,
            source: format!(
                "Cannot assign an array to a uninitialized variable {}",
                varname
            ),
        });
    }

    Ok((varname, vardata))
}
