use regex::{Captures, Match};
use crate::defs::token::{Token, TokenType, TOKEN_REGEX_MAP};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_index: usize = 0;
    let mut line_counter: usize = 0;

    while current_index < input.len() {
        let remainder: &str = &input[current_index..];
        
        if let Some(c) = remainder.chars().next() {
            if c == '\n' {
                line_counter += 1;
                current_index += 1;
                continue;
            } else if c.is_whitespace() {
                current_index += c.len_utf8();
                continue;
            }
        }

        let mut best_capture_group: Option<(Captures, &TokenType)> = None;
        for (regex_pattern, token_type) in TOKEN_REGEX_MAP.iter() {
            if let Some(captures) = regex_pattern.captures(remainder) {
                let m: Match = captures.get(0).unwrap();
                let is_better_match: bool = best_capture_group
                    .as_ref()
                    .map_or(true, |(old_m, _)| m.end() >= old_m.get(0).unwrap().end());
                
                if m.start() == 0 && is_better_match {
                    best_capture_group = Some((captures, token_type));
                }
            }
        }
        
        match best_capture_group {
            Some((captures, token_type)) => {
                let m: Match = captures.get(0).unwrap();
                let first_capture: Option<Match> = captures.get(1);
                tokens.push(Token {
                    value: if first_capture.is_some() {first_capture.unwrap().as_str()} else {m.as_str()},
                    token_type: token_type.clone(),
                    start: current_index,
                    end: current_index + m.end(),
                    line: line_counter
                });
                current_index += m.len();
            }
            
            None => {
                let next_newline: usize = remainder.find("\n").unwrap_or(remainder.len());
                tokens.push(Token {
                    value: &remainder[..next_newline],
                    token_type: TokenType::Unknown,
                    start: current_index,
                    end: next_newline,
                    line: line_counter
                });
                break;
            }
        }
    }
    tokens
}