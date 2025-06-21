use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::types::{to_processed_type, MascalType, MascalUnprocessedType};
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

pub fn execute_typecast(
    function: Box<MascalUnprocessedType>, arguments: Vec<MascalExpression>, 
    exec_data: Rc<RefCell<ExecutionData>>
) -> Result<MascalValue, MascalError> {
    if arguments.len() != 1 {
        return Err(MascalError {
            error_type: MascalErrorType::ArgumentError,
            character: 0,
            line: 0,
            source: format!("Expected one value for the type casting but got {:?} arguments", arguments.len())
        })
    }
    let value: MascalValue = execute_expression(arguments[0].clone(), exec_data)?;
    match (to_processed_type(*function.clone())?, value) {
        (MascalType::Integer, MascalValue::Float(f)) => {
            Ok(MascalValue::Integer(IntegerNum::new(f.round() as i128)))
        }

        (MascalType::Float, MascalValue::Integer(i)) => {
            Ok(MascalValue::Float(i.as_f64()))
        }

        (MascalType::String, v) => {
            Ok(MascalValue::String(Arc::from(v.as_string()?)))
        }

        (MascalType::Integer, MascalValue::Boolean(b)) => {
            Ok(MascalValue::Integer(IntegerNum::I8(if b {1} else {0})))
        }

        (MascalType::Float, MascalValue::Boolean(b)) => {
            Ok(MascalValue::Float(if b {1f64} else {0f64}))
        }

        (MascalType::Dynamic, v) => {
            Ok(v)
        }
        
        (t, v) => {
            if v.is_type_of(&t) {
                return Ok(v);
            }
            Err(MascalError {
                error_type: MascalErrorType::TypeError,
                line: 0,
                character: 0,
                source: format!("Unable to cast {:?} into the type {:?}", v.as_string()?, t.as_string())
            })
        },
    }
}