use rstest::rstest;
use mascal::ast::AbstractSyntaxTree;
use mascal::defs::blocks::ScopedBlocks;
use mascal::defs::errors::{MascalError, MascalErrorType};
use mascal::defs::statements::MascalStatement;
use crate::{define_program_boilerplate, run_parsing};

#[rstest(
    input, condition_part,
    case("WHILE a = b {c <- 0;}", "a = b"),
    case("WHILE a <= b AND 3 = 1 {c <- 0;}", "a <= b AND 3 = 1"),
    case("WHILE TRUE {c <- 0;}", "TRUE"),
    case("WHILE FALSE OR 2 = 2 {c <- 0;}", "FALSE OR 2 = 2"),
)]
fn test_correct_parsing(input: &str, condition_part: &str) {
    let input: String = define_program_boilerplate!(
        Vec::<String>::new(),
        vec![ input ]
    );
    let ast: AbstractSyntaxTree = run_parsing!(input.as_str()).unwrap();
    assert_eq!(ast.blocks.len(), 1);
    let ScopedBlocks::Program(exec) = &ast.blocks[0] else {unreachable!()};
    assert_eq!(exec.body.len(), 1);
    match &exec.body[0] {
        MascalStatement::While(branch) => {
            let input_expect: String = define_program_boilerplate!(
                Vec::<String>::new(),
                vec![ format!("{condition_part};") ]
            );
            
            let ast: AbstractSyntaxTree = run_parsing!(input_expect.as_str()).unwrap();
            let ScopedBlocks::Program(exec) = &ast.blocks[0] else {unreachable!()};
            let MascalStatement::ExpressionStatement(expr) = &exec.body[0] else {unreachable!()};
            assert_eq!(branch.condition.as_ref().unwrap(), expr);
        }
        _ => panic!("The statement is not a while one")
    }
}

#[rstest(
    input, message,
    case("WHILE {c <- 0;}", "Expected an expression to parse but got nothing"),
    case("WHILE ; {c <- 0;}", "Unexpected characters in primary expression: \";\""),
    case("WHILE TRUE c <- 0;", "Unexpected character sequences found in a supposed expression"),
    case("WHILE TRUE {c <- 0;", "DEFINE_PROGRAM block not properly closed"),
    case("WHILE TRUE c <- 0;}", "Unexpected character sequences found in a supposed expression"),
    case("WHILE TRUE {{c <- 0;}", "DEFINE_PROGRAM block not properly closed"),
    case("WHILE TRUE {{c <- 0;}", "DEFINE_PROGRAM block not properly closed"),
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