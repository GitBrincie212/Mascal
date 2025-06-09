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
    Equals,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Colon,
    ReturnIndicator,

    Comment,

    // Special Stuff Regarding Mascal
    Unknown,
    NULL
}

pub static TOKEN_REGEX_MAP: Lazy<TokenRegexMap> = Lazy::new(|| {
    let mut map: TokenRegexMap = Vec::new();
    map.push((Regex::new(r"//.*").unwrap(), TokenType::Comment));
    map.push((Regex::new(r"NULL").unwrap(), TokenType::NULL));
    map.push((Regex::new(r"(?:^|[^0-9.])(\d+\.\d*|\d*\.\d+)").unwrap(), TokenType::FloatLiteral));
    map.push((Regex::new(r"(\d+)").unwrap(), TokenType::IntegerLiteral));
    map.push((Regex::new("\"[^\"]*\"").unwrap(), TokenType::StringLiteral));
    map.push((Regex::new(r"->").unwrap(), TokenType::ReturnIndicator));
    map.push((Regex::new(r"\+").unwrap(), TokenType::Plus));
    map.push((Regex::new(r"-").unwrap(), TokenType::Minus));
    map.push((Regex::new(r"\*").unwrap(), TokenType::Asterisk));
    map.push((Regex::new(r"/").unwrap(), TokenType::Slash));
    map.push((Regex::new(r"/").unwrap(), TokenType::Equals));
    map.push((Regex::new(r";").unwrap(), TokenType::Semicolon));
    map.push((Regex::new(r"\(").unwrap(), TokenType::LParen));
    map.push((Regex::new(r"\)").unwrap(), TokenType::RParen));
    map.push((Regex::new(r"\{").unwrap(), TokenType::LBrace));
    map.push((Regex::new(r"}").unwrap(), TokenType::RBrace));
    map.push((Regex::new(r"\[").unwrap(), TokenType::LBracket));
    map.push((Regex::new(r"]").unwrap(), TokenType::RBracket));
    map.push((Regex::new(r"\.").unwrap(), TokenType::Dot));
    map.push((Regex::new(r",").unwrap(), TokenType::Comma));
    map.push((Regex::new(r":").unwrap(), TokenType::Colon));
    map.push((Regex::new(r"=").unwrap(), TokenType::Equals));
    map.push((Regex::new(r"<-").unwrap(), TokenType::VariableInitializer));
    map.push((Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(), TokenType::Identifier));

    map
});

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: &'a str,
    pub start: usize,
    pub end: usize,
    pub line: usize
}