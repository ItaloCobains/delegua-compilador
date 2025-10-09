#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub line: u32,
    pub column: u32,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Number(i64, Position),
    String(&'a str, Position),
    Verdadeiro(Position),
    Falso(Position),
    Ident(&'a str, Position),
    Var(Position),
    Escreva(Position),
    Texto(Position),
    Leia(Position),
    Comprimento(Position),
    Maiuscula(Position),
    Minuscula(Position),
    Absoluto(Position),
    Potencia(Position),
    RaizQuadrada(Position),
    Import(Position),
    Se(Position),
    Senao(Position),
    SenaoSe(Position),
    Escolha(Position),
    Caso(Position),
    Padrao(Position),
    Enquanto(Position),
    Fazer(Position),
    Para(Position),
    ParaCada(Position),
    Sustar(Position),
    Continua(Position),
    Funcao(Position),
    Retorna(Position),
    Equal(Position),
    NotEqual(Position),
    Less(Position),
    Greater(Position),
    LessEqual(Position),
    GreaterEqual(Position),
    E(Position),              // e (AND)
    Ou(Position),             // ou (OR)
    Nao(Position),            // não (NOT)
    Plus(Position),
    Minus(Position),
    Multiply(Position),
    Divide(Position),
    Modulo(Position),         // % operator
    Power(Position),          // ** operator
    Increment(Position),      // ++ operator
    Decrement(Position),      // -- operator
    Assign(Position),
    Semicolon(Position),
    LeftParen(Position),
    RightParen(Position),
    LeftBrace(Position),
    RightBrace(Position),
    LeftBracket(Position),
    RightBracket(Position),
    Comma(Position),
    Colon(Position),
    Dot(Position),
    EOF(Position),
    Error(char, Position),
}
