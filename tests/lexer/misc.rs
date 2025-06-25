use mascal::defs::token::{Token, TokenType};
use mascal::lexer::tokenize;

macro_rules! test_stray_token_repetitions {
    ($fn_name: ident, $token_char: expr, $token_type: expr) => {
        #[test]
        fn $fn_name() {
            for size in 1usize..10usize {
                let input: &str = &$token_char.repeat(size); 
                let tokens: Vec<Token> = tokenize(input);
                assert_eq!(tokens.len(), size);
                assert_eq!(tokens.iter().all(|x| x.token_type == $token_type), true);
            }
        }
    };
}

test_stray_token_repetitions!(test_colon,  ":", TokenType::Colon);
test_stray_token_repetitions!(test_question_mark,  "?", TokenType::QuestionMark);
test_stray_token_repetitions!(test_semicolon,  ";", TokenType::Semicolon);
test_stray_token_repetitions!(test_return_indicator,  "->", TokenType::ReturnIndicator);
test_stray_token_repetitions!(test_variable_init,  "<-", TokenType::VariableInitializer);
test_stray_token_repetitions!(test_comma,  ",", TokenType::Comma);