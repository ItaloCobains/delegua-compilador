//! # Abstract Syntax Tree (AST) Module
//!
//! Defines the data structures that represent the parsed structure
//! of DC language programs. The AST is used by the code generator
//! to produce LLVM IR.

/// Represents an expression in the DC language
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// Integer literal
    Number(i64),

    /// String literal
    String(String),

    /// Variable reference
    Identifier(String),

    /// Binary operation (e.g., a + b)
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    /// Function call (e.g., escreva("hello"))
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

/// Binary operators supported by the language
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add, Subtract, Multiply, Divide,
}

/// Represents a statement in the DC language
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// Variable declaration with initialization
    VarDeclaration {
        name: String,
        value: Expr,
    },

    /// Variable assignment
    Assignment {
        name: String,
        value: Expr,
    },

    /// Function call as a statement
    FunctionCall(Expr),
}

/// Complete DC program representation
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
