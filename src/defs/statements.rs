use crate::defs::expressions::MascalExpression;

#[derive(Debug, Clone)]
pub struct MascalConditionalBranch {
    pub condition: Option<MascalExpression>,
    pub statements: Vec<MascalStatement>,
}

#[derive(Debug, Clone)]
pub struct MascalForLoopStatement {
    pub variable: String,
    pub from: MascalExpression,
    pub to: MascalExpression,
    pub step: MascalExpression,
    pub statements: Vec<MascalStatement>,
}

#[derive(Debug, Clone)]
pub struct MascalDeclarationStatement {
    pub variable: String,
    pub value: MascalExpression,
}

#[derive(Debug, Clone)]
pub enum MascalStatement {
    /*
    IF a = b {
      // ...
    };
    */
    ConditionalStatement(Vec<MascalConditionalBranch>),

    /*
    WHILE a = b {
      // ...
    };
    */
    While(MascalConditionalBranch),

    /*
    FOR i FROM a TO b {
      // ...
    };
    */
    For(MascalForLoopStatement),

    /*
    (10 + 5) / 3;
    */
    Expression(MascalExpression),
    
    /*
    a <- 3;
     */
    Declaration(MascalDeclarationStatement),
}