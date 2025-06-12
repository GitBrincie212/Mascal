use crate::defs::expressions::MascalExpression;

#[derive(Debug, Clone)]
pub enum MascalType {
    Integer,
    Float,
    Boolean,
    String,
    Dynamic,
    Type,
    NULL,
    DynamicArray {
        array_type: Box<MascalType>,
        initial_size: MascalExpression,
    },
    StaticArray {
        array_type: Box<MascalType>,
        size: MascalExpression,
    },
}