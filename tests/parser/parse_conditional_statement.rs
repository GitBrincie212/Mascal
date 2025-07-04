use rstest::rstest;
use mascal::ast::AbstractSyntaxTree;
use mascal::defs::blocks::ScopedBlocks;
use mascal::defs::errors::{MascalError, MascalErrorType};
use mascal::defs::statements::MascalStatement;
use crate::{define_program_boilerplate, run_parsing};

#[rstest(
    input, condition_part,
    case("IF a = b {c <- 0;}", vec![Some("a = b")]),
    case("IF a = b {c <- TRUE;} ELSE {b <- FALSE;}", vec![Some("a = b"), None]),
    case("IF a > b {c <- 1;} ELSE IF a < b {c <- 2;} ELSE {c <- 3;}", vec![Some("a > b"), Some("a < b"), None]),
    case("IF a > b {c <- 1;} ELIF a < b {c <- 2;} ELSE {c <- 3;}", vec![Some("a > b"), Some("a < b"), None]),
    case("IF FALSE {c <- 1;} ELSE {c <- 2;}", vec![Some("FALSE"), None]),
    case("IF FALSE {c <- 1;} ELSE IF TRUE {c <- 2;}", vec![Some("FALSE"), Some("TRUE")]),
    case("IF FALSE {c <- 1;} ELIF TRUE {c <- 2;}", vec![Some("FALSE"), Some("TRUE")]),
    case("IF FALSE {c <- 1;} ELIF a > 3 {c <- 2;} ELIF a < 3 {c <- 3;}", vec![Some("FALSE"), Some("a > 3"), Some("a < 3")]),
    case(
        "IF FALSE OR 1 = 1 {c <- 1;} ELIF b = a AND a = b {c <- 2;} ELIF 2 < 3 {c <- 3;}", 
        vec![Some("FALSE OR 1 = 1"), Some("b = a AND a = b"), Some("2 < 3")]
    ),
)]
fn test_correct_parsing(input: &str, condition_part: Vec<Option<&str>>) {
    let input: String = define_program_boilerplate!(
        Vec::<String>::new(),
        vec![ input ]
    );
    let ast: AbstractSyntaxTree = run_parsing!(input.as_str()).unwrap();
    assert_eq!(ast.blocks.len(), 1);
    let ScopedBlocks::Program(exec) = &ast.blocks[0] else {unreachable!()};
    assert_eq!(exec.body.len(), 1);
    match &exec.body[0] {
        MascalStatement::ConditionalStatement(branches) => {
            assert_eq!(branches.len(), condition_part.len());
            for (branch, expected) in branches.iter().zip(condition_part.iter()) {
                assert_eq!(branch.statements.len(), 1);
                if let Some(unwrapped_expected) = *expected {
                    let input_expect: String = define_program_boilerplate!(
                        Vec::<String>::new(),
                        vec![ format!("{unwrapped_expected};") ]
                    );
                    
                    let ast: AbstractSyntaxTree = run_parsing!(input_expect.as_str()).unwrap();
                    let ScopedBlocks::Program(exec) = &ast.blocks[0] else {unreachable!()};
                    let MascalStatement::ExpressionStatement(expr) = &exec.body[0] else {unreachable!()};
                    assert_eq!(branch.condition.as_ref().unwrap(), expr);
                    continue;
                }
                assert_eq!(branch.condition.is_none(), true);
            }
        }
        _ => panic!("The statement is not a conditional one")
    }
}

