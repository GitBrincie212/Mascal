use crate::defs::dynamic_int::IntegerNum;
use crate::defs::types::{MascalType};

#[derive(Clone, Debug)]
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

    pub fn as_type_string(&self) -> String {
        match self {
            MascalValue::String(_) => String::from("STRING"),
            MascalValue::Integer(_) => String::from("INTEGER"),
            MascalValue::Float(_) => String::from("FLOAT"),
            MascalValue::Boolean(_) => String::from("BOOLEAN"),
            MascalValue::NULL => String::from("NULL"),
            MascalValue::DynamicArray(values) => {
                if let Some(first) = values.first() {
                    return first.as_string() + format!("<{}>", values.len()).as_str();
                }
                String::from("ANY")
            }
            MascalValue::StaticArray(values) => {
                if let Some(first) = values.first() {
                    return first.as_string() + format!("[{}]", values.len()).as_str();
                }
                String::from("ANY")
            }
            MascalValue::Type(t) => {
                t.as_string()
            }
        }
    }
    
    pub fn extract_as_float(&self) -> Option<f64> {
        match self {
            MascalValue::Integer(i) => Some(i.as_f64()),
            MascalValue::Float(f) => Some(*f),
            _ => {None}
        }
    }

    pub fn extract_as_int(&self) -> Option<i128> {
        match self {
            MascalValue::Integer(i) => Some(i.to_i128()),
            MascalValue::Float(f) => Some(f.round() as i128),
            _ => {None}
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
            (_, MascalType::Dynamic) => true,
            (MascalValue::Type(..), MascalType::Type) => true,
            (MascalValue::StaticArray(values), _) => {
                values.iter().all(|val| val.is_atomic_type_of(value_type))
            }
            (MascalValue::DynamicArray(values), _) => {
                values.iter().all(|val| val.is_atomic_type_of(value_type))
            }
            _ => false
        }
    }
}