//! # Error Module
//!
//! Defines custom error types for the DC language compiler.
//! Provides structured error handling throughout the compilation pipeline.

use std::fmt;

/// Represents errors that can occur during compilation
#[derive(Debug)]
pub enum CompilerError {
    /// Lexical analysis errors
    Lexer(String),

    /// Syntax parsing errors
    Parser(String),

    /// Code generation errors
    CodeGen(String),

    /// I/O errors
    Io(std::io::Error),

    /// REPL-specific errors
    Repl(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Lexer(msg) => write!(f, "Lexer Error: {}", msg),
            CompilerError::Parser(msg) => write!(f, "Parser Error: {}", msg),
            CompilerError::CodeGen(msg) => write!(f, "Code Generation Error: {}", msg),
            CompilerError::Io(err) => write!(f, "I/O Error: {}", err),
            CompilerError::Repl(msg) => write!(f, "REPL Error: {}", msg),
        }
    }
}

impl std::error::Error for CompilerError {}

impl From<std::io::Error> for CompilerError {
    fn from(err: std::io::Error) -> Self {
        CompilerError::Io(err)
    }
}

/// Type alias for Results used throughout the compiler
pub type CompilerResult<T> = Result<T, CompilerError>;
