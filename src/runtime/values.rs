use crate::defs::dynamic_int::IntegerNum;
use crate::defs::types::{MascalType};

#[derive(Clone)]
pub enum MascalValue {
    Integer(IntegerNum),
    Float(f64),
    String(String),
    Boolean(bool),
    NULL,
    DynamicArray(Vec<MascalValue>),
    StaticArray(Vec<MascalValue>),
    Type(MascalType)
}

impl MascalValue {
    pub fn as_string(&self) -> String {
        match self {
            MascalValue::String(s) => s.clone(),
            MascalValue::Integer(i) => {i.as_string()}
            MascalValue::Float(f) => {f.to_string()}
            MascalValue::Boolean(b) => {b.to_string()}
            MascalValue::NULL => {String::from("NULL")}
            MascalValue::DynamicArray(values) => {
                String::from("<") + &*values.iter().map(|v| v.as_string())
                    .collect::<Vec<String>>()
                    .join(",") + ">"
            }
            MascalValue::StaticArray(values) => {
                String::from("[") + &*values.iter().map(|v| v.as_string())
                    .collect::<Vec<String>>()
                    .join(",") + "]"
            }
            MascalValue::Type(t) => {
                t.as_string()
            }
        }
    }
    
    pub fn is_type_of(&self, value_type: &MascalType) -> bool {
        match (self, value_type) {
            (MascalValue::Integer {..}, MascalType::Integer) => true,
            (MascalValue::Float{..}, MascalType::Float) => true,
            (MascalValue::String(..), MascalType::String) => true,
            (MascalValue::Boolean(..), MascalType::Boolean) => true,
            (MascalValue::NULL, _) => true,
            (MascalValue::Type(..), MascalType::Type) => true,
            (MascalValue::StaticArray(..), MascalType::StaticArray {array_type, ..}) => {
                self.is_type_of(array_type)
            }
            (MascalValue::DynamicArray(..), MascalType::DynamicArray {array_type, ..}) => {
                self.is_type_of(array_type)
            }
            _ => false
        }
    }

    pub fn is_atomic_type_of(&self, value_type: &MascalType) -> bool {
        match (self, value_type) {
            (MascalValue::Integer {..}, MascalType::Integer {..}) => true,
            (MascalValue::Float {..}, MascalType::Float) => true,
            (MascalValue::String(..), MascalType::String) => true,
            (MascalValue::Boolean(..), MascalType::Boolean) => true,
            (MascalValue::NULL, _) => true,
            (MascalValue::Type(..), MascalType::Type) => true,
            (MascalValue::StaticArray(values), MascalType::StaticArray {array_type, ..}) => {
                for val in values {
                    return val.is_atomic_type_of(value_type);
                }
                true
            }
            (MascalValue::DynamicArray(values), MascalType::DynamicArray {array_type, ..}) => {
                for val in values {
                    return val.is_atomic_type_of(value_type);
                }
                true
            }
            _ => false
        }
    }
}