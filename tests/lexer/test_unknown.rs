use logos::Span;
use rstest::rstest;
use mascal::defs::token::{Token};
use mascal::lexer::tokenize;

#[rstest(
    input, start, expected,
    case("Hello@gmail.com", 5, "@"),
    case("Nice Job!", 8, "!"),
    case("§ Paragraph 1", 0, "§"),
    case("## HEADER 2", 0, "#"),
    case("H$llo", 1, "$"),
    case("0E 61\\", 5, "\\"),
)]
fn test_unknown(input: &str, start: usize, expected: &str) {
    let tokens: Result<Vec<Token>, (Span, usize, &str)> = tokenize(input);
    assert_eq!(tokens.is_err(), true);
    let Err((span, line, val)) = tokens else {unreachable!()};
    assert_eq!(line, 0);
    assert_eq!(span.start, start);
    assert_eq!(val, expected);
}