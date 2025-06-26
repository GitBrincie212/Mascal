use mascal::defs::token::{Token, TokenType};
use mascal::lexer::tokenize;

#[test]
fn test_individual_groups() {
    let groups = vec![
        ("(", ")", TokenType::OpenParen, TokenType::CloseParen),
        ("[", "]", TokenType::OpenBracket, TokenType::CloseBracket),
        ("{", "}", TokenType::OpenBrace, TokenType::CloseBrace),
        ("<<", ">>", TokenType::OpenDynamicArray, TokenType::CloseDynamicArray)
    ];
    for (group_open_char, group_close_char, group_tok_open, group_tok_close) in groups {
        let inputs: Vec<(&str, Vec<&TokenType>)> = vec![
            (
                "() () ",
                vec![
                    &group_tok_open,
                    &group_tok_close,
                    &group_tok_open,
                    &group_tok_close
                ]
            ),
            (
                "( () )",
                vec![
                    &group_tok_open,
                    &group_tok_open,
                    &group_tok_close,
                    &group_tok_close
                ]
            ),
            (
                "() ( ( ) ) ",
                vec![
                    &group_tok_open,
                    &group_tok_close,
                    &group_tok_open,
                    &group_tok_open,
                    &group_tok_close,
                    &group_tok_close
                ]
            ),
            (
                "( ( ) ( ) )",
                vec![
                    &group_tok_open,
                    &group_tok_open,
                    &group_tok_close,
                    &group_tok_open,
                    &group_tok_close,
                    &group_tok_close
                ]
            ),
            (
                "(()(()))",
                vec![
                    &group_tok_open,
                    &group_tok_open,
                    &group_tok_close,
                    &group_tok_open,
                    &group_tok_open,
                    &group_tok_close,
                    &group_tok_close,
                    &group_tok_close
                ]
            )
        ];
        for (input, token_types) in inputs {
            let transformed_input: String = input.replace("(", group_open_char)
                .replace(")", group_close_char);
            let tokens: Vec<Token> = tokenize(transformed_input.as_str()).unwrap();
            assert_eq!(tokens.len(), token_types.len());
            for (idx, tok) in tokens.iter().enumerate() {
                assert_eq!(tok.token_type, *token_types[idx]);
                if tok.token_type == TokenType::OpenParen
                    || tok.token_type == TokenType::OpenBracket
                    || tok.token_type == TokenType::OpenBrace
                    || tok.token_type == TokenType::OpenDynamicArray {
                    assert_eq!(tok.value, group_open_char);
                }
                if tok.token_type == TokenType::CloseParen
                    || tok.token_type == TokenType::CloseBracket
                    || tok.token_type == TokenType::CloseBrace
                    || tok.token_type == TokenType::CloseDynamicArray {
                    assert_eq!(tok.value, group_close_char);
                }
            }
        }
    }
}

#[test]
fn test_mixed_groups() {
    let inputs: Vec<(&str, Vec<TokenType>)> = vec![
        (" ({} )[ ] ", vec![
            TokenType::OpenParen,
            TokenType::OpenBrace,
            TokenType::CloseBrace,
            TokenType::CloseParen,
            TokenType::OpenBracket,
            TokenType::CloseBracket,
        ]),
        ("<< [{}()]>> [({<<>> })]", vec![
            TokenType::OpenDynamicArray,
            TokenType::OpenBracket,
            TokenType::OpenBrace,
            TokenType::CloseBrace,
            TokenType::OpenParen,
            TokenType::CloseParen,
            TokenType::CloseBracket,
            TokenType::CloseDynamicArray,
            TokenType::OpenBracket,
            TokenType::OpenParen,
            TokenType::OpenBrace,
            TokenType::OpenDynamicArray,
            TokenType::CloseDynamicArray,
            TokenType::CloseBrace,
            TokenType::CloseParen,
            TokenType::CloseBracket,
        ]),
        ("<<{ [()] }>>", vec![
            TokenType::OpenDynamicArray,
            TokenType::OpenBrace,
            TokenType::OpenBracket,
            TokenType::OpenParen,
            TokenType::CloseParen,
            TokenType::CloseBracket,
            TokenType::CloseBrace,
            TokenType::CloseDynamicArray,
        ]),
        ("<< >> {}[]( )", vec![
            TokenType::OpenDynamicArray,
            TokenType::CloseDynamicArray,
            TokenType::OpenBrace,
            TokenType::CloseBrace,
            TokenType::OpenBracket,
            TokenType::CloseBracket,
            TokenType::OpenParen,
            TokenType::CloseParen,
        ]),
    ];
    for (input, token_types) in inputs {
        let tokens: Vec<Token> = tokenize(input).unwrap();
        assert_eq!(tokens.len(), token_types.len());
        for (idx, tok) in tokens.iter().enumerate() {
            assert_eq!(tok.token_type, token_types[idx]);
        }
    }
}

#[test]
fn test_nonequal_groups() {
    let inputs: Vec<(&str, Vec<TokenType>)> = vec![
        ("[[[ ] ]", vec![
            TokenType::OpenBracket,
            TokenType::OpenBracket,
            TokenType::OpenBracket,
            TokenType::CloseBracket,
            TokenType::CloseBracket,
        ]),
        ("{[ ] })", vec![
            TokenType::OpenBrace,
            TokenType::OpenBracket,
            TokenType::CloseBracket,
            TokenType::CloseBrace,
            TokenType::CloseParen,
        ]),
        (" ( [[ ]}", vec![
            TokenType::OpenParen,
            TokenType::OpenBracket,
            TokenType::OpenBracket,
            TokenType::CloseBracket,
            TokenType::CloseBrace,
        ]),
        ("]] ( [ [>> }>>[[", vec![
            TokenType::CloseBracket,
            TokenType::CloseBracket,
            TokenType::OpenParen,
            TokenType::OpenBracket,
            TokenType::OpenBracket,
            TokenType::CloseDynamicArray,
            TokenType::CloseBrace,
            TokenType::CloseDynamicArray,
            TokenType::OpenBracket,
            TokenType::OpenBracket,
        ]),
    ];
    for (input, token_types) in inputs {
        let tokens: Vec<Token> = tokenize(input).unwrap();
        assert_eq!(tokens.len(), token_types.len());
        for (idx, tok) in tokens.iter().enumerate() {
            assert_eq!(tok.token_type, token_types[idx]);
        }
    }
}