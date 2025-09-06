use dc::core::lexer::Lexer;
use dc::core::token::{Token, Position};

fn main() {
    let mut lexer = Lexer::new();
    let input = "var x = 42; escreva(\"Hello World\");";
    let tokens = lexer.tokenize(input);

    println!("Tokenized {} characters into {} tokens", input.len(), tokens.len());

    for token in tokens {
        match token {
            Token::Var(pos) => println!("VAR at line {}, col {}", pos.line, pos.column),
            Token::Ident(ident, pos) => println!("IDENT '{}' at line {}, col {}", ident, pos.line, pos.column),
            Token::Assign(pos) => println!("ASSIGN at line {}, col {}", pos.line, pos.column),
            Token::Number(n, pos) => println!("NUMBER {} at line {}, col {}", n, pos.line, pos.column),
            Token::Semicolon(pos) => println!("SEMICOLON at line {}, col {}", pos.line, pos.column),
            Token::Escreva(pos) => println!("ESCREVA at line {}, col {}", pos.line, pos.column),
            Token::LeftParen(pos) => println!("LPAREN at line {}, col {}", pos.line, pos.column),
            Token::String(s, pos) => println!("STRING '{}' at line {}, col {}", s, pos.line, pos.column),
            Token::RightParen(pos) => println!("RPAREN at line {}, col {}", pos.line, pos.column),
            Token::EOF(pos) => println!("EOF at line {}, col {}", pos.line, pos.column),
            _ => println!("Other token"),
        }
    }
}
