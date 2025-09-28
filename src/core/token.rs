//! # Token Module
//!
//! Defines the lexical tokens that make up the DC language.
//! Each token represents a fundamental unit of the language's syntax.

/// Position information for tokens
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub line: u32,
    pub column: u32,
    pub offset: usize,
}

/// Represents a lexical token in the DC language with position information
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    /// Integer literal (e.g., 42)
    Number(i64, Position),

    /// String literal (e.g., "hello") - zero-copy using string slice
    String(&'a str, Position),

    /// Boolean literals
    Verdadeiro(Position),
    Falso(Position),

    /// Identifier (variable names, function names) - zero-copy using string slice
    Ident(&'a str, Position),

    /// Variable declaration keyword
    Var(Position),

    /// Print function keyword
    Escreva(Position),

    /// String conversion function keyword
    Texto(Position),

    /// Input function keyword
    Leia(Position),

    /// String length function keyword
    Comprimento(Position),

    /// String uppercase function keyword
    Maiuscula(Position),

    /// String lowercase function keyword
    Minuscula(Position),

    /// Math function keywords
    Absoluto(Position),
    Potencia(Position),
    RaizQuadrada(Position),

    /// Import keyword
    Import(Position),

    /// Conditional keywords
    Se(Position),
    Senao(Position),
    SenaoSe(Position),
    Escolha(Position),
    Caso(Position),
    Padrao(Position),

    /// Loop keywords
    Enquanto(Position),
    Fazer(Position),
    Para(Position),
    ParaCada(Position),
    Sustar(Position),
    Continua(Position),

    /// Function keywords
    Funcao(Position),
    Retorna(Position),

    /// Comparison operators
    Equal(Position),
    NotEqual(Position),
    Less(Position),
    Greater(Position),
    LessEqual(Position),
    GreaterEqual(Position),

    /// Logical operators
    E(Position),              // e (AND)
    Ou(Position),             // ou (OR)
    Nao(Position),            // não (NOT)

    /// Arithmetic operators
    Plus(Position),
    Minus(Position),
    Multiply(Position),
    Divide(Position),
    Modulo(Position),         // % operator
    Power(Position),          // ** operator
    
    /// Increment/decrement operators
    Increment(Position),      // ++ operator
    Decrement(Position),      // -- operator

    /// Assignment operator
    Assign(Position),

    /// Statement terminator
    Semicolon(Position),

    /// Grouping symbols
    LeftParen(Position),
    RightParen(Position),
    LeftBrace(Position),
    RightBrace(Position),
    LeftBracket(Position),
    RightBracket(Position),

    /// Comma separator
    Comma(Position),

    /// Colon separator
    Colon(Position),

    /// Dot for property access
    Dot(Position),

    /// End of file marker
    EOF(Position),

    /// Error token for invalid characters
    Error(char, Position),
}
