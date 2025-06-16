use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::InfinityControl;
use crate::defs::token::TokenType;
use crate::runtime::execute_expression::execute_expression;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

#[derive(Debug, Clone)]
pub enum MascalUnprocessedType {
    Integer,
    Float,
    Boolean,
    String,
    Dynamic,
    Type,
    DynamicArray {
        array_type: Box<MascalUnprocessedType>,
        initial_size: MascalExpression,
    },
    StaticArray {
        array_type: Box<MascalUnprocessedType>,
        size: MascalExpression,
    },
}

impl MascalUnprocessedType {
    pub fn as_string(&self) -> String {
        match self {
            MascalUnprocessedType::String => String::from("String"),
            MascalUnprocessedType::Integer => String::from("Integer"),
            MascalUnprocessedType::Float => {String::from("Float")}
            MascalUnprocessedType::Boolean => {String::from("Boolean")}
            MascalUnprocessedType::DynamicArray {array_type, initial_size} => {
                todo!()
            }
            MascalUnprocessedType::StaticArray {array_type, size} => {
                todo!()
            }
            MascalUnprocessedType::Dynamic => {
                String::from("Dynamic")
            }
            MascalUnprocessedType::Type => {
                String::from("Type")
            }
        }
    }
}

impl MascalType {
    pub fn as_string(&self) -> String {
        match self {
            MascalType::String => String::from("String"),
            MascalType::Integer => String::from("Integer"),
            MascalType::Float => {String::from("Float")}
            MascalType::Boolean => {String::from("Boolean")}
            MascalType::DynamicArray {array_type, initial_size} => {
                todo!()
            }
            MascalType::StaticArray {array_type, size} => {
                todo!()
            }
            MascalType::Dynamic => {
                String::from("Dynamic")
            }
            MascalType::Type => {
                String::from("Type")
            }
        }
    }
}

pub fn token_type_to_atom_mascal_type(tt: &TokenType) -> Option<MascalUnprocessedType> {
    match tt {
        TokenType::Integer => Some(MascalUnprocessedType::Integer),
        TokenType::Float => Some(MascalUnprocessedType::Float),
        TokenType::String => Some(MascalUnprocessedType::String),
        TokenType::Dynamic => Some(MascalUnprocessedType::Dynamic),
        TokenType::Type => Some(MascalUnprocessedType::Type),
        _ => None
    }
}

macro_rules! process_array_size {
    ($array_size: expr, $mutable_size: expr) => {
        let size = execute_expression($array_size, &ExecutionData {
            variable_table: None,
            infinity_control: InfinityControl::DisallowInfinity
        })?.into_owned();
        match size {
            MascalValue::Integer(int) => {
                if int.is_negative_or_zero() {
                    return Err(MascalError {
                        error_type: MascalErrorType::RuntimeError,
                        line: 0,
                        character: 0,
                        source: format!("Cannot create an array with a value of {:?} (negative or zero)", int.as_string())
                    })
                }
                $mutable_size = Some(int.to_i128() as usize);
            }
            _ => return Err(MascalError {
                error_type: MascalErrorType::TypeError,
                line: 0,
                character: 0,
                source: format!("Cannot determine size of an array with the value {:?}", size.as_string())
            })
        };
    };
}

pub fn to_processed_type(unprocessed: MascalUnprocessedType) -> Result<MascalType, MascalError> {
    match unprocessed {
        MascalUnprocessedType::Integer => Ok(MascalType::Integer),
        MascalUnprocessedType::Float => Ok(MascalType::Float),
        MascalUnprocessedType::Boolean => Ok(MascalType::Boolean),
        MascalUnprocessedType::String => Ok(MascalType::String),
        MascalUnprocessedType::Dynamic => Ok(MascalType::Dynamic),
        MascalUnprocessedType::Type => Ok(MascalType::Type),
        MascalUnprocessedType::DynamicArray {array_type, initial_size} => {
            let mut size_val: Option<usize> = None;
            process_array_size!(initial_size, size_val);
            Ok(MascalType::DynamicArray {
                array_type: Box::new(to_processed_type(*array_type)?), 
                initial_size: size_val
            })
        }
        MascalUnprocessedType::StaticArray {array_type, size} => {
            let mut size_val: Option<usize> = None;
            process_array_size!(size, size_val);
            if size_val.is_none() {
                return Err(MascalError {
                    error_type: MascalErrorType::RuntimeError,
                    line: 0,
                    character: 0,
                    source: String::from("Size must be defined for the static array")
                })
            }
            Ok(MascalType::StaticArray {
                array_type: Box::new(to_processed_type(*array_type)?),
                size: size_val.unwrap()
            })
        }
    }
}

#[derive(Debug, Clone)]
pub enum MascalType {
    Integer,
    Float,
    Boolean,
    String,
    Dynamic,
    Type,
    DynamicArray {
        array_type: Box<MascalType>,
        initial_size: Option<usize>,
    },
    StaticArray {
        array_type: Box<MascalType>,
        size: usize,
    },
}