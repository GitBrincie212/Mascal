use crate::lexer::tokenize;
use mascal::defs::token::{Token, TokenType};
use crate::{test_individual_token};

test_individual_token!(test_dynamic,  "Dynamic", TokenType::Dynamic);
test_individual_token!(test_integer,  "Integer", TokenType::Integer);
test_individual_token!(test_string,  "String", TokenType::String);
test_individual_token!(test_float,  "Float", TokenType::Float);
test_individual_token!(test_type,  "Type", TokenType::Type);
test_individual_token!(test_boolean, "Boolean", TokenType::Boolean);