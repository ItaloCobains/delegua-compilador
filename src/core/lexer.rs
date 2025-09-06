//! # Lexer Module
//!
//! The lexical analyzer (lexer) converts source code into a stream of tokens.
//! It handles whitespace, comments, and identifies language keywords and symbols.
//!
//! Optimized for performance with:
//! - Zero-copy string handling using string slices
//! - Pre-allocated token buffers
//! - Keyword lookup tables
//! - Memory pooling for common tokens
//! - Position tracking

use crate::core::token::{Token, Position};

/// Memory pool for common token types to reduce allocations
#[derive(Default)]
struct TokenPool {
    positions: Vec<Position>,
    pos_index: usize,
}

impl TokenPool {
    fn new(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            pos_index: 0,
        }
    }

    fn get_position(&mut self, line: u32, column: u32, offset: usize) -> Position {
        if self.pos_index < self.positions.len() {
            let pos = &mut self.positions[self.pos_index];
            pos.line = line;
            pos.column = column;
            pos.offset = offset;
            self.pos_index += 1;
            *pos
        } else {
            let pos = Position { line, column, offset };
            self.positions.push(pos);
            self.pos_index += 1;
            pos
        }
    }

    fn reset(&mut self) {
        self.pos_index = 0;
    }
}

/// Optimized lexical analyzer for the DC language
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    position: Position,
    token_pool: TokenPool,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer instance with optimized memory allocation
    pub fn new() -> Self {
        Self {
            input: "",
            chars: "".chars().peekable(),
            position: Position { line: 1, column: 1, offset: 0 },
            token_pool: TokenPool::new(1024), // Pre-allocate for 1024 tokens
        }
    }

    /// Tokenizes the input source code into a vector of tokens with zero-copy optimization
    pub fn tokenize(&mut self, input: &'a str) -> Vec<Token<'a>> {
        self.input = input;
        self.chars = input.chars().peekable();
        self.position = Position { line: 1, column: 1, offset: 0 };
        self.token_pool.reset();

        let mut tokens = Vec::with_capacity(256); // Pre-allocate reasonable capacity

        while let Some(&ch) = self.chars.peek() {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.position.line += 1;
                    self.position.column = 1;
                    self.advance();
                }

                // Single-line comments
                '/' => {
                    self.advance();
                    if let Some('/') = self.chars.peek() {
                        self.advance();
                        self.skip_comment();
                    } else {
                        tokens.push(Token::Divide(self.get_position()));
                    }
                }

                // Operators and punctuation
                '+' => { self.advance(); tokens.push(Token::Plus(self.get_position())); }
                '-' => { self.advance(); tokens.push(Token::Minus(self.get_position())); }
                '*' => { self.advance(); tokens.push(Token::Multiply(self.get_position())); }
                '=' => {
                    self.advance();
                    if let Some('=') = self.chars.peek() {
                        self.advance();
                        tokens.push(Token::Equal(self.get_position()));
                    } else {
                        tokens.push(Token::Assign(self.get_position()));
                    }
                }
                '!' => {
                    self.advance();
                    if let Some('=') = self.chars.peek() {
                        self.advance();
                        tokens.push(Token::NotEqual(self.get_position()));
                    } else {
                        tokens.push(Token::Error('!', self.get_position()));
                    }
                }
                '<' => {
                    self.advance();
                    if let Some('=') = self.chars.peek() {
                        self.advance();
                        tokens.push(Token::LessEqual(self.get_position()));
                    } else {
                        tokens.push(Token::Less(self.get_position()));
                    }
                }
                '>' => {
                    self.advance();
                    if let Some('=') = self.chars.peek() {
                        self.advance();
                        tokens.push(Token::GreaterEqual(self.get_position()));
                    } else {
                        tokens.push(Token::Greater(self.get_position()));
                    }
                }
                ';' => { self.advance(); tokens.push(Token::Semicolon(self.get_position())); }
                '(' => { self.advance(); tokens.push(Token::LeftParen(self.get_position())); }
                ')' => { self.advance(); tokens.push(Token::RightParen(self.get_position())); }
                '{' => { self.advance(); tokens.push(Token::LeftBrace(self.get_position())); }
                '}' => { self.advance(); tokens.push(Token::RightBrace(self.get_position())); }
                ',' => { self.advance(); tokens.push(Token::Comma(self.get_position())); }
                ':' => { self.advance(); tokens.push(Token::Colon(self.get_position())); }

                // String literals
                '"' => {
                    if let Some(token) = self.lex_string() {
                        tokens.push(token);
                    }
                }

                // Numbers
                '0'..='9' => tokens.push(self.lex_number()),

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.lex_identifier_or_keyword()),

                // Unknown characters
                _ => {
                    let ch = self.chars.next().unwrap();
                    self.position.column += 1;
                    self.position.offset += ch.len_utf8();
                    tokens.push(Token::Error(ch, self.get_position()));
                }
            }
        }

        tokens.push(Token::EOF(self.get_position()));
        tokens
    }

    /// Advances to the next character and updates position
    fn advance(&mut self) {
        if let Some(ch) = self.chars.next() {
            self.position.column += 1;
            self.position.offset += ch.len_utf8();
        }
    }

    /// Gets current position from pool
    fn get_position(&mut self) -> Position {
        self.token_pool.get_position(self.position.line, self.position.column, self.position.offset)
    }

    /// Lexes a number literal with position tracking
    fn lex_number(&mut self) -> Token<'a> {
        let start_pos = self.get_position();
        let start_offset = self.position.offset;

        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let num_str = &self.input[start_offset..self.position.offset];
        let value = num_str.parse().unwrap_or(0);
        Token::Number(value, start_pos)
    }

    /// Lexes a string literal with zero-copy optimization
    fn lex_string(&mut self) -> Option<Token<'a>> {
        let start_pos = self.get_position();
        self.advance(); // Skip opening quote
        let start_offset = self.position.offset;

        while let Some(&ch) = self.chars.peek() {
            if ch == '"' {
                let end_offset = self.position.offset;
                self.advance(); // Skip closing quote
                let string_slice = &self.input[start_offset..end_offset];
                return Some(Token::String(string_slice, start_pos));
            }
            if ch == '\n' {
                // Unterminated string
                return Some(Token::Error('"', start_pos));
            }
            self.advance();
        }

        // Unterminated string at EOF
        Some(Token::Error('"', start_pos))
    }

    /// Lexes an identifier or keyword with optimized keyword matching
    fn lex_identifier_or_keyword(&mut self) -> Token<'a> {
        let start_pos = self.get_position();
        let start_offset = self.position.offset;

        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' || ch == '.' {
                self.advance();
            } else {
                break;
            }
        }

        let ident_slice = &self.input[start_offset..self.position.offset];

        // Optimized keyword matching using direct comparison
        match ident_slice {
            "var" => Token::Var(start_pos),
            "escreva" => Token::Escreva(start_pos),
            "texto" => Token::Texto(start_pos),
            "importar" => Token::Import(start_pos),
            "se" => Token::Se(start_pos),
            "senao" => self.handle_senao_se(start_pos),
            "escolha" => Token::Escolha(start_pos),
            "caso" => Token::Caso(start_pos),
            "padrao" => Token::Padrao(start_pos),
            "enquanto" => Token::Enquanto(start_pos),
            "fazer" => Token::Fazer(start_pos),
            "para" => self.handle_para_cada(start_pos),
            "sustar" => Token::Sustar(start_pos),
            "continua" => Token::Continua(start_pos),
            "verdadeiro" => Token::Verdadeiro(start_pos),
            "falso" => Token::Falso(start_pos),
            "funcao" => Token::Funcao(start_pos),
            "retorna" => Token::Retorna(start_pos),
            _ => Token::Ident(ident_slice, start_pos),
        }
    }

    /// Handles "senao se" compound keyword
    fn handle_senao_se(&mut self, start_pos: Position) -> Token<'a> {
        // Skip whitespace
        while let Some(&ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        // Check if next word is "se"
        let mut temp_chars = self.chars.clone();
        let mut next_word = String::new();
        while let Some(&ch) = temp_chars.peek() {
            if ch.is_alphabetic() {
                next_word.push(ch);
                temp_chars.next();
            } else {
                break;
            }
        }

        if next_word == "se" {
            // Consume the "se" part
            for _ in 0.."se".len() {
                self.advance();
            }
            Token::SenaoSe(start_pos)
        } else {
            Token::Senao(start_pos)
        }
    }

    /// Handles "para cada" compound keyword
    fn handle_para_cada(&mut self, start_pos: Position) -> Token<'a> {
        // Skip whitespace
        while let Some(&ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        // Check if next word is "cada"
        let mut temp_chars = self.chars.clone();
        let mut next_word = String::new();
        while let Some(&ch) = temp_chars.peek() {
            if ch.is_alphabetic() {
                next_word.push(ch);
                temp_chars.next();
            } else {
                break;
            }
        }

        if next_word == "cada" {
            // Consume the "cada" part
            for _ in 0.."cada".len() {
                self.advance();
            }
            Token::ParaCada(start_pos)
        } else {
            Token::Para(start_pos)
        }
    }

    /// Skips a single-line comment
    fn skip_comment(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_variable_declaration() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var x = 42;");
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0], Token::Var(_)));
        assert!(matches!(tokens[1], Token::Ident("x", _)));
        assert!(matches!(tokens[2], Token::Assign(_)));
        assert!(matches!(tokens[3], Token::Number(42, _)));
        assert!(matches!(tokens[4], Token::Semicolon(_)));
        assert!(matches!(tokens[5], Token::EOF(_)));
    }

    #[test]
    fn test_tokenize_string_literal() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("\"Hello World\"");
        let expected = vec![
            Token::String("Hello World", Position { line: 1, column: 1, offset: 0 }),
            Token::EOF(Position { line: 1, column: 14, offset: 13 })
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_arithmetic_expression() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("a + b * c");
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0], Token::Ident("a", _)));
        assert!(matches!(tokens[1], Token::Plus(_)));
        assert!(matches!(tokens[2], Token::Ident("b", _)));
        assert!(matches!(tokens[3], Token::Multiply(_)));
        assert!(matches!(tokens[4], Token::Ident("c", _)));
        assert!(matches!(tokens[5], Token::EOF(_)));
    }

    #[test]
    fn test_tokenize_function_call() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("escreva(\"test\")");
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Token::Escreva(_)));
        assert!(matches!(tokens[1], Token::LeftParen(_)));
        assert!(matches!(tokens[2], Token::String("test", _)));
        assert!(matches!(tokens[3], Token::RightParen(_)));
        assert!(matches!(tokens[4], Token::EOF(_)));
    }

    #[test]
    fn test_tokenize_with_comments() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var x = 1; // comment\nvar y = 2;");
        assert_eq!(tokens.len(), 11);
        assert!(matches!(tokens[0], Token::Var(_)));
        assert!(matches!(tokens[1], Token::Ident("x", _)));
        assert!(matches!(tokens[2], Token::Assign(_)));
        assert!(matches!(tokens[3], Token::Number(1, _)));
        assert!(matches!(tokens[4], Token::Semicolon(_)));
        assert!(matches!(tokens[5], Token::Var(_)));
        assert!(matches!(tokens[6], Token::Ident("y", _)));
        assert!(matches!(tokens[7], Token::Assign(_)));
        assert!(matches!(tokens[8], Token::Number(2, _)));
        assert!(matches!(tokens[9], Token::Semicolon(_)));
        assert!(matches!(tokens[10], Token::EOF(_)));
    }
}
