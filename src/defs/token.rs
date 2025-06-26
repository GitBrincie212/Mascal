use once_cell::sync::Lazy;
use std::collections::HashSet;
use logos::{Lexer, Logos, Skip};

#[allow(dead_code)]
fn newline_callback(lex: &mut Lexer<TokenType>) -> Skip {
    let slice: &str = lex.slice();
    let count: usize = slice.chars().filter(|&c| c == '\n').count();
    lex.extras += count;
    Skip
}


#[derive(Debug, Hash, Eq, PartialEq, Clone, Logos)]
#[repr(u8)]
#[logos(extras = usize)]
pub enum TokenType {
    #[logos(skip r"[\t \f]+")]
    Whitespace,

    #[logos(skip(r"\n", callback = newline_callback))]
    Newline,

    #[logos(skip r"//[^\n]*", priority=100)]
    Comment,

    // Identifiers Of Mascal
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
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
    #[regex(r"OR|or|Or")]
    Or,
    #[regex(r"AND|and|And")]
    And,
    #[regex(r"NOT|Not|not")]
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
    #[regex(r"For|for|FOR")]
    For,
    #[regex(r"If|if|IF")]
    If,
    #[regex(r"Else|else|Else")]
    Else,
    #[regex(r"ELIF|Elif|elif|Else if| ELSE IF| Else If")]
    ElseIf,
    #[regex(r"const|CONST|Const")]
    Const,
    #[regex(r"VARIABLES|Variables|variables")]
    Variables,
    #[regex(r"String|STRING|string")]
    String,
    #[regex(r"INTEGER|integer|Integer")]
    Integer,
    #[regex(r"FLOAT|float|Float")]
    Float,
    #[regex(r"Dynamic|dynamic|DYNAMIC")]
    Dynamic,
    #[regex(r"BOOLEAN|boolean|Boolean")]
    Boolean,
    #[regex(r"DEFINE_FUNCTION|Define_Function|define_function")]
    DefineFunction,
    #[regex(r"DEFINE_PROGRAM|Define_Program|define_program")]
    DefineProgram,
    #[regex(r"IMPLEMENTATION|implementation|Implementation")]
    Implementation,
    #[regex(r"WHILE|while|While")]
    While,
    #[regex(r"FROM|from|From")]
    From,
    #[regex(r"To|to|TO")]
    To,
    #[regex(r"WITH_STEP|with_step|With_Step")]
    WithStep,
    #[regex(r"MUT|mut|Mut")]
    Mutable,
    #[regex(r"TYPE|type|Type")]
    Type,
    #[regex(r"TYPEOF|Typeof|typeof")]
    Typeof,
    #[regex(r"TRUE|true|True")]
    True,

    #[regex(r"FALSE|false|False")]
    False,

    #[regex(r"THROW|throw|Throw")]
    Throw,

    #[regex(r"BREAK|break|Break")]
    Break,

    #[regex(r"Continue|continue|CONTINUE")]
    Continue,

    #[token("<<")]
    OpenDynamicArray,
    #[token(">>")]
    CloseDynamicArray,

    // Special Stuff Regarding Mascal
    #[regex(r"NULL|Null|null")]
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
