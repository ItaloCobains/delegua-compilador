#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(i64),

    String(String),

    Bool(bool),

    Identifier(String),

    Unary {
        operator: BinaryOp,
        operand: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    FunctionCall {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    
    Increment {
        operand: Box<Expr>,
        prefix: bool, // true for ++x, false for x++
    },
    
    Decrement {
        operand: Box<Expr>,
        prefix: bool, // true for --x, false for x--
    },

    Function {
        params: Vec<String>,
        body: Vec<Statement>,
    },

    Array {
        elements: Vec<Expr>,
    },

    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },

    Object {
        properties: Vec<(String, Expr)>,
    },

    PropertyAccess {
        object: Box<Expr>,
        property: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add, Subtract, Multiply, Divide, Modulo, Power,
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,
    And, Or, Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VarDeclaration {
        name: String,
        value: Expr,
    },

    Assignment {
        name: String,
        value: Expr,
    },

    Import {
        module: String,
        items: Option<Vec<String>>,
    },

    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },

    IfElseIf {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_if_branches: Vec<(Expr, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    },

    Switch {
        value: Expr,
        cases: Vec<(Expr, Vec<Statement>)>,
        default: Option<Vec<Statement>>,
    },

    While {
        condition: Expr,
        body: Vec<Statement>,
    },

    DoWhile {
        body: Vec<Statement>,
        condition: Expr,
    },

    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Vec<Statement>,
    },

    ForEach {
        variable: String,
        iterable: Expr,
        body: Vec<Statement>,
    },

    Break,

    Continue,

    FunctionCall(Expr),

    FunctionDeclaration {
        name: Option<String>,
        params: Vec<String>,
        body: Vec<Statement>,
    },

    Return(Option<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
