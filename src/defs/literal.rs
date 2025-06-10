#[derive(Debug, Clone, PartialEq)]
pub enum MascalLiteral {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    NULL,
}