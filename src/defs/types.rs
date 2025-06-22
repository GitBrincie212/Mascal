use crate::defs::errors::{MascalError};
use crate::defs::token::TokenType;
use crate::runtime::values::MascalValue;

#[derive(Debug, Clone)]
pub enum MascalUnprocessedType {
    Integer,
    Float,
    Boolean,
    String,
    Dynamic,
    Type,
    DynamicArray (Box<MascalUnprocessedType>),
    StaticArray (Box<MascalUnprocessedType>),
}

impl MascalType {
    pub fn as_string(&self) -> String {
        let mut modifiers: Vec<&str> = Vec::new();
        let mut ty: &MascalType = self;
        loop {
            match ty {
                MascalType::StaticArray(inner) => {
                    modifiers.push("[]");
                    ty = inner;
                }
                MascalType::DynamicArray(inner) => {
                    modifiers.push("<>");
                    ty = inner;
                }
                _ => break,
            }
        }

        let mut s: String = String::from(match ty {
            MascalType::String => "STRING",
            MascalType::Integer => "INTEGER",
            MascalType::Float => "FLOAT",
            MascalType::Boolean => "BOOLEAN",
            MascalType::Dynamic => "DYNAMIC",
            MascalType::Type => "TYPE",
            _ => unreachable!(),
        });

        for &m in &modifiers {
            s.push_str(m);
        }
        
        s
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

pub fn to_processed_type(unprocessed: MascalUnprocessedType) -> Result<MascalType, MascalError> {
    match unprocessed {
        MascalUnprocessedType::Integer => Ok(MascalType::Integer),
        MascalUnprocessedType::Float => Ok(MascalType::Float),
        MascalUnprocessedType::Boolean => Ok(MascalType::Boolean),
        MascalUnprocessedType::String => Ok(MascalType::String),
        MascalUnprocessedType::Dynamic => Ok(MascalType::Dynamic),
        MascalUnprocessedType::Type => Ok(MascalType::Type),
        MascalUnprocessedType::DynamicArray(array_type) => {
            Ok(MascalType::DynamicArray(Box::new(to_processed_type(*array_type)?)))
        }
        MascalUnprocessedType::StaticArray(array_type) => {
            Ok(MascalType::StaticArray(Box::new(to_processed_type(*array_type)?)))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MascalType {
    Integer,
    Float,
    Boolean,
    String,
    Dynamic,
    Type,
    DynamicArray (Box<MascalType>),
    StaticArray(Box<MascalType>),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MascalTypeKind {
    Integer,
    Float,
    Boolean,
    String,
    Dynamic,
    Type,
    DynamicArray,
    StaticArray
}

impl MascalTypeKind {
    pub fn is_type_of_for_value(&self, v: &MascalValue) -> bool {
        match (self, v) {
            (MascalTypeKind::Dynamic, _) => true,
            (MascalTypeKind::Integer, MascalValue::Integer(..)) => true,
            (MascalTypeKind::Float, MascalValue::Float(..)) => true,
            (MascalTypeKind::String, MascalValue::String(..)) => true,
            (MascalTypeKind::Type, MascalValue::Type(..)) => true,
            (MascalTypeKind::Boolean, MascalValue::Boolean(..)) => true,
            (MascalTypeKind::StaticArray, MascalValue::StaticArray {..}) => true,
            (MascalTypeKind::DynamicArray, MascalValue::DynamicArray {..}) => true,
            _ => false
        }
    }
    
    pub fn as_string(&self) -> String {
        match self {
            MascalTypeKind::String => String::from("STRING"),
            MascalTypeKind::Integer => String::from("INTEGER"),
            MascalTypeKind::Float => {String::from("FLOAT")}
            MascalTypeKind::Boolean => {String::from("BOOLEAN")}
            MascalTypeKind::DynamicArray => {
                String::from("DYNAMIC_ARRAY")
            }
            MascalTypeKind::StaticArray => {
                String::from("STATIC_ARRAY")
            }
            MascalTypeKind::Dynamic => {
                String::from("DYNAMIC")
            }
            MascalTypeKind::Type => {
                String::from("TYPE")
            }
        }
    }
}