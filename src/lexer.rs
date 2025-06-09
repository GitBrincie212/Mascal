use std::fmt::Debug;
use regex::Regex;

type TokenRegexMap = Vec<(Regex, TokenType)>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum TokenType {
    // Identifiers Of Mascal
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,

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

    Comment,

    // Special Stuff Regarding Mascal
    Unknown,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub line: usize
}

fn build_regex_map() -> TokenRegexMap {
    let mut map: TokenRegexMap = Vec::new();
    map.push((Regex::new(r"//.*").unwrap(), TokenType::Comment));
    map.push((Regex::new(r"(?:^|[^0-9.])(\d+\.\d*|\d*\.\d+)").unwrap(), TokenType::FloatLiteral));
    map.push((Regex::new(r"(\d+)").unwrap(), TokenType::IntegerLiteral));
    map.push((Regex::new("\"[^\"]*\"").unwrap(), TokenType::StringLiteral));
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
    map.push((Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(), TokenType::Identifier));

    map
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let map: TokenRegexMap = build_regex_map();
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_index: usize = 0;
    let minified_input = Regex::new(r"[\s\t]+").unwrap().replace_all(input, "");
    let mut line_counter: usize = 0;
    while current_index < minified_input.len() {
        let remainder: &str = &minified_input[current_index..];
        let mut value: Option<Token> = None;
        if remainder.starts_with("\n") {
            line_counter += 1;
            current_index += 2;
            continue;
        }
        for (regex_pattern, token_type) in &map {
            if let Some(m) = regex_pattern.find(remainder) {
                if m.start() != 0 {continue;}
                value = Some(Token {
                    value: m.as_str().to_string(),
                    token_type: token_type.clone(),
                    start: m.start(),
                    end: m.end(),
                    line: line_counter
                });
                current_index += m.len();
                break;
            }
        }
        if value.is_none() {
            tokens.push(Token {
                value: remainder.to_string(),
                token_type: TokenType::Unknown,
                start: current_index,
                end: minified_input.len(),
                line: line_counter
            });
            break;
        }
        tokens.push(value.unwrap());
    }
    tokens
}