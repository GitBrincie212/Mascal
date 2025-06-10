#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MascalType {
    Integer,
    Float,
    Boolean,
    String,
    Dynamic,
    NULL,
    DynamicArray {
        array_type: Box<MascalType>,
        initial_size: usize,
    },
    StaticArray {
        array_type: Box<MascalType>,
        size: usize,
    },
}