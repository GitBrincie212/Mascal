use mascal::defs::token::{Token, TokenType};
use mascal::lexer::tokenize;

#[test]
fn test_expression_statement1() {
    let input: &str = "WRITE(\"Hello World\");";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].token_type, TokenType::OpenParen);
    assert_eq!(tokens[2].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[3].token_type, TokenType::CloseParen);
    assert_eq!(tokens[4].token_type, TokenType::Semicolon);
}

#[test]
fn test_arithmetic_expression() {
    let input: &str = "(a + 5) / (3 - b * INTEGER(2.5));";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 17);
    assert_eq!(tokens[0].token_type, TokenType::OpenParen);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Plus);
    assert_eq!(tokens[3].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[4].token_type, TokenType::CloseParen);
    assert_eq!(tokens[5].token_type, TokenType::Division);
    assert_eq!(tokens[6].token_type, TokenType::OpenParen);
    assert_eq!(tokens[7].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[8].token_type, TokenType::Minus);
    assert_eq!(tokens[9].token_type, TokenType::Identifier);
    assert_eq!(tokens[10].token_type, TokenType::Asterisk);
    assert_eq!(tokens[11].token_type, TokenType::Integer);
    assert_eq!(tokens[12].token_type, TokenType::OpenParen);
    assert_eq!(tokens[13].token_type, TokenType::FloatLiteral);
    assert_eq!(tokens[14].token_type, TokenType::CloseParen);
    assert_eq!(tokens[15].token_type, TokenType::CloseParen);
    assert_eq!(tokens[16].token_type, TokenType::Semicolon);
}

#[test]
fn test_variable_init_statement() {
    let input: &str = "a <- b[3] + LEN(\"Hello World\");";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 12);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].token_type, TokenType::VariableInitializer);
    assert_eq!(tokens[2].token_type, TokenType::Identifier);
    assert_eq!(tokens[3].token_type, TokenType::OpenBracket);
    assert_eq!(tokens[4].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[5].token_type, TokenType::CloseBracket);
    assert_eq!(tokens[6].token_type, TokenType::Plus);
    assert_eq!(tokens[7].token_type, TokenType::Identifier);
    assert_eq!(tokens[8].token_type, TokenType::OpenParen);
    assert_eq!(tokens[9].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[10].token_type, TokenType::CloseParen);
    assert_eq!(tokens[11].token_type, TokenType::Semicolon);
}

#[test]
fn test_controlflow_statement1() {
    let input: &str = "IF a = b { WRITE(a, b); SWAP(a, b); WRITE(a, b); }";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 27);
    assert_eq!(tokens[0].token_type, TokenType::If);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Equals);
    assert_eq!(tokens[3].token_type, TokenType::Identifier);
    assert_eq!(tokens[4].token_type, TokenType::OpenBrace);
    assert_eq!(tokens[5].token_type, TokenType::Identifier);
    assert_eq!(tokens[6].token_type, TokenType::OpenParen);
    assert_eq!(tokens[7].token_type, TokenType::Identifier);
    assert_eq!(tokens[8].token_type, TokenType::Comma);
    assert_eq!(tokens[9].token_type, TokenType::Identifier);
    assert_eq!(tokens[10].token_type, TokenType::CloseParen);
    assert_eq!(tokens[11].token_type, TokenType::Semicolon);
    assert_eq!(tokens[12].token_type, TokenType::Identifier);
    assert_eq!(tokens[13].token_type, TokenType::OpenParen);
    assert_eq!(tokens[14].token_type, TokenType::Identifier);
    assert_eq!(tokens[15].token_type, TokenType::Comma);
    assert_eq!(tokens[16].token_type, TokenType::Identifier);
    assert_eq!(tokens[17].token_type, TokenType::CloseParen);
    assert_eq!(tokens[18].token_type, TokenType::Semicolon);
    assert_eq!(tokens[19].token_type, TokenType::Identifier);
    assert_eq!(tokens[20].token_type, TokenType::OpenParen);
    assert_eq!(tokens[21].token_type, TokenType::Identifier);
    assert_eq!(tokens[22].token_type, TokenType::Comma);
    assert_eq!(tokens[23].token_type, TokenType::Identifier);
    assert_eq!(tokens[24].token_type, TokenType::CloseParen);
    assert_eq!(tokens[25].token_type, TokenType::Semicolon);
    assert_eq!(tokens[26].token_type, TokenType::CloseBrace);
}

