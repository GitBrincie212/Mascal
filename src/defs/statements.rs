use crate::defs::expressions::MascalExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct MascalConditionalBranch {
    pub condition: Option<MascalExpression>,
    pub statements: Vec<MascalStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MascalConditionalStatement {
    pub branches: Vec<MascalConditionalBranch>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MascalForLoopStatement {
    pub variable: String,
    pub from: MascalExpression,
    pub to: MascalExpression,
    pub step: MascalExpression,
    pub statements: Vec<MascalStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MascalStatement {
    If(MascalConditionalStatement),
    Elif(MascalConditionalStatement),
    Else(MascalConditionalStatement),
    While(MascalConditionalStatement),
    For(MascalForLoopStatement),
    Expression(MascalExpression),
    Declaration(MascalExpression),
}