use rstest::rstest;
use mascal::ast::AbstractSyntaxTree;
use mascal::defs::blocks::ScopedBlocks;
use mascal::defs::errors::{MascalError, MascalErrorType};
use mascal::defs::expressions::MascalExpression;
use mascal::defs::statements::MascalStatement;
use crate::{define_program_boilerplate, run_parsing, unwrap_to_expression};

#[rstest(
    input, expected_exprs,
    case("FOR i FROM 1 TO 10 {c <- 3;}", ("i", "1", "10", "1")),
    case("FOR i FROM 1 / 2 TO 10 / 5 {c <- 3;}", ("i", "1 / 2", "10 / 5", "1")),
    case("FOR j FROM a + b TO 5 - 2 WITH_STEP 3 + 2 {c <- 3;}", ("j", "a + b", "5 - 2", "3 + 2")),
    case("FOR k FROM -2 + -2 TO -5 WITH_STEP -1 {c <- 3;}", ("k", "-2 + -2", "-5", "-1")),
)]
fn test_correct_parsing(input: &str, expected_exprs: (&str, &str, &str, &str)) {
    let input: String = define_program_boilerplate!(
        Vec::<String>::new(),
        vec![ input ]
    );
    let ast: AbstractSyntaxTree = run_parsing!(input.as_str()).unwrap();
    assert_eq!(ast.blocks.len(), 1);
    let ScopedBlocks::Program(exec) = &ast.blocks[0] else {unreachable!()};
    assert_eq!(exec.body.len(), 1);
    match &exec.body[0] {
        MascalStatement::For {
            variable,
            from,
            to,
            step,
            ..
        } => {
            assert_eq!(variable, expected_exprs.0);
            let expr: &MascalExpression = &unwrap_to_expression!(expected_exprs.1);
            assert_eq!(from, expr);
            let expr: &MascalExpression = &unwrap_to_expression!(expected_exprs.2);
            assert_eq!(to, expr);
            let expr: &MascalExpression = &unwrap_to_expression!(expected_exprs.3);
            assert_eq!(step, expr);
        }
        _ => panic!("The statement is not a for loop one")
    }
}

#[rstest(
    input, message,
    case("FOR", "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"),
    case("FOR;", "Expected a variable identifier to use but got \";\""),
    case("FOR {c <- 3;}", "Expected a variable identifier to use but got \"{\""),
    case("FOR i {c <- 3;}", "Expected FROM but got \"{\""),
    case("FOR i", "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"),
    case("FOR i FROM {c <- 3;}", "Unexpected characters in primary expression: \"{\""),
    case("FOR i FROM", "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"),
    case("FOR i FROM 2 {c <- 3;}", "Unexpected character sequences found in a supposed expression"),
    case("FOR i FROM 2 ", "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"),
    case("FOR i FROM 2 TO ", "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"),
    case("FOR i FROM 2 TO ;", "Unexpected characters in primary expression: \";\""),
    case("FOR i FROM 2 TO 5", "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"),
    case("FOR i FROM 2 TO 5;", "Unexpected character sequences found in a supposed expression"),
    case("FOR i FROM 2 TO 5 {", "DEFINE_PROGRAM block not properly closed"),
    case("FOR i FROM 2 TO 5 {{}", "DEFINE_PROGRAM block not properly closed"),
    case("FOR i FROM 2 TO 5 }", "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"),
    case("FOR i FROM 2 TO 5 WITH_STEP {", "DEFINE_PROGRAM block not properly closed"),
    case("FOR i FROM 2 TO 5 WITH_STEP {{}", "DEFINE_PROGRAM block not properly closed"),
    case("FOR i FROM 2 TO 5 WITH_STEP }", "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"),
    case("FOR i FROM 2 TO 5 WITH_STEP 3 {", "DEFINE_PROGRAM block not properly closed"),
    case("FOR i FROM 2 TO 5 WITH_STEP 3 {{}", "DEFINE_PROGRAM block not properly closed"),
    case("FOR i FROM 2 TO 5 WITH_STEP 3 }", "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"),
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