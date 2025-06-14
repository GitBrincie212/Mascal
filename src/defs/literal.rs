use crate::defs::dynamic_int::IntegerNum;

#[derive(Debug, Clone, PartialEq)]
pub enum MascalLiteral {
    Integer(IntegerNum),
    Float(f64),
    Boolean(bool),
    String(String),
    NULL,
}