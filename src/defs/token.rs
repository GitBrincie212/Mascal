use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

pub type TokenRegexMap = Vec<(Regex, TokenType)>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum TokenType {
    // Identifiers Of Mascal
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    VariableInitializer,

    // Specific Operators And Symbols That Mascal Supports
    Plus,
    Minus,
    Asterisk,
    Division,
    Modulo,
    Exponentiation,
    Equals,
    NotEquals,
    GreaterThanEqual,
    LesserThanEqual,
    Or,
    And,
    Not,
    Semicolon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    ReturnIndicator,
    QuestionMark,
    LessThan,
    GreaterThan,

    Comment,

    // Reserved Keywords
    For,
    If,
    Else,
    ElseIf,
    Const,
    Variables,
    String,
    Integer,
    Float,
    Dynamic,
    Boolean,
    DefineFunction,
    DefineProgram,
    Implementation,
    While,
    From,
    To,
    WithStep,
    Mutable,
    Type,
    Typeof,
    True,
    False,
    Throw,
    Break,
    Continue,
    OpenDynamicArray,
    CloseDynamicArray,

    // Special Stuff Regarding Mascal
    Unknown,
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

pub static TOKEN_REGEX_MAP: Lazy<TokenRegexMap> = Lazy::new(|| {
    vec![
        (Regex::new(r"//.*").unwrap(), TokenType::Comment),
        (
            Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
            TokenType::Identifier,
        ),
        (Regex::new(r"TRUE|true|True").unwrap(), TokenType::True),
        (Regex::new(r"FALSE|false|False").unwrap(), TokenType::False),
        (Regex::new(r"BREAK|break|Break").unwrap(), TokenType::Break),
        (
            Regex::new(r"CONTINUE|continue|Continue").unwrap(),
            TokenType::Continue,
        ),
        (Regex::new(r"NULL|null|Null").unwrap(), TokenType::Null),
        (Regex::new(r"FOR|for|For").unwrap(), TokenType::For),
        (Regex::new(r"If|if|IF").unwrap(), TokenType::If),
        (Regex::new(r"Else|else|ELSE").unwrap(), TokenType::Else),
        (Regex::new(r"Elif|elif|ELIF").unwrap(), TokenType::ElseIf),
        (Regex::new(r"Const|const|CONST").unwrap(), TokenType::Const),
        (
            Regex::new(r"IMPLEMENTATION|implementation|Implementation").unwrap(),
            TokenType::Implementation,
        ),
        (
            Regex::new(r"VARIABLES|variables|Variables").unwrap(),
            TokenType::Variables,
        ),
        (
            Regex::new(r"String|string|STRING").unwrap(),
            TokenType::String,
        ),
        (
            Regex::new(r"INTEGER|integer|Integer").unwrap(),
            TokenType::Integer,
        ),
        (Regex::new(r"FLOAT|float|Float").unwrap(), TokenType::Float),
        (
            Regex::new(r"Dynamic|DYNAMIC|dynamic").unwrap(),
            TokenType::Dynamic,
        ),
        (
            Regex::new(r"Boolean|BOOLEAN|boolean").unwrap(),
            TokenType::Boolean,
        ),
        (
            Regex::new(r"DEFINE_FUNCTION|define_function|Define_Function").unwrap(),
            TokenType::DefineFunction,
        ),
        (
            Regex::new(r"DEFINE_PROGRAM|define_program|Define_Program").unwrap(),
            TokenType::DefineProgram,
        ),
        (Regex::new(r"While|while|WHILE").unwrap(), TokenType::While),
        (Regex::new(r"From|from|FROM").unwrap(), TokenType::From),
        (Regex::new(r"To|to|TO").unwrap(), TokenType::To),
        (Regex::new(r"Mut|mut|MUT").unwrap(), TokenType::Mutable),
        (
            Regex::new(r"with_step|With_Step|WITH_STEP").unwrap(),
            TokenType::WithStep,
        ),
        (
            Regex::new(r"typeof|Typeof|TYPEOF").unwrap(),
            TokenType::Typeof,
        ),
        (Regex::new(r"type|Type|TYPE").unwrap(), TokenType::Type),
        (Regex::new(r"and|AND|And").unwrap(), TokenType::And),
        (Regex::new(r"or|OR|Or").unwrap(), TokenType::Or),
        (Regex::new(r"not|NOT|Not").unwrap(), TokenType::Not),
        (Regex::new(r"throw|THROW|thro").unwrap(), TokenType::Throw),
        (Regex::new(r"\^").unwrap(), TokenType::Exponentiation),
        (
            Regex::new(r"(\d+\.\d*)|(\d*\.\d+)").unwrap(),
            TokenType::FloatLiteral,
        ),
        (Regex::new(r"(\d+)").unwrap(), TokenType::IntegerLiteral),
        (
            Regex::new("\"([^\"]*)\"").unwrap(),
            TokenType::StringLiteral,
        ),
        (Regex::new(r"->").unwrap(), TokenType::ReturnIndicator),
        (Regex::new(r"\+").unwrap(), TokenType::Plus),
        (Regex::new(r"-").unwrap(), TokenType::Minus),
        (Regex::new(r"%").unwrap(), TokenType::Modulo),
        (Regex::new(r"\*").unwrap(), TokenType::Asterisk),
        (Regex::new(r"\?").unwrap(), TokenType::QuestionMark),
        (Regex::new(r"/").unwrap(), TokenType::Division),
        (Regex::new(r";").unwrap(), TokenType::Semicolon),
        (Regex::new(r"\(").unwrap(), TokenType::OpenParen),
        (Regex::new(r"\)").unwrap(), TokenType::CloseParen),
        (Regex::new(r"\{").unwrap(), TokenType::OpenBrace),
        (Regex::new(r"}").unwrap(), TokenType::CloseBrace),
        (Regex::new(r"\[").unwrap(), TokenType::OpenBracket),
        (Regex::new(r"]").unwrap(), TokenType::CloseBracket),
        (Regex::new(r",").unwrap(), TokenType::Comma),
        (Regex::new(r":").unwrap(), TokenType::Colon),
        (Regex::new(r"!=").unwrap(), TokenType::NotEquals),
        (Regex::new(r"=").unwrap(), TokenType::Equals),
        (Regex::new(r"<-").unwrap(), TokenType::VariableInitializer),
        (Regex::new(r">=").unwrap(), TokenType::GreaterThanEqual),
        (Regex::new(r"<=").unwrap(), TokenType::LesserThanEqual),
        (Regex::new(r"<").unwrap(), TokenType::LessThan),
        (Regex::new(r">").unwrap(), TokenType::GreaterThan),
        (Regex::new(r">>").unwrap(), TokenType::CloseDynamicArray),
        (Regex::new(r"<<").unwrap(), TokenType::OpenDynamicArray),
    ]
});

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: &'a str,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}
