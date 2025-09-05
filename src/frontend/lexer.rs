use crate::frontend::token::Token;

pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Lexer
    }

    fn lex_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut num = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                num.push(c);
                chars.next();
            } else {
                break;
            }
        }
        Token::Number(num.parse().unwrap())
    }

    fn lex_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut string_val = String::new();
        chars.next(); // Consome a primeira aspas

        while let Some(&c) = chars.peek() {
            if c == '"' {
                chars.next(); // Consome a última aspas
                break;
            }
            string_val.push(c);
            chars.next();
        }

        Token::String(string_val)
    }

    fn lex_ident_or_keyword(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut ident = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_' {
                ident.push(c);
                chars.next();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "var" => Token::Var,
            "escreva" => Token::Escreva,
            "texto" => Token::Texto,
            _ => Token::Ident(ident),
        }
    }

    fn skip_comment(chars: &mut std::iter::Peekable<std::str::Chars>) {
        while let Some(&c) = chars.peek() {
            if c == '\n' {
                break;
            }
            chars.next();
        }
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' | '\n' | '\r' => { chars.next(); }
                '+' => { chars.next(); tokens.push(Token::Plus); }
                '-' => { chars.next(); tokens.push(Token::Minus); }
                '*' => { chars.next(); tokens.push(Token::Multiply); }
                '/' => {
                    chars.next();
                    if let Some(&'/') = chars.peek() {
                        chars.next();
                        Self::skip_comment(&mut chars);
                    } else {
                        tokens.push(Token::Divide);
                    }
                }
                '=' => { chars.next(); tokens.push(Token::Assign); }
                ';' => { chars.next(); tokens.push(Token::Semicolon); }
                '(' => { chars.next(); tokens.push(Token::LeftParen); }
                ')' => { chars.next(); tokens.push(Token::RightParen); }
                '"' => tokens.push(Self::lex_string(&mut chars)),
                '0'..='9' => tokens.push(Self::lex_number(&mut chars)),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(Self::lex_ident_or_keyword(&mut chars)),
                _ => { chars.next(); }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var a = 10;");
        assert_eq!(tokens, vec![
            Token::Var,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Number(10),
            Token::Semicolon,
            Token::EOF
        ]);
    }

    #[test]
    fn test_string_literal() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("\"Hello World\"");
        assert_eq!(tokens, vec![
            Token::String("Hello World".to_string()),
            Token::EOF
        ]);
    }

    #[test]
    fn test_escreva_function() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("escreva(\"Hello\");");
        assert_eq!(tokens, vec![
            Token::Escreva,
            Token::LeftParen,
            Token::String("Hello".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::EOF
        ]);
    }

    #[test]
    fn test_arithmetic_operators() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("a + b - c * d / e");
        assert_eq!(tokens, vec![
            Token::Ident("a".to_string()),
            Token::Plus,
            Token::Ident("b".to_string()),
            Token::Minus,
            Token::Ident("c".to_string()),
            Token::Multiply,
            Token::Ident("d".to_string()),
            Token::Divide,
            Token::Ident("e".to_string()),
            Token::EOF
        ]);
    }

    #[test]
    fn test_texto_function() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("texto(a)");
        assert_eq!(tokens, vec![
            Token::Texto,
            Token::LeftParen,
            Token::Ident("a".to_string()),
            Token::RightParen,
            Token::EOF
        ]);
    }

    #[test]
    fn test_complex_expression() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("\"Valor: \" + texto(a)");
        assert_eq!(tokens, vec![
            Token::String("Valor: ".to_string()),
            Token::Plus,
            Token::Texto,
            Token::LeftParen,
            Token::Ident("a".to_string()),
            Token::RightParen,
            Token::EOF
        ]);
    }

    #[test]
    fn test_multiple_statements() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var a = 10;\nvar b = 20;");
        assert_eq!(tokens, vec![
            Token::Var,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Number(10),
            Token::Semicolon,
            Token::Var,
            Token::Ident("b".to_string()),
            Token::Assign,
            Token::Number(20),
            Token::Semicolon,
            Token::EOF
        ]);
    }

    #[test]
    fn test_comment_single_line() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var a = 10; // Este é um comentário");
        assert_eq!(tokens, vec![
            Token::Var,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Number(10),
            Token::Semicolon,
            Token::EOF
        ]);
    }

    #[test]
    fn test_comment_full_line() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("// Comentário completo\nvar a = 5;");
        assert_eq!(tokens, vec![
            Token::Var,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Number(5),
            Token::Semicolon,
            Token::EOF
        ]);
    }

    #[test]
    fn test_identifier_with_numbers() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var var1 = 10;");
        assert_eq!(tokens, vec![
            Token::Var,
            Token::Ident("var1".to_string()),
            Token::Assign,
            Token::Number(10),
            Token::Semicolon,
            Token::EOF
        ]);
    }

    #[test]
    fn test_identifier_with_underscore() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var _private_var = 5;");
        assert_eq!(tokens, vec![
            Token::Var,
            Token::Ident("_private_var".to_string()),
            Token::Assign,
            Token::Number(5),
            Token::Semicolon,
            Token::EOF
        ]);
    }

    #[test]
    fn test_empty_string() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("\"\"");
        assert_eq!(tokens, vec![
            Token::String("".to_string()),
            Token::EOF
        ]);
    }

    #[test]
    fn test_string_with_spaces() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("\"Hello World With Spaces\"");
        assert_eq!(tokens, vec![
            Token::String("Hello World With Spaces".to_string()),
            Token::EOF
        ]);
    }

    #[test]
    fn test_division_vs_comment() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("a / b");
        assert_eq!(tokens, vec![
            Token::Ident("a".to_string()),
            Token::Divide,
            Token::Ident("b".to_string()),
            Token::EOF
        ]);
    }

    #[test]
    fn test_complete_program_sample() {
        let lexer = Lexer::new();
        let code = r#"var a = 10;
var b = 4;
escreva("Valor de A: " + texto(a));"#;

        let tokens = lexer.tokenize(code);
        assert_eq!(tokens, vec![
            Token::Var,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Number(10),
            Token::Semicolon,
            Token::Var,
            Token::Ident("b".to_string()),
            Token::Assign,
            Token::Number(4),
            Token::Semicolon,
            Token::Escreva,
            Token::LeftParen,
            Token::String("Valor de A: ".to_string()),
            Token::Plus,
            Token::Texto,
            Token::LeftParen,
            Token::Ident("a".to_string()),
            Token::RightParen,
            Token::RightParen,
            Token::Semicolon,
            Token::EOF
        ]);
    }

    #[test]
    fn test_all_keywords() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var escreva texto");
        assert_eq!(tokens, vec![
            Token::Var,
            Token::Escreva,
            Token::Texto,
            Token::EOF
        ]);
    }

    #[test]
    fn test_mixed_operators_and_assignments() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("soma = a + b * c / d - e;");
        assert_eq!(tokens, vec![
            Token::Ident("soma".to_string()),
            Token::Assign,
            Token::Ident("a".to_string()),
            Token::Plus,
            Token::Ident("b".to_string()),
            Token::Multiply,
            Token::Ident("c".to_string()),
            Token::Divide,
            Token::Ident("d".to_string()),
            Token::Minus,
            Token::Ident("e".to_string()),
            Token::Semicolon,
            Token::EOF
        ]);
    }
}