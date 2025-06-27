use rstest::rstest;
use mascal::ast::AbstractSyntaxTree;
use mascal::defs::blocks::ScopedBlocks;
use mascal::defs::errors::{MascalError, MascalErrorType};
use mascal::defs::expressions::MascalExpression;
use mascal::defs::statements::MascalStatement;
use crate::{define_program_boilerplate, run_parsing};

#[rstest(
    input,
    case("a <- 3;"),
    case("b <- a + b;"),
    case("d <- 3 - 1 + a + b;"),
    case("c <- 2 - a[0] / 2;"),
)]
fn test_correct_parsing1(input: &str) {
    let input: String = define_program_boilerplate!(
        Vec::<String>::new(),
        vec![ input ]
    );
    let ast: AbstractSyntaxTree = run_parsing!(input.as_str()).unwrap();
    assert!(matches!(&ast.blocks[0],
        ScopedBlocks::Program(exec) if exec.body.len() == 1
        && matches!(&exec.body[0],
            MascalStatement::Declaration { variable, .. }
                if matches!(variable, MascalExpression::Symbolic(_))
        )
    ));
}

#[rstest(
    input, expected_dynamics,
    case("a[0] <- 3;", vec![false]),
    case("a<<1>> <- a + b;", vec![true]),
    case("d<<1 + 1>>[a[0]] <- 3 - 1;", vec![true, false]),
    case("c[0]<<3 + 2>> <- 2 - a[0] / 2;", vec![false, true]),
    case("e[0][1 / a[0]][2 + b<<2>>] <- 2;", vec![false, false, false]),
    case("f[ <<1, 2, 3>><<0>> ]<<[1, 2, 3][1]>> <- 2;", vec![false, true]),
    case("f<< <<1, 2, 3>><<0>> >> [ [1, 2, 3][1] ] <- 2;", vec![true, false]),
)]
fn test_correct_parsing2(input: &str, expected_dynamics: Vec<bool>) {
    let input: String = define_program_boilerplate!(
        Vec::<String>::new(),
        vec![ input ]
    );
    let ast: AbstractSyntaxTree = run_parsing!(input.as_str()).unwrap();
    assert_eq!(ast.blocks.len(), 1);
    let ScopedBlocks::Program(exec_block) = &ast.blocks[0] else {unreachable!()};
    assert_eq!(exec_block.body.len(), 1);
    let target: &MascalStatement = &exec_block.body[0];
    assert_eq!(matches!(target, MascalStatement::Declaration {..}), true);
    let MascalStatement::Declaration {variable: target_var, ..} = target else {
        unreachable!()
    };
    let mut are_dynamics: Vec<bool> = vec![];
    let mut curr: &MascalExpression = target_var;
    while let MascalExpression::Indexing {is_dynamic, array, ..} = curr {
        are_dynamics.push(*is_dynamic);
        curr = &*array;
    }
    are_dynamics.reverse();
    assert_eq!(expected_dynamics, are_dynamics);
}

#[rstest(
    input, closing_symbol,
    case("a[0 <- 391;", "]"),
    case("a[b[0] <- 19 + 21;", "]"),
    case("a<<b[0] <- 1 + 1;", ">>"),
    case("c<<b<<0>> <- -1313;", ">>"),
    case("c<<3>><< <- 1134;", ">>"),
    case("c[3][ <- 1;", "]"),
    case("b[3][3 / b[2] + b[1] <- 1929;", "]"),
    case("a<<3>><<b<<2>> + a[1] <- 34.32;", ">>"),
    case("a[b[0] + c[0]<<3 + 1>> <- 3948;", "]"),
)]
fn test_incorrect_parsing1(input: &str, closing_symbol: &str) {
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
                && source == &format!("Expected \"{}\" after index expression", closing_symbol)
            ),
        "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
        &ast, MascalErrorType::ParserError, &format!("Expected \"{}\" after index expression", closing_symbol)
    );
}

#[rstest(
    input, array_type,
    case("a[0]] <- 391;", "static"),
    case("a<<0>>>> <- 391;", "dynamic"),
    case("a[0]>> <- 391;", "dynamic"),
    case("a<<9>>] <- 391;", "static"),
    case("a[9]>> <- 391;", "dynamic"),
)]
fn test_incorrect_parsing2(input: &str, array_type: &str) {
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
                && source == &format!(
                    "Expected an opening character {:?} before closing an unopened {} array",
                    if array_type == "dynamic" {"<<"} else {"["},
                    array_type,
                )
            ),
        "got {:?}, expected MascalError {{ error_type: {:?}, message: {:?}, ... }}",
        &ast, MascalErrorType::ParserError, &format!(
            "Expected an opening character {:?} before closing an unopened {} array",
            if array_type == "dynamic" {"<<"} else {"["},
            array_type,
        )
    );
}