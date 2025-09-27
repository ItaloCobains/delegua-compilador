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

    /// Boolean literal
    Bool(bool),

    /// Variable reference
    Identifier(String),

    /// Unary operation (e.g., -x)
    Unary {
        operator: BinaryOp,
        operand: Box<Expr>,
    },

    /// Binary operation (e.g., a + b)
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    /// Function call (e.g., func(args) or expr(args))
    FunctionCall {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    
    /// Increment expression (e.g., ++x or x++)
    Increment {
        operand: Box<Expr>,
        prefix: bool, // true for ++x, false for x++
    },
    
    /// Decrement expression (e.g., --x or x--)
    Decrement {
        operand: Box<Expr>,
        prefix: bool, // true for --x, false for x--
    },

    /// Anonymous function (e.g., funcao(a) { return a + 1; })
    Function {
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add, Subtract, Multiply, Divide, Modulo, Power,
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,
    And, Or, Not,
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

    /// Import statement
    Import {
        module: String,
        items: Option<Vec<String>>,
    },

    /// If statement
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },

    /// If-ElseIf-Else chain
    IfElseIf {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_if_branches: Vec<(Expr, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    },

    /// Switch statement
    Switch {
        value: Expr,
        cases: Vec<(Expr, Vec<Statement>)>,
        default: Option<Vec<Statement>>,
    },

    /// While loop
    While {
        condition: Expr,
        body: Vec<Statement>,
    },

    /// Do-while loop
    DoWhile {
        body: Vec<Statement>,
        condition: Expr,
    },

    /// For loop
    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Vec<Statement>,
    },

    /// For-each loop
    ForEach {
        variable: String,
        iterable: Expr,
        body: Vec<Statement>,
    },

    /// Break statement
    Break,

    /// Continue statement
    Continue,

    /// Function call as a statement
    FunctionCall(Expr),

    /// Function declaration
    FunctionDeclaration {
        name: Option<String>,
        params: Vec<String>,
        body: Vec<Statement>,
    },

    /// Return statement
    Return(Option<Expr>),
}

/// Complete DC program representation
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
