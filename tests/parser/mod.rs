mod parse_throw_statement;
mod parse_declare_statement;
mod parse_conditional_statement;
mod parse_while_statements;
mod parse_for_statements;
mod parse_function;

#[macro_export]
macro_rules! run_parsing {
    ($input: expr) => {{
        use mascal::defs::token::Token;
        use mascal::lexer;
        use mascal::parser::TokenSequence;
        use mascal::parser;
        
        let tokens: Vec<Token> = lexer::tokenize($input).unwrap();
        let token_sequence: TokenSequence = TokenSequence::new(tokens);
        parser::parse(token_sequence)
    }};
}

#[macro_export]
macro_rules! expect_error {
    ($vals: expr) => {{
        let input: String = define_program_boilerplate!(Vec::<String>::new(), $vals);
        let ast: Result<AbstractSyntaxTree, MascalError> = run_parsing!(input.as_str());
        assert_eq!(ast.is_err(), true);
        ast
    }};
}

#[macro_export]
macro_rules! unwrap_to_expression {
    ($expr_str: expr) => {{
        let input_expect: String = define_program_boilerplate!(
            Vec::<String>::new(),
            vec![ format!("{};", $expr_str) ]
        );
        let Ok(ast) = run_parsing!(input_expect.as_str()) else {unreachable!()};
        let ScopedBlocks::Program(exec) = ast.blocks[0].clone() else {unreachable!()};
        let MascalStatement::ExpressionStatement(expr) = exec.body[0].clone() else {unreachable!()};
        expr
    }};
}

#[macro_export]
macro_rules! define_program_boilerplate {
    ($variables: expr, $code: expr) => {{
        format!("\
            DEFINE_PROGRAM {{\n \
                \tVARIABLES {{\n \
                    \t\t{} \n \
                \t}} \n \
                \n \
                \tIMPLEMENTATION {{\n \
                    \t\t{}\n \
                \t}}\n \
            }}", $variables.join("\n\t\t\t\t\t"), $code.join("\n\t\t\t\t\t"))
    }};
}