use rstest::rstest;
use mascal::ast::AbstractSyntaxTree;
use mascal::defs::blocks::ScopedBlocks;
use mascal::defs::errors::{MascalError, MascalErrorType};
use mascal::defs::statements::MascalStatement;
use crate::{define_program_boilerplate, expect_error, run_parsing};

macro_rules! get_correct_error_inputs {
    () => {{
        vec![
            ("TypeError", MascalErrorType::TypeError),
            ("RuntimeError", MascalErrorType::RuntimeError),
            ("OverflowError",  MascalErrorType::OverflowError),
            ("UndefinedOperationError", MascalErrorType::UndefinedOperation),
            ("IndexError", MascalErrorType::IndexError),
            ("InputError", MascalErrorType::InputError),
            ("ArgumentError",  MascalErrorType::ArgumentError),
            ("ValueError", MascalErrorType::ValueError)
        ]
    }}
}

macro_rules! define_general_throw_statement_boilerplate {
    ($func: expr, $throws: expr) => {
        let errors: Vec<(&str, MascalErrorType)> = get_correct_error_inputs!();
        for (error_id, expected) in errors {
            for throw_case in $throws {
                $func(error_id, throw_case, &expected);
            }
        }
    };

    ($func: expr) => {
        define_general_throw_statement_boilerplate!($func, vec!["THROW", "throw", "Throw"]);
    }
}

#[rstest(
    error_id, expected,
    case("TypeError",           MascalErrorType::TypeError),
    case("RuntimeError",        MascalErrorType::RuntimeError),
    case("OverflowError",       MascalErrorType::OverflowError),
    case("UndefinedOperationError", MascalErrorType::UndefinedOperation),
    case("IndexError",          MascalErrorType::IndexError),
    case("InputError",          MascalErrorType::InputError),
    case("ArgumentError",       MascalErrorType::ArgumentError),
    case("ValueError",          MascalErrorType::ValueError),
)]
fn test_correct_parsing(error_id: &str, expected: MascalErrorType) {
    for throw_case in &["THROW", "throw", "Throw"] {
        let input = define_program_boilerplate!(
            Vec::<String>::new(),
            vec![ format!("{throw_case} {error_id}: \"test Test TEST!\";") ]
        );
        let ast = run_parsing!(input.as_str()).unwrap();

        assert!(matches!(&ast.blocks[0],
            ScopedBlocks::Program(exec) if exec.body.len() == 1
                && matches!(&exec.body[0],
                    MascalStatement::Throw { error_type, message }
                        if *error_type == expected && message == "test Test TEST!"
                )
        ));
    }
}

#[rstest(
    error_id,
    case("TypeErRor"),
    case("RuntImeError"),
    case("OVerfloWError"),
    case("UnDEfinedOPErationERRor"),
    case("IndexErro"),
    case("nputError"),
    case("ArgumentRror"),
    case("Value"),
)]
fn test_incorrect_parsing1(error_id: &str) {
    for throw_case in vec!["THROW", "throw", "Throw"] {
        let ast: Result<AbstractSyntaxTree, MascalError> = expect_error!(vec![
                format!("{throw_case} {error_id}: \"test Test TEST!\";")
            ]);
        assert!(
            matches!(ast.as_ref().unwrap_err(),
                    MascalError {
                        error_type,
                        source,
                        ..
                    } if *error_type == MascalErrorType::UndefinedErrorType
                        && source == "Use of an undefined usable error type in the throw statement (perhaps a typo?)"
                ),
            "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
            &ast, MascalErrorType::UndefinedErrorType, "test Test TEST!"
        );
    }
}

#[test]
fn test_incorrect_parsing2() {
    define_general_throw_statement_boilerplate!(|error_id, throw_case, _| {
        let ast: Result<AbstractSyntaxTree, MascalError> = expect_error!(vec![
                format!("{throw_case} {error_id} \"test Test TEST!\";")
        ]);
        assert!(
            matches!(ast.as_ref().unwrap_err(),
                MascalError {
                    error_type,
                    source,
                    ..
                } if *error_type == MascalErrorType::ParserError
                && source == "Expected a colon for the throw statement but got \"test Test TEST!\""
            ),
            "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
            &ast, MascalErrorType::ParserError, "Expected a colon for the throw statement but got <...>"
        );
    });
}