#[test]
fn test_controlflow_statement2() {
    let input: &str = "IF a > b { \
                            WRITE(\"Bigger\"); \
                       } ELIF a < b { \
                            WRITE(\"Smaller\"); \
                       } ELSE { \
                            WRITE(\"Equal\"); \
                       }";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 30);
    assert_eq!(tokens[0].token_type, TokenType::If);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::GreaterThan);
    assert_eq!(tokens[3].token_type, TokenType::Identifier);
    assert_eq!(tokens[4].token_type, TokenType::OpenBrace);
    assert_eq!(tokens[5].token_type, TokenType::Identifier);
    assert_eq!(tokens[6].token_type, TokenType::OpenParen);
    assert_eq!(tokens[7].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[8].token_type, TokenType::CloseParen);
    assert_eq!(tokens[9].token_type, TokenType::Semicolon);
    assert_eq!(tokens[10].token_type, TokenType::CloseBrace);
    assert_eq!(tokens[11].token_type, TokenType::ElseIf);
    assert_eq!(tokens[12].token_type, TokenType::Identifier);
    assert_eq!(tokens[13].token_type, TokenType::LessThan);
    assert_eq!(tokens[14].token_type, TokenType::Identifier);
    assert_eq!(tokens[15].token_type, TokenType::OpenBrace);
    assert_eq!(tokens[16].token_type, TokenType::Identifier);
    assert_eq!(tokens[17].token_type, TokenType::OpenParen);
    assert_eq!(tokens[18].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[19].token_type, TokenType::CloseParen);
    assert_eq!(tokens[20].token_type, TokenType::Semicolon);
    assert_eq!(tokens[21].token_type, TokenType::CloseBrace);
    assert_eq!(tokens[22].token_type, TokenType::Else);
    assert_eq!(tokens[23].token_type, TokenType::OpenBrace);
    assert_eq!(tokens[24].token_type, TokenType::Identifier);
    assert_eq!(tokens[25].token_type, TokenType::OpenParen);
    assert_eq!(tokens[26].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[27].token_type, TokenType::CloseParen);
    assert_eq!(tokens[28].token_type, TokenType::Semicolon);
    assert_eq!(tokens[29].token_type, TokenType::CloseBrace);
}

#[test]
fn test_controlflow_statement3() {
    let input: &str = "WHILE a >= b {\
                            a2 <- PUSH(<<1, 2, 3>>, 4.6);
                            IF a <= b {
                            }
                       }";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 27);
    assert_eq!(tokens[0].token_type, TokenType::While);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::GreaterThanEqual);
    assert_eq!(tokens[3].token_type, TokenType::Identifier);
    assert_eq!(tokens[4].token_type, TokenType::OpenBrace);
    assert_eq!(tokens[5].token_type, TokenType::Identifier);
    assert_eq!(tokens[6].token_type, TokenType::VariableInitializer);
    assert_eq!(tokens[7].token_type, TokenType::Identifier);
    assert_eq!(tokens[8].token_type, TokenType::OpenParen);
    assert_eq!(tokens[9].token_type, TokenType::OpenDynamicArray);
    assert_eq!(tokens[10].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[11].token_type, TokenType::Comma);
    assert_eq!(tokens[12].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[13].token_type, TokenType::Comma);
    assert_eq!(tokens[14].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[15].token_type, TokenType::CloseDynamicArray);
    assert_eq!(tokens[16].token_type, TokenType::Comma);
    assert_eq!(tokens[17].token_type, TokenType::FloatLiteral);
    assert_eq!(tokens[18].token_type, TokenType::CloseParen);
    assert_eq!(tokens[19].token_type, TokenType::Semicolon);
    assert_eq!(tokens[20].token_type, TokenType::If);
    assert_eq!(tokens[21].token_type, TokenType::Identifier);
    assert_eq!(tokens[22].token_type, TokenType::LesserThanEqual);
    assert_eq!(tokens[23].token_type, TokenType::Identifier);
    assert_eq!(tokens[24].token_type, TokenType::OpenBrace);
    assert_eq!(tokens[25].token_type, TokenType::CloseBrace);
    assert_eq!(tokens[26].token_type, TokenType::CloseBrace);
}

#[test]
fn test_controlflow_statement4() {
    let input: &str = "FOR i FROM 1 TO 5 WITH_STEP 3 {\
                            WRITE(\"Hello World\");
                       }";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 15);
    assert_eq!(tokens[0].token_type, TokenType::For);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::From);
    assert_eq!(tokens[3].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[4].token_type, TokenType::To);
    assert_eq!(tokens[5].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[6].token_type, TokenType::WithStep);
    assert_eq!(tokens[7].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[8].token_type, TokenType::OpenBrace);
    assert_eq!(tokens[9].token_type, TokenType::Identifier);
    assert_eq!(tokens[10].token_type, TokenType::OpenParen);
    assert_eq!(tokens[11].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[12].token_type, TokenType::CloseParen);
    assert_eq!(tokens[13].token_type, TokenType::Semicolon);
    assert_eq!(tokens[14].token_type, TokenType::CloseBrace);
}

#[test]
fn test_throw_statement() {
    let input: &str = "THROW RuntimeError: \"here is a COOL MESSAGE\"";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Throw);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Colon);
    assert_eq!(tokens[3].token_type, TokenType::StringLiteral);
}

#[test]
fn test_continue_statement() {
    let input: &str = "CONTINUE;";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Continue);
    assert_eq!(tokens[1].token_type, TokenType::Semicolon);
}

#[test]
fn test_break_statement() {
    let input: &str = "BREAK;";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Break);
    assert_eq!(tokens[1].token_type, TokenType::Semicolon);
}

#[test]
fn test_logic_statement_expressions() {
    let input: &str = "9 = 3 AND NOT a % 3 = 0 OR b ^ 2 > 4;";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 17);
    assert_eq!(tokens[0].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[1].token_type, TokenType::Equals);
    assert_eq!(tokens[2].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[3].token_type, TokenType::And);
    assert_eq!(tokens[4].token_type, TokenType::Not);
    assert_eq!(tokens[5].token_type, TokenType::Identifier);
    assert_eq!(tokens[6].token_type, TokenType::Modulo);
    assert_eq!(tokens[7].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[8].token_type, TokenType::Equals);
    assert_eq!(tokens[9].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[10].token_type, TokenType::Or);
    assert_eq!(tokens[11].token_type, TokenType::Identifier);
    assert_eq!(tokens[12].token_type, TokenType::Exponentiation);
    assert_eq!(tokens[13].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[14].token_type, TokenType::GreaterThan);
    assert_eq!(tokens[15].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[16].token_type, TokenType::Semicolon);
}

#[test]
fn test_type_statement_expression() {
    let input: &str = "TYPE_CAST(\"abc\", INTEGER(FLOAT(3)));";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 13);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].token_type, TokenType::OpenParen);
    assert_eq!(tokens[2].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[3].token_type, TokenType::Comma);
    assert_eq!(tokens[4].token_type, TokenType::Integer);
    assert_eq!(tokens[5].token_type, TokenType::OpenParen);
    assert_eq!(tokens[6].token_type, TokenType::Float);
    assert_eq!(tokens[7].token_type, TokenType::OpenParen);
    assert_eq!(tokens[8].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[9].token_type, TokenType::CloseParen);
    assert_eq!(tokens[10].token_type, TokenType::CloseParen);
    assert_eq!(tokens[11].token_type, TokenType::CloseParen);
    assert_eq!(tokens[12].token_type, TokenType::Semicolon);
}

#[test]
fn test_mixed_statement_expression() {
    let input: &str = "(1 * TYPE_CAST(2.3, a)) = 2.3 AND NOT 3 % 3 = 0;";
    let tokens: Vec<Token> = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 20);
    assert_eq!(tokens[0].token_type, TokenType::OpenParen);
    assert_eq!(tokens[1].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[2].token_type, TokenType::Asterisk);
    assert_eq!(tokens[3].token_type, TokenType::Identifier);
    assert_eq!(tokens[4].token_type, TokenType::OpenParen);
    assert_eq!(tokens[5].token_type, TokenType::FloatLiteral);
    assert_eq!(tokens[6].token_type, TokenType::Comma);
    assert_eq!(tokens[7].token_type, TokenType::Identifier);
    assert_eq!(tokens[8].token_type, TokenType::CloseParen);
    assert_eq!(tokens[9].token_type, TokenType::CloseParen);
    assert_eq!(tokens[10].token_type, TokenType::Equals);
    assert_eq!(tokens[11].token_type, TokenType::FloatLiteral);
    assert_eq!(tokens[12].token_type, TokenType::And);
    assert_eq!(tokens[13].token_type, TokenType::Not);
    assert_eq!(tokens[14].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[15].token_type, TokenType::Modulo);
    assert_eq!(tokens[16].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[17].token_type, TokenType::Equals);
    assert_eq!(tokens[18].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[19].token_type, TokenType::Semicolon);
}