use once_cell::sync::Lazy;
use regex::Regex;

pub type TokenRegexMap = Vec<(Regex, TokenType)>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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
    Slash,
    Modulo,
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
    Dot,
    Colon,
    ReturnIndicator,
    QuestionMark,
    OpenArrow,
    CloseArrow,

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

    // Special Stuff Regarding Mascal
    Unknown,
    NULL
}

pub static SCOPABLE_TOKEN_TYPES: Lazy<[TokenType; 11]> = Lazy::new(|| {
    [
        TokenType::Integer, TokenType::Float, TokenType::String,
        TokenType::Dynamic, TokenType::Type, TokenType::Boolean,
        TokenType::DefineProgram, TokenType::Implementation, TokenType::Variables,
        TokenType::DefineFunction, TokenType::Implementation,
    ]
});

pub static TOKEN_REGEX_MAP: Lazy<TokenRegexMap> = Lazy::new(|| {
    let mut map: TokenRegexMap = Vec::new();
    map.push((Regex::new(r"//.*").unwrap(), TokenType::Comment));
    map.push((Regex::new(r"NULL").unwrap(), TokenType::NULL));
    map.push((Regex::new(r"FOR|for|For").unwrap(), TokenType::For));
    map.push((Regex::new(r"If|if|IF").unwrap(), TokenType::If));
    map.push((Regex::new(r"Else|else|ELSE").unwrap(), TokenType::Else));
    map.push((Regex::new(r"Elif|elif|ELIF").unwrap(), TokenType::ElseIf));
    map.push((Regex::new(r"Const|const|CONST").unwrap(), TokenType::Const));
    map.push((Regex::new(r"IMPLEMENTATION|implemenentation|Implementation").unwrap(), TokenType::Implementation));
    map.push((Regex::new(r"VARIABLES|variables|Variables").unwrap(), TokenType::Variables));
    map.push((Regex::new(r"String|string|STRING").unwrap(), TokenType::String));
    map.push((Regex::new(r"INTEGER|integer|Integer").unwrap(), TokenType::Integer));
    map.push((Regex::new(r"FLOAT|float|Float").unwrap(), TokenType::Float));
    map.push((Regex::new(r"Dynamic|DYNAMIC|dynamic").unwrap(), TokenType::Dynamic));
    map.push((Regex::new(r"Boolean|BOOLEAN|boolean").unwrap(), TokenType::Boolean));
    map.push((Regex::new(r"DEFINE_FUNCTION|define_function|Define_Function").unwrap(), TokenType::DefineFunction));
    map.push((Regex::new(r"DEFINE_PROGRAM|define_program|Define_Program").unwrap(), TokenType::DefineProgram));
    map.push((Regex::new(r"While|while|WHILE").unwrap(), TokenType::While));
    map.push((Regex::new(r"From|from|FROM").unwrap(), TokenType::From));
    map.push((Regex::new(r"To|to|TO").unwrap(), TokenType::To));
    map.push((Regex::new(r"Mut|mut|MUT").unwrap(), TokenType::Mutable));
    map.push((Regex::new(r"with_step|With_Step|WITH_STEP").unwrap(), TokenType::WithStep));
    map.push((Regex::new(r"with_step|With_Step|WITH_STEP").unwrap(), TokenType::WithStep));
    map.push((Regex::new(r"typeof|TypeOf|TYPEOF").unwrap(), TokenType::Typeof));
    map.push((Regex::new(r"type|Type|TYPE").unwrap(), TokenType::Type));
    map.push((Regex::new(r"and|AND|And").unwrap(), TokenType::And));
    map.push((Regex::new(r"or|OR|Or").unwrap(), TokenType::Or));
    map.push((Regex::new(r"not|NOT|Not").unwrap(), TokenType::Not));
    map.push((Regex::new(r"(?:^|[^0-9.])(\d+\.\d*|\d*\.\d+)").unwrap(), TokenType::FloatLiteral));
    map.push((Regex::new(r"(\d+)").unwrap(), TokenType::IntegerLiteral));
    map.push((Regex::new("\"[^\"]*\"").unwrap(), TokenType::StringLiteral));
    map.push((Regex::new(r"->").unwrap(), TokenType::ReturnIndicator));
    map.push((Regex::new(r"\+").unwrap(), TokenType::Plus));
    map.push((Regex::new(r"-").unwrap(), TokenType::Minus));
    map.push((Regex::new(r"%").unwrap(), TokenType::Modulo));
    map.push((Regex::new(r"\*").unwrap(), TokenType::Asterisk));
    map.push((Regex::new(r"\?").unwrap(), TokenType::QuestionMark));
    map.push((Regex::new(r"/").unwrap(), TokenType::Slash));
    map.push((Regex::new(r"/").unwrap(), TokenType::Equals));
    map.push((Regex::new(r";").unwrap(), TokenType::Semicolon));
    map.push((Regex::new(r"\(").unwrap(), TokenType::OpenParen));
    map.push((Regex::new(r"\)").unwrap(), TokenType::CloseParen));
    map.push((Regex::new(r"\{").unwrap(), TokenType::OpenBrace));
    map.push((Regex::new(r"}").unwrap(), TokenType::CloseBrace));
    map.push((Regex::new(r"\[").unwrap(), TokenType::OpenBracket));
    map.push((Regex::new(r"]").unwrap(), TokenType::CloseBracket));
    map.push((Regex::new(r"\.").unwrap(), TokenType::Dot));
    map.push((Regex::new(r",").unwrap(), TokenType::Comma));
    map.push((Regex::new(r":").unwrap(), TokenType::Colon));
    map.push((Regex::new(r"!=").unwrap(), TokenType::NotEquals));
    map.push((Regex::new(r"=").unwrap(), TokenType::Equals));
    map.push((Regex::new(r"<-").unwrap(), TokenType::VariableInitializer));
    map.push((Regex::new(r">=").unwrap(), TokenType::GreaterThanEqual));
    map.push((Regex::new(r"<=").unwrap(), TokenType::LesserThanEqual));
    map.push((Regex::new(r"<").unwrap(), TokenType::OpenArrow));
    map.push((Regex::new(r">").unwrap(), TokenType::CloseArrow));
    map.push((Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(), TokenType::Identifier));

    map
});

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: &'a str,
    pub start: usize,
    pub end: usize,
    pub line: usize
}