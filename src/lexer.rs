use crate::defs::token::{Token, TokenType, TOKEN_REGEX_MAP};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_index: usize = 0;
    let mut line_counter: usize = 0;
    while current_index < input.len() {
        let remainder: &str = &input[current_index..];
        let mut value: Option<Token> = None;

        if remainder.starts_with("\n") {
            line_counter += 1;
            current_index += "\n".len();
            continue;
        } else if remainder.starts_with(" ") {
            current_index += remainder
                .bytes()
                .take_while(|b| b.is_ascii_whitespace() && *b != b'\n')
                .count();
            continue;
        }

        for (regex_pattern, token_type) in TOKEN_REGEX_MAP.iter() {
            if let Some(m) = regex_pattern.find(remainder) {
                if m.start() != 0 {continue;}
                value = Some(Token {
                    value: m.as_str(),
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
        tokens.push(value.unwrap());
    }
    tokens
}