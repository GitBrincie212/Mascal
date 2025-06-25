use mascal::defs::token::{Token, TokenType};
use mascal::lexer::tokenize;

#[test]
fn test_unknown() {
    let inputs = vec![
        ("Hello@gmail.com", vec![TokenType::Identifier]),
        ("Nice Job!", vec![TokenType::Identifier, TokenType::Identifier]),
        ("§ Paragraph 1", vec![]),
        ("## HEADER 2", vec![]),
        ("H$llo", vec![TokenType::Identifier]),
        ("0E 61\\", vec![TokenType::IntegerLiteral, TokenType::Identifier, TokenType::IntegerLiteral]),
    ];
    for (input, tok_types) in inputs {
        let tokens: Vec<Token> = tokenize(input);
        for (index, token) in tokens[..tok_types.len()].iter().enumerate() {
            assert_eq!(token.token_type, tok_types[index]);
        }
        assert_eq!(tokens.last().unwrap().token_type, TokenType::Unknown);
    }
}