use rstest::rstest;
use mascal::defs::token::{Token, TokenType};
use mascal::lexer::tokenize;

mod identifiers;
mod numbers;
mod groups;
mod practical;
mod misc;
mod types;
mod keywords;
mod comments;
mod test_unknown;

#[macro_export]
macro_rules! test_individual_token {
    ($fn_name: ident, $token_char: expr, $token_type: expr) => {
        #[test]
        fn $fn_name() {
            let input_cases: Vec<String> = vec![
                $token_char.to_string(),
                $token_char.to_uppercase(), 
                $token_char.to_lowercase()
            ];
            for input in input_cases.iter() {
                let tokens: Vec<Token> = tokenize(input).unwrap();
                assert_eq!(tokens.len(), 1);
                assert_eq!(tokens[0].token_type, $token_type);
            }
        }
    };
}

#[rstest(
    input, expected_tok_types, expected_values,
    case("cc0301000", vec![TokenType::Identifier], vec!["cc0301000"]),
    case("1.2345co2kc29", vec![TokenType::FloatLiteral, TokenType::Identifier], vec!["1.2345", "co2kc29"]),
    case("cj2j3.034", vec![TokenType::Identifier, TokenType::FloatLiteral], vec!["cj2j3", ".034"]),
    case("234cj2j3.9214",
        vec![TokenType::IntegerLiteral, TokenType::Identifier, TokenType::FloatLiteral],
        vec!["234", "cj2j3", ".9214"]
    ),
    case(".234roj2cg39214",
        vec![TokenType::FloatLiteral, TokenType::Identifier],
        vec![".234", "roj2cg39214"]
    ),
)]
fn mixed_identifiers_and_numbers(input: &str, expected_tok_types: Vec<TokenType>, expected_values: Vec<&str>) {
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), expected_tok_types.len());
    for (index, tok_type) in expected_tok_types.iter().enumerate() {
        assert_eq!(tokens[index].value, expected_values[index]);
        assert_eq!(tokens[index].token_type, *tok_type);
        assert_eq!(tokens[index].line, 0);
        assert_eq!(
            tokens[index].start,
            expected_values[..index]
                .iter()
                .map(|x| x.len())
                .sum::<usize>()
        );
    }
}