use crate::defs::token::{TOKEN_REGEX_MAP, Token, TokenType};
use regex::{Captures, Match};

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
                    .is_none_or(|(old_m, _)| m.end() >= old_m.get(0).unwrap().end());

                if m.start() == 0 && is_better_match {
                    best_capture_group = Some((captures, token_type));
                }
            }
        }

        match best_capture_group {
            Some((captures, token_type)) => {
                let m: Match = captures.get(0).unwrap();
                if token_type == &TokenType::Comment {
                    current_index += m.len();
                    continue;
                }
                let first_capture: Option<Match> = captures.get(1);
                let mut value: &str = m.as_str();
                if let Some(capture) = first_capture {
                    value = capture.as_str();
                }
                tokens.push(Token {
                    value,
                    token_type: token_type.clone(),
                    start: current_index,
                    end: current_index + m.end(),
                    line: line_counter,
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
                    line: line_counter,
                });
                break;
            }
        }
    }
    tokens
}
