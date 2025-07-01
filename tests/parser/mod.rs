mod parse_throw_statement;
mod parse_declare_statement;
mod parse_conditional_statement;
mod parse_while_statements;

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