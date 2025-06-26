use mascal::defs::token::{Token, TokenType};
use mascal::lexer::tokenize;

#[test]
fn test_integer_numbers() {
    let inputs: Vec<&str> = vec!["1234", "901", "010", "1234567890", "03"];
    for input in inputs {
        let tokens: Vec<Token> = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::IntegerLiteral);
        assert_eq!(tokens[0].value, input);
        assert_eq!(tokens[0].line, 0);
        assert_eq!(tokens[0].start, 0);
    }
}

#[test]
fn test_float_numbers() {
    let inputs: Vec<&str> = vec![
        "1.2345", 
        ".3848193", 
        "2394.0", 
        "0.3948291", 
        "234213."
    ];
    for input in inputs {
        let tokens: Vec<Token> = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::FloatLiteral);
        assert_eq!(tokens[0].value, input);
        assert_eq!(tokens[0].line, 0);
        assert_eq!(tokens[0].start, 0);
    }
}

#[test]
fn test_in_mix_numbers() {
    let inputs: Vec<(&str, TokenType, &str)> = vec![
        ("0301000  ", TokenType::IntegerLiteral, "0301000"),
        ("  1.2345 // cool", TokenType::FloatLiteral, "1.2345"),
        (".3848193", TokenType::FloatLiteral, ".3848193"),
        ("0101000", TokenType::IntegerLiteral, "0101000"),
        ("2394.0 ", TokenType::FloatLiteral, "2394.0"),
        ("3941     // ?", TokenType::IntegerLiteral, "3941"),
        ("0.3948291 // TYPEOF 3",TokenType::FloatLiteral, "0.3948291"),
        (" 234213.", TokenType::FloatLiteral, "234213."),
        ("239456 // 3.341", TokenType::IntegerLiteral, "239456"),
    ];
    for (input, expected, expected_input) in inputs {
        let tokens: Vec<Token> = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, expected);
        assert_eq!(tokens[0].value, expected_input);
        assert_eq!(tokens[0].line, 0);
        assert_eq!(
            tokens[0].start, 
            input.chars()
                .position(|x| !x.is_whitespace())
                .unwrap_or(0)
        );
    }
}