use logos::Span;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::token::{Token};
use crate::lexer;
use crate::parser::{TokenSequence, parse};
use crate::runtime::interpert;
use crate::semantic_analysis::conduct_semantic_analysis;

macro_rules! define_pipeline_step {
    ($func: expr, $($args: expr)*) => {
        match $func($($args),*) {
            Ok(val) => Some(val),
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    };
}

pub fn trigger_pipeline(contents: String) {
    let tokens: Result<Vec<Token>, (Span, usize, &str)> = lexer::tokenize(&contents);
    let token_sequence: TokenSequence = match tokens {
        Ok(toks) => {
            if toks.is_empty() {
                return;
            }
            let token_sequence: TokenSequence = TokenSequence::new(toks);
            token_sequence
        }
        
        Err((range, line, value)) => {
            println!(
                "{}",
                MascalError {
                    error_type: MascalErrorType::LexerError,
                    line,
                    character: range.start,
                    source: format!("Unknown Character Sequence \"{}\"", value)
                }
            );
            return;
        }
    };
    let Some(tree) = define_pipeline_step!(parse, token_sequence) else {
        return;
    };
    let Some(tree) = define_pipeline_step!(conduct_semantic_analysis, tree) else {
        return;
    };
    let _ = define_pipeline_step!(interpert, tree).is_none();
}
