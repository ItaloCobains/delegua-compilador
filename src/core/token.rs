//! # Token Module
//!
//! Defines the lexical tokens that make up the DC language.
//! Each token represents a fundamental unit of the language's syntax.

/// Represents a lexical token in the DC language
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Integer literal (e.g., 42)
    Number(i64),

    /// String literal (e.g., "hello")
    String(String),

    /// Identifier (variable names, function names)
    Ident(String),

    /// Variable declaration keyword
    Var,

    /// Print function keyword
    Escreva,

    /// String conversion function keyword
    Texto,

    /// Arithmetic operators
    Plus, Minus, Multiply, Divide,

    /// Assignment operator
    Assign,

    /// Statement terminator
    Semicolon,

    /// Grouping symbols
    LeftParen, RightParen,

    /// End of file marker
    EOF,
}
