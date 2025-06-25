use mascal::defs::token::{Token};
use mascal::lexer::tokenize;

#[test]
fn test_comments() {
    let inputs = vec![
        "// 1 + 3\t\t",
        "\t\n// COOL\n\n",
        "\n\t\t// Some documentation? blah, blah..."
    ];
    for input in inputs {
        let tokens: Vec<Token> = tokenize(input);
        assert_eq!(tokens.len(), 0);
    }
}