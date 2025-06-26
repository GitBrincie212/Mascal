use logos::Span;
use mascal::defs::token::{Token};
use mascal::lexer::tokenize;

#[test]
fn test_unknown() {
    let inputs: Vec<(&str, usize, &str)> = vec![
        ("Hello@gmail.com", 5, "@"),
        ("Nice Job!", 8, "!"),
        ("§ Paragraph 1", 0, "§"),
        ("## HEADER 2", 0, "#"),
        ("H$llo", 1, "$"),
        ("0E 61\\", 5, "\\"),
    ];
    for (input, start, expected) in inputs {
        let tokens: Result<Vec<Token>, (Span, usize, &str)> = tokenize(input);
        assert_eq!(tokens.is_err(), true);
        let Err((span, line, val)) = tokens else {unreachable!()};
        assert_eq!(line, 0);
        assert_eq!(span.start, start);
        assert_eq!(val, expected);
    }
}