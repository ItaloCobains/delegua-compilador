//! # Lexer Module
//!
//! The lexical analyzer (lexer) converts source code into a stream of tokens.
//! It handles whitespace, comments, and identifies language keywords and symbols.

use crate::core::token::Token;

/// Lexical analyzer for the DC language
pub struct Lexer;

impl Lexer {
    /// Creates a new lexer instance
    pub fn new() -> Self {
        Lexer
    }

    /// Tokenizes the input source code into a vector of tokens
    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                }

                // Single-line comments
                '/' => {
                    chars.next();
                    if let Some('/') = chars.peek() {
                        chars.next();
                        Self::skip_comment(&mut chars);
                    } else {
                        tokens.push(Token::Divide);
                    }
                }

                // Operators and punctuation
                '+' => { chars.next(); tokens.push(Token::Plus); }
                '-' => { chars.next(); tokens.push(Token::Minus); }
                '*' => { chars.next(); tokens.push(Token::Multiply); }
                '=' => { chars.next(); tokens.push(Token::Assign); }
                ';' => { chars.next(); tokens.push(Token::Semicolon); }
                '(' => { chars.next(); tokens.push(Token::LeftParen); }
                ')' => { chars.next(); tokens.push(Token::RightParen); }

                // String literals
                '"' => tokens.push(Self::lex_string(&mut chars)),

                // Numbers
                '0'..='9' => tokens.push(Self::lex_number(&mut chars)),

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    tokens.push(Self::lex_identifier_or_keyword(&mut chars));
                }

                // Skip unknown characters
                _ => { chars.next(); }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    /// Lexes a number literal
    fn lex_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut num_str = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        let value = num_str.parse().unwrap_or(0);
        Token::Number(value)
    }

    /// Lexes a string literal
    fn lex_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut string_content = String::new();
        chars.next(); // Skip opening quote

        while let Some(&ch) = chars.peek() {
            if ch == '"' {
                chars.next(); // Skip closing quote
                break;
            }
            string_content.push(ch);
            chars.next();
        }

        Token::String(string_content)
    }

    /// Lexes an identifier or keyword
    fn lex_identifier_or_keyword(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut ident = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' {
                ident.push(ch);
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

    /// Skips a single-line comment
    fn skip_comment(chars: &mut std::iter::Peekable<std::str::Chars>) {
        while let Some(&ch) = chars.peek() {
            if ch == '\n' {
                break;
            }
            chars.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_variable_declaration() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var x = 42;");
        let expected = vec![
            Token::Var,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Number(42),
            Token::Semicolon,
            Token::EOF
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_string_literal() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("\"Hello World\"");
        let expected = vec![
            Token::String("Hello World".to_string()),
            Token::EOF
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_arithmetic_expression() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("a + b * c");
        let expected = vec![
            Token::Ident("a".to_string()),
            Token::Plus,
            Token::Ident("b".to_string()),
            Token::Multiply,
            Token::Ident("c".to_string()),
            Token::EOF
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_function_call() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("escreva(\"test\")");
        let expected = vec![
            Token::Escreva,
            Token::LeftParen,
            Token::String("test".to_string()),
            Token::RightParen,
            Token::EOF
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_with_comments() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var x = 1; // comment\nvar y = 2;");
        let expected = vec![
            Token::Var,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Number(1),
            Token::Semicolon,
            Token::Var,
            Token::Ident("y".to_string()),
            Token::Assign,
            Token::Number(2),
            Token::Semicolon,
            Token::EOF
        ];
        assert_eq!(tokens, expected);
    }
}