#[test]
fn test_incorrect_parsing3() {
    define_general_throw_statement_boilerplate!(|error_id, throw_case, _| {
        let ast: Result<AbstractSyntaxTree, MascalError> = expect_error!(vec![
                format!("{throw_case} {error_id} ;")
        ]);
        assert!(
            matches!(ast.as_ref().unwrap_err(),
                MascalError {
                    error_type,
                    source,
                    ..
                } if *error_type == MascalErrorType::ParserError
                && source == "Expected a colon for the throw statement but got nothing"
            ),
            "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
            &ast, MascalErrorType::ParserError, "Expected a colon for the throw statement but got nothing"
        );
    });
}

#[test]
fn test_incorrect_parsing4() {
    define_general_throw_statement_boilerplate!(|error_id, throw_case, _| {
        let ast: Result<AbstractSyntaxTree, MascalError> = expect_error!(vec![
                format!("{throw_case}: {error_id};")
        ]);
        assert!(
            matches!(ast.as_ref().unwrap_err(),
                MascalError {
                    error_type,
                    source,
                    ..
                } if *error_type == MascalErrorType::ParserError
                && source == "Expected a error type to throw but got \":\""
            ),
            "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
            &ast, MascalErrorType::ParserError, "Expected a error type to throw but got \":\""
        );
    });
}

#[test]
fn test_incorrect_parsing5() {
    define_general_throw_statement_boilerplate!(|error_id, throw_case, _| {
        let ast: Result<AbstractSyntaxTree, MascalError> = expect_error!(vec![
                format!("{throw_case} {error_id};")
        ]);
        assert!(
            matches!(ast.as_ref().unwrap_err(),
                MascalError {
                    error_type,
                    source,
                    ..
                } if *error_type == MascalErrorType::ParserError
                && source == "Expected a colon for the throw statement but got nothing"
            ),
            "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
            &ast, MascalErrorType::ParserError, "Expected a colon for the throw statement but got nothing"
        );
    });
}

#[test]
fn test_incorrect_parsing6() {
    define_general_throw_statement_boilerplate!(|error_id, throw_case, _| {
        let ast: Result<AbstractSyntaxTree, MascalError> = expect_error!(vec![
                format!("{throw_case} {error_id}:;")
        ]);
        assert!(
            matches!(ast.as_ref().unwrap_err(),
                MascalError {
                    error_type,
                    source,
                    ..
                } if *error_type == MascalErrorType::ParserError
                && source == "Expected a message for the throw statement but got nothing"
            ),
            "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
            &ast, MascalErrorType::ParserError, "Expected a message for the throw statement but got nothing"
        );
    });
}

#[test]
fn test_incorrect_parsing7() {
    let errors: Vec<(&str, MascalErrorType)> = get_correct_error_inputs!();
    for (error_id, _) in errors {
        let ast: Result<AbstractSyntaxTree, MascalError> = expect_error!(vec![
                format!("ThrOW {error_id}: \"12345\";")
        ]);
        assert!(
            matches!(ast.as_ref().unwrap_err(),
                MascalError {
                    error_type,
                    source,
                    ..
                } if *error_type == MascalErrorType::ParserError
                && source == "Unexpected character sequences found in a supposed expression"
            ),
            "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
            &ast, MascalErrorType::ParserError, "Unexpected character sequences found in a supposed expression"
        );
    }
}

#[test]
fn test_incorrect_parsing8() {
    let errors: Vec<(&str, MascalErrorType)> = get_correct_error_inputs!();
    for (error_id, _) in errors {
        let ast: Result<AbstractSyntaxTree, MascalError> = expect_error!(vec![
                format!("THROW {error_id}: \"12345\"")
        ]);
        assert!(
            matches!(ast.as_ref().unwrap_err(),
                MascalError {
                    error_type,
                    source,
                    ..
                } if *error_type == MascalErrorType::ParserError
                && source == "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"
            ),
            "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
            &ast, MascalErrorType::ParserError, "Unexpected characters found inside implementation block, perhaps forgot a semicolon?"
        );
    }
}