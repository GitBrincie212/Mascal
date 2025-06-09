use crate::defs::token::Token;

pub enum Expression<'a> {
    Atom(Token<'a>),
    Operation(Token<'a>, Vec<Expression<'a>>),
    
}

pub fn parse(tokens: Vec<Token>) {
    
}