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

    /// Boolean literals
    Verdadeiro, Falso,

    /// Identifier (variable names, function names)
    Ident(String),

    /// Variable declaration keyword
    Var,

    /// Print function keyword
    Escreva,

    /// String conversion function keyword
    Texto,

    /// Import keyword
    Import,

    /// Conditional keywords
    Se, Senao, SenaoSe,
    Escolha, Caso, Padrao,

    /// Loop keywords
    Enquanto, Fazer, Para, ParaCada, Sustar, Continua,

    /// Comparison operators
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,

    /// Arithmetic operators
    Plus, Minus, Multiply, Divide,

    /// Assignment operator
    Assign,

    /// Statement terminator
    Semicolon,

    /// Grouping symbols
    LeftParen, RightParen,
    LeftBrace, RightBrace,

    /// Comma separator
    Comma,

    /// Colon separator
    Colon,

    /// End of file marker
    EOF,
}
