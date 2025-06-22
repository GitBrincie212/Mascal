use crate::defs::expressions::MascalExpression;

#[derive(Debug, Clone)]
pub struct MascalConditionalBranch {
    pub condition: Option<MascalExpression>,
    pub statements: Vec<MascalStatement>,
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
    For {
        variable: String,
        from: MascalExpression,
        to: MascalExpression,
        step: MascalExpression,
        statements: Vec<MascalStatement>,
    },

    /*
    (10 + 5) / 3;
    */
    ExpressionStatement(MascalExpression),
    
    /*
    a <- 3;
     */
    Declaration {
        variable: MascalExpression,
        value: MascalExpression,
    },
    
    /*
    THROW RuntimeError: "...";
     */
    Throw {
        error_type: String,
        message: String,
    },
    
    /* BREAK; */
    Break,
    
    /* CONTINUE */
    Continue
}