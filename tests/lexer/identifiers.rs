use mascal::defs::token::{Token, TokenType};
use mascal::lexer::tokenize;

#[test]
fn test_identifiers_without_whitespaces() {
    let input: &str = "AcCeT_3";
    let tokens: Vec<Token> = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[0].value, input);
}

#[test]
fn test_identifiers_with_spaces() {
    let input: Vec<&str> = vec!["_AbE_", "FOM_", "bAr_3"];
    let char_count: Vec<usize> = input.iter().map(|x| x.len()).collect();
    let stringified: String = input.join(" ");
    let tokens: Vec<Token> = tokenize(stringified.as_str());
    for (index, token) in tokens.iter().enumerate() {
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, input[index]);
        assert_eq!(token.start, index + char_count[..index].iter().sum::<usize>());
    }
}

#[test]
fn test_identifiers_with_newlines() {
    let input: Vec<&str> = vec!["a3_c", "_D2FPco", "dko2kcoc"];
    let stringified: String = input.join("\n");
    let tokens: Vec<Token> = tokenize(stringified.as_str());
    for (index, token) in tokens.iter().enumerate() {
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, input[index]);
        assert_eq!(token.line, index);
    }
}

#[test]
fn test_identifiers_with_combination() {
    for spaces in 0usize..10usize {
        let input: Vec<&str> = vec!["okcCCcjI___3039_", "cjn23vBUEhu", "P1Kd0iCV20i"];
        let char_count: Vec<usize> = input.iter().map(|x| x.len()).collect();
        let sep: String = format!("{}\n", " ".repeat(spaces).as_str());
        let stringified: String = input.join(sep.as_str());
        let tokens: Vec<Token> = tokenize(stringified.as_str());
        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, TokenType::Identifier);
            assert_eq!(token.value, input[index]);
            assert_eq!(token.line, index);
            assert_eq!(token.start, (index * sep.len()) + char_count[..index].iter().sum::<usize>());
        }
    }
}

#[test]
fn test_identifiers_with_combination2() {
    for spaces in 0usize..10usize {
        let input: Vec<&str> = vec!["CMCMWM29", "CCNCNMP", "__CMCMMWmxmwkcj_29"];
        let char_count: Vec<usize> = input.iter().map(|x| x.len()).collect();
        let sep: String = format!("{}\n{}", " ".repeat(spaces).as_str(), "\t".repeat(spaces).as_str());
        let stringified: String = input.join(sep.as_str());
        let tokens: Vec<Token> = tokenize(stringified.as_str());
        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, TokenType::Identifier);
            assert_eq!(token.value, input[index]);
            assert_eq!(token.line, index);
            assert_eq!(token.start, (index * sep.len()) + char_count[..index].iter().sum::<usize>());
        }
    }
}

#[test]
fn test_identifiers_with_combination3() {
    for spaces in 0usize..10usize {
        let input: Vec<&str> = vec!["wjcnWXEUHew03", "cl3cmiji9", "__3ckqnc3"];
        let char_count: Vec<usize> = input.iter().map(|x| x.len()).collect();
        let sep: String = format!("{}\n{}", "\t ".repeat(spaces).as_str(), " \t".repeat(spaces).as_str());
        let stringified: String = input.join(sep.as_str());
        let tokens: Vec<Token> = tokenize(stringified.as_str());
        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, TokenType::Identifier);
            assert_eq!(token.value, input[index]);
            assert_eq!(token.line, index);
            assert_eq!(token.start, (index * sep.len()) + char_count[..index].iter().sum::<usize>());
        }
    }
}

#[test]
fn test_identifiers_with_combination4() {
    for spaces in 1usize..10usize {
        let input: Vec<&str> = vec!["nwjnbbwj81", "cl3cmiji9", "fig33ijv3", "vi3uvj2n2"];
        let char_count: Vec<usize> = input.iter().map(|x| x.len()).collect();
        let sep: String = format!("{}{}", "\t\n".repeat(spaces).as_str(), " \t".repeat(spaces).as_str());
        let stringified: String = input.join(sep.as_str());
        let tokens: Vec<Token> = tokenize(stringified.as_str());
        for (index, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, TokenType::Identifier);
            assert_eq!(token.value, input[index]);
            assert_eq!(token.line, index * (sep.lines().count() - 1));
            assert_eq!(token.start, (index * sep.len()) + char_count[..index].iter().sum::<usize>());
        }
    }
}