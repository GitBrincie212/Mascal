use logos::{Lexer, Logos, Span};
use crate::defs::token::{Token, TokenType};

pub fn tokenize(input: &str) -> Result<Vec<Token>, (Span, usize, &str)> {
    let mut lexer: Lexer<TokenType> = TokenType::lexer(input);
    let mut tokens: Vec<Token> = Vec::new();
    lexer.extras = 0;

    while let Some(kind) = lexer.next() {
        let span: Span = lexer.span();
        let value: &str = &input[span.clone()];
        let line: usize = lexer.extras;
        if kind.is_err() {
            return Err((span, line, value));
        }
        tokens.push(Token {
            token_type: kind.unwrap(),
            start: span.start,
            line,
            value
        });
    }

    Ok(tokens)
}
