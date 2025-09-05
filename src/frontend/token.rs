#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Number(i64),
    String(String),
    Var,
    Escreva,
    Texto,
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Semicolon,
    LeftParen,
    RightParen,
    EOF
}