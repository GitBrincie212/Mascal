use logos::{Lexer, Logos, Span};
use crate::defs::token::{Token, TokenType};

pub fn tokenize(input: &str) -> Result<Vec<Token>, (Span, usize, &str)> {
    let mut lexer: Lexer<TokenType> = TokenType::lexer(input);
    let mut tokens: Vec<Token> = Vec::new();
    lexer.extras = 0;

    while let Some(kind) = lexer.next() {
        let span: Span = lexer.span();
        let line: usize = lexer.extras;
        let value: &str = match kind {
            Ok(TokenType::StringLiteral) => {
                &input[span.start + 1..span.end - 1]
            }

            Err(_) => {
                let val: &str = &input[span.clone()];
                return Err((span, line, val));
            }

            _ => {&input[span.clone()]}
        };
        let unwrapped_kind: TokenType = kind.unwrap();
        tokens.push(Token {
            token_type: unwrapped_kind,
            start: span.start,
            line,
            value
        });
    }

    Ok(tokens)
}
