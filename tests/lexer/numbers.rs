use rstest::rstest;
use mascal::defs::token::{Token, TokenType};
use mascal::lexer::tokenize;

#[rstest(
    input,
    case("1234"),
    case("901"), 
    case("010"), 
    case("1234567890"), 
    case("03")
)]
fn test_integer_numbers(input: &str) {
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[0].value, input);
    assert_eq!(tokens[0].line, 0);
    assert_eq!(tokens[0].start, 0);
}

#[rstest(
    input,
    case("1.2345"),
    case(".3848193"),
    case("2394.0"),
    case("0.3948291"),
    case("234213.")
)]
fn test_float_numbers(input: &str) {
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::FloatLiteral);
    assert_eq!(tokens[0].value, input);
    assert_eq!(tokens[0].line, 0);
    assert_eq!(tokens[0].start, 0);
}

#[rstest(
    input, expected, expected_input,
    case("0301000  ", TokenType::IntegerLiteral, "0301000"),
    case("  1.2345 // cool", TokenType::FloatLiteral, "1.2345"),
    case(".3848193", TokenType::FloatLiteral, ".3848193"),
    case("0101000", TokenType::IntegerLiteral, "0101000"),
    case("2394.0 ", TokenType::FloatLiteral, "2394.0"),
    case("3941     // ?", TokenType::IntegerLiteral, "3941"),
    case("0.3948291 // TYPEOF 3",TokenType::FloatLiteral, "0.3948291"),
    case(" 234213.", TokenType::FloatLiteral, "234213."),
    case("239456 // 3.341", TokenType::IntegerLiteral, "239456"),
)]
fn test_in_mix_numbers(input: &str, expected: TokenType, expected_input: &str) {
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