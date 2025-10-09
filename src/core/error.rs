use std::fmt;

#[derive(Debug, Clone)]
pub enum CompilerError {
    Lexer(String),

    Parser(String),

    CodeGen(String),

    Io(String)
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Lexer(msg) => write!(f, "Lexer Error: {}", msg),
            CompilerError::Parser(msg) => write!(f, "Parser Error: {}", msg),
            CompilerError::CodeGen(msg) => write!(f, "Code Generation Error: {}", msg),
            CompilerError::Io(err) => write!(f, "I/O Error: {}", err),
        }
    }
}

impl std::error::Error for CompilerError {}

impl From<std::io::Error> for CompilerError {
    fn from(err: std::io::Error) -> Self {
        CompilerError::Io(err.to_string())
    }
}