#[rstest(
    input, message,
    case("IF {c <- 0;}", "Expected an expression to parse but got nothing"),
    case("IF ; {c <- 0;}", "Unexpected characters in primary expression: \";\""),
    case("IF TRUE c <- 0;", "Unexpected character sequences found in a supposed expression"),
    case("IF TRUE {c <- 0;", "DEFINE_PROGRAM block not properly closed"),
    case("IF TRUE c <- 0;}", "Unexpected character sequences found in a supposed expression"),
    case("IF TRUE a + b;}", "Unexpected character sequences found in a supposed expression"),
    case("IF TRUE {{c <- 0;}", "DEFINE_PROGRAM block not properly closed"),
    case("IF TRUE {{c <- 0;}", "DEFINE_PROGRAM block not properly closed"),
    case("IF FALSE {} ELIF {c <- 0;}", "Expected an expression to parse but got nothing"),
    case("IF FALSE {} ELIF ; {c <- 0;}", "Unexpected characters in primary expression: \";\""),
    case("IF FALSE {} ELIF TRUE c <- 0;", "Unexpected character sequences found in a supposed expression"),
    case("IF FALSE {} ELIF TRUE {c <- 0;", "DEFINE_PROGRAM block not properly closed"),
    case("IF FALSE {} ELIF TRUE c <- 0;}", "Unexpected character sequences found in a supposed expression"),
    case("IF FALSE {} ELIF TRUE a + b;}", "Unexpected character sequences found in a supposed expression"),
    case("IF FALSE {} ELIF TRUE {{c <- 0;}", "DEFINE_PROGRAM block not properly closed"),
    case("IF FALSE {} ELIF TRUE {{c <- 0;}", "DEFINE_PROGRAM block not properly closed"),
    case("IF FALSE {} ELIF FALSE {} ELIF {c <- 0;}", "Expected an expression to parse but got nothing"),
    case("IF FALSE {} ELIF FALSE {} ELIF ; {c <- 0;}", "Unexpected characters in primary expression: \";\""),
    case("IF FALSE {} ELIF FALSE {} ELIF TRUE c <- 0;", "Unexpected character sequences found in a supposed expression"),
    case("IF FALSE {} ELIF FALSE {} ELIF TRUE {c <- 0;", "DEFINE_PROGRAM block not properly closed"),
    case("IF FALSE {} ELIF FALSE {} ELIF TRUE c <- 0;}", "Unexpected character sequences found in a supposed expression"),
    case("IF FALSE {} ELIF FALSE {} ELIF TRUE a + b;}", "Unexpected character sequences found in a supposed expression"),
    case("IF FALSE {} ELIF FALSE {} ELIF TRUE {{c <- 0;}", "DEFINE_PROGRAM block not properly closed"),
    case("IF FALSE {} ELIF FALSE {} ELIF TRUE {{c <- 0;}", "DEFINE_PROGRAM block not properly closed"),
    case("IF FALSE {} ELIF FALSE {} ELSE {{c <- 0;}", "DEFINE_PROGRAM block not properly closed"),
    case("IF FALSE {} ELIF FALSE {} ELSE TRUE c <- 0;", "Expected a opening brace for a conditional branch"),
    case("IF FALSE {} ELIF FALSE {} ELSE {c <- 0;", "DEFINE_PROGRAM block not properly closed"),
    case("IF FALSE {} ELIF FALSE {} ELSE c <- 0;}", "Expected a opening brace for a conditional branch"),
)]
fn test_incorrect_parsing(input: &str, message: &str) {
    let input: String = define_program_boilerplate!(
        Vec::<String>::new(),
        vec![ input ]
    );
    let ast: Result<AbstractSyntaxTree, MascalError> = run_parsing!(input.as_str());
    assert!(
        matches!(ast.as_ref().unwrap_err(),
                MascalError {
                    error_type,
                    source,
                    ..
                } if *error_type == MascalErrorType::ParserError
                && source == message
            ),
        "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
        &ast, MascalErrorType::ParserError, message
    );
}

#[rstest(
    input, message,
    case("IF a = b {c <- 0;} ELSE {} ELIF TRUE {}", "Cannot supply an ELIF condition after an ELSE condition without opening a new IF condition"),
    case("ELSE {c <- 0;}", "Expected an IF condition before this ELSE condition"),
    case("ELIF a = b {c <- 0;}", "Expected an IF condition before this ELIF condition"),
)]
fn test_incorrect_parsing2(input: &str, message: &str) {
    let input: String = define_program_boilerplate!(
        Vec::<String>::new(),
        vec![ input ]
    );
    let ast: Result<AbstractSyntaxTree, MascalError> = run_parsing!(input.as_str());
    assert!(
        matches!(ast.as_ref().unwrap_err(),
                MascalError {
                    error_type,
                    source,
                    ..
                } if *error_type == MascalErrorType::ParserError
                && source == message
            ),
        "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
        &ast, MascalErrorType::ParserError, message
    );
}