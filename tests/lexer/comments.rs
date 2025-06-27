use rstest::rstest;
use mascal::defs::token::{Token};
use mascal::lexer::tokenize;

#[rstest(
    input,
    case("// 1 + 3\t\t"),
    case("\t\n// COOL\n\n"),
    case("\n\t\t// Some documentation? blah, blah...")
)]
fn test_comments(input: &str) {
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 0);
}