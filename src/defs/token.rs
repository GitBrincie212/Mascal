use once_cell::sync::Lazy;
use std::collections::HashSet;
use logos::{Lexer, Logos, Skip};

#[allow(dead_code)]
fn newline_callback(lexer: &mut Lexer<'_, TokenType>) -> Skip {
    lexer.extras += 1;
    Skip
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Logos)]
#[repr(u8)]
#[logos(extras = usize)]
#[logos(skip r"[ \t\f]+")]
pub enum TokenType {
    #[regex(r"\n", callback = newline_callback)]
    Newline,
    #[regex(r"//[^\n]*", callback = logos::skip)]
    Comment,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", priority=0)]
    Identifier,
    #[regex(r"(\d+)")]
    IntegerLiteral,
    #[regex(r"(\d+\.\d*)|(\d*\.\d+)")]
    FloatLiteral,
    #[regex("\"([^\"]*)\"")]
    StringLiteral,
    #[regex(r"<-")]
    VariableInitializer,

    // Specific Operators And Symbols That Mascal Supports
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Division,
    #[token(r"%")]
    Modulo,
    #[token("^")]
    Exponentiation,
    #[regex(r"=|==")]
    Equals,
    #[token("!=")]
    NotEquals,
    #[token(">=")]
    GreaterThanEqual,
    #[token("<=")]
    LesserThanEqual,
    #[regex(r"OR|or|Or", priority=10)]
    Or,
    #[regex(r"AND|and|And", priority=10)]
    And,
    #[regex(r"NOT|Not|not", priority=10)]
    Not,
    #[token(";")]
    Semicolon,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,

    #[token("->")]
    ReturnIndicator,
    #[token("?")]
    QuestionMark,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,

    // Reserved Keywords
    #[regex(r"For|for|FOR", priority=10)]
    For,
    #[regex(r"If|if|IF", priority=10)]
    If,
    #[regex(r"Else|else|ELSE", priority=10)]
    Else,
    #[regex(r"ELIF|Elif|elif|Else if| ELSE IF| Else If", priority=10)]
    ElseIf,
    #[regex(r"const|CONST|Const", priority=10)]
    Const,
    #[regex(r"VARIABLES|Variables|variables", priority=10)]
    Variables,
    #[regex(r"String|STRING|string", priority=10)]
    String,
    #[regex(r"INTEGER|integer|Integer", priority=10)]
    Integer,
    #[regex(r"FLOAT|float|Float", priority=10)]
    Float,
    #[regex(r"Dynamic|dynamic|DYNAMIC", priority=10)]
    Dynamic,
    #[regex(r"BOOLEAN|boolean|Boolean", priority=10)]
    Boolean,
    #[regex(r"DEFINE_FUNCTION|Define_Function|define_function", priority=10)]
    DefineFunction,
    #[regex(r"DEFINE_PROGRAM|Define_Program|define_program", priority=10)]
    DefineProgram,
    #[regex(r"IMPLEMENTATION|implementation|Implementation", priority=10)]
    Implementation,
    #[regex(r"WHILE|while|While", priority=10)]
    While,
    #[regex(r"FROM|from|From", priority=10)]
    From,
    #[regex(r"To|to|TO", priority=10)]
    To,
    #[regex(r"WITH_STEP|with_step|With_Step", priority=10)]
    WithStep,
    #[regex(r"MUT|mut|Mut", priority=10)]
    Mutable,
    #[regex(r"TYPE|type|Type", priority=10)]
    Type,
    #[regex(r"TYPEOF|Typeof|typeof", priority=10)]
    Typeof,
    #[regex(r"TRUE|true|True", priority=10)]
    True,

    #[regex(r"FALSE|false|False", priority=10)]
    False,

    #[regex(r"THROW|throw|Throw", priority=10)]
    Throw,

    #[regex(r"BREAK|break|Break", priority=10)]
    Break,

    #[regex(r"Continue|continue|CONTINUE", priority=10)]
    Continue,

    #[token("<<")]
    OpenDynamicArray,
    #[token(">>")]
    CloseDynamicArray,

    // Special Stuff Regarding Mascal
    #[regex(r"NULL|Null|null", priority=10)]
    Null,
}

pub static SCOPABLE_TOKEN_TYPES: Lazy<HashSet<TokenType>> = Lazy::new(|| {
    HashSet::from_iter(
        [
            TokenType::Integer,
            TokenType::Float,
            TokenType::String,
            TokenType::Dynamic,
            TokenType::Type,
            TokenType::Boolean,
            TokenType::DefineProgram,
            TokenType::Implementation,
            TokenType::Variables,
            TokenType::DefineFunction,
            TokenType::Implementation,
        ]
        .iter()
        .cloned(),
    )
});

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: &'a str,
    pub start: usize,
    pub line: usize,
}
