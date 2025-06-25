use crate::lexer::tokenize;
use mascal::defs::token::{Token, TokenType};
use crate::{test_individual_token};

test_individual_token!(test_define_func,  "Define_Function", TokenType::DefineFunction);
test_individual_token!(test_define_program,  "Define_Program", TokenType::DefineProgram);
test_individual_token!(test_false,  "False", TokenType::False);
test_individual_token!(test_true,  "True", TokenType::True);
test_individual_token!(test_const,  "Const", TokenType::Const);
test_individual_token!(test_null, "Null", TokenType::Null);
test_individual_token!(test_typeof, "Typeof", TokenType::Typeof);
test_individual_token!(test_mut, "Mut", TokenType::Mutable);
test_individual_token!(test_from, "From", TokenType::From);
test_individual_token!(test_to, "To", TokenType::To);
test_individual_token!(test_with_step, "With_Step", TokenType::WithStep);
test_individual_token!(test_implementation, "Implementation", TokenType::Implementation);
test_individual_token!(test_variables, "Variables", TokenType::Variables);