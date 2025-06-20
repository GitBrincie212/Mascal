use std::fmt;

#[derive(Debug)]
pub enum MascalErrorType {
    LexerError,
    ParserError,
    RuntimeError,
    OverflowError,
    TypeError,
    IndexError,
    ValueError,
    UndefinedOperation,
    UndefinedErrorType,
    InputError
}

#[derive(Debug)]
pub struct MascalError {
    pub character: usize,
    pub line: usize,
    pub error_type: MascalErrorType,
    pub source: String,
}

impl fmt::Display for MascalError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(
            format!("\x1b[1;31m{:?}: {}\nAT LINE: {}; STARTING IN CHARACTER POSITION: {}\x1b[0m", 
                    self.error_type, self.source, self.line + 1, self.character + 1
            ).as_str()
        )
    }
}