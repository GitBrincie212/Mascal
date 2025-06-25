use mascal::defs::token::{Token, TokenType};
use mascal::lexer::tokenize;

mod identifiers;
mod numbers;
mod groups;
mod practical;

#[test]
fn mixed_identifiers_and_numbers() {
    let inputs: Vec<(&str, Vec<TokenType>, Vec<&str>)> = vec![
        ("cc0301000", vec![TokenType::Identifier], vec!["cc0301000"]),
        ("1.2345co2kc29", vec![TokenType::FloatLiteral, TokenType::Identifier], vec!["1.2345", "co2kc29"]),
        (".45co2kc29", vec![TokenType::FloatLiteral, TokenType::Identifier], vec![".45", "co2kc29"]),
        ("cj2j3.034", vec![TokenType::Identifier, TokenType::FloatLiteral], vec!["cj2j3", ".034"]),
        (
            "234cj2j3.9214",
            vec![TokenType::IntegerLiteral, TokenType::Identifier, TokenType::FloatLiteral],
            vec!["234", "cj2j3", ".9214"]
        ),
        (
            ".234roj2cg39214",
            vec![TokenType::FloatLiteral, TokenType::Identifier],
            vec![".234", "roj2cg39214"]
        ),
    ];
    for (input, expected_tok_types, expected_values) in inputs {
        let tokens: Vec<Token> = tokenize(input);
        assert_eq!(tokens.len(), expected_tok_types.len());
        for (index, tok_type) in expected_tok_types.iter().enumerate() {
            assert_eq!(tokens[index].value, expected_values[index]);
            assert_eq!(tokens[index].token_type, *tok_type);
            assert_eq!(tokens[index].line, 0);
            assert_eq!(tokens[index].start, expected_values[..index].iter().map(|x| x.len()).sum());
        }
    }
}