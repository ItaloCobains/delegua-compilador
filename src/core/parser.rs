//! # Parser Module
//!
//! The syntactic analyzer (parser) converts a stream of tokens into
//! an Abstract Syntax Tree (AST). It enforces the language's grammar rules
//! and provides meaningful error messages for syntax errors.

use crate::core::token::Token;
use crate::core::ast::{Expr, BinaryOp, Statement, Program};
use crate::core::error::CompilerError;

/// Parser for the DC language
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new parser with the given tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parses the complete program
    pub fn parse(&mut self) -> Result<Program, CompilerError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    /// Parses a single expression (for REPL use)
    pub fn parse_expression_only(&mut self) -> Result<Expr, CompilerError> {
        let expr = self.parse_expression()?;
        if !self.is_at_end() {
            return Err(CompilerError::Parser(
                "Unexpected tokens after expression".to_string()
            ));
        }
        Ok(expr)
    }

    /// Parses a single statement
    fn parse_statement(&mut self) -> Result<Statement, CompilerError> {
        match self.current_token() {
            Token::Var => self.parse_variable_declaration(),
            Token::Escreva => self.parse_function_call_statement(),
            Token::Import => self.parse_import_statement(),
            Token::Ident(_) => self.parse_assignment_or_call(),
            _ => Err(CompilerError::Parser(format!(
                "Unexpected token: {:?}", self.current_token()
            ))),
        }
    }

    /// Parses a variable declaration: var name = expression;
    fn parse_variable_declaration(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Var, "Expected 'var' keyword")?;

        let name = self.consume_identifier("Expected variable name")?;
        self.consume(Token::Assign, "Expected '=' after variable name")?;

        let value = self.parse_expression()?;
        self.consume(Token::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Statement::VarDeclaration { name, value })
    }

    /// Parses an assignment or function call: ident = expr; or ident(args);
    fn parse_assignment_or_call(&mut self) -> Result<Statement, CompilerError> {
        let name = match self.current_token() {
            Token::Ident(name) => name.clone(),
            _ => return Err(CompilerError::Parser("Expected identifier".to_string())),
        };
        self.advance();

        if self.match_token(Token::Assign) {
            let value = self.parse_expression()?;
            self.consume(Token::Semicolon, "Expected ';' after assignment")?;
            Ok(Statement::Assignment { name, value })
        } else if self.match_token(Token::LeftParen) {
            let args = self.parse_arguments()?;
            self.consume(Token::RightParen, "Expected ')' after function arguments")?;
            self.consume(Token::Semicolon, "Expected ';' after function call")?;
            Ok(Statement::FunctionCall(Expr::FunctionCall { name, args }))
        } else {
            Err(CompilerError::Parser(
                "Expected '=' for assignment or '(' for function call".to_string()
            ))
        }
    }

    /// Parses a function call as a statement
    fn parse_function_call_statement(&mut self) -> Result<Statement, CompilerError> {
        let expr = self.parse_function_call()?;
        self.consume(Token::Semicolon, "Expected ';' after function call")?;
        Ok(Statement::FunctionCall(expr))
    }

    /// Parses an import statement: import "module" or import {item1, item2} from "module"
    fn parse_import_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Import, "Expected 'import' keyword")?;

        if let Token::String(module) = self.current_token().clone() {
            self.advance();
            self.consume(Token::Semicolon, "Expected ';' after import")?;
            Ok(Statement::Import { module, items: None })
        } else if self.match_token(Token::LeftBrace) {
            // import {items} from "module"
            let mut items = Vec::new();
            while !self.check(Token::RightBrace) && !self.is_at_end() {
                if let Token::Ident(name) = self.current_token().clone() {
                    items.push(name);
                    self.advance();
                } else {
                    return Err(CompilerError::Parser("Expected identifier in import list".to_string()));
                }
                if self.match_token(Token::Comma) {
                    // continue
                } else if !self.check(Token::RightBrace) {
                    return Err(CompilerError::Parser("Expected ',' or '}' in import list".to_string()));
                }
            }
            self.consume(Token::RightBrace, "Expected '}' after import list")?;
            self.consume(Token::Ident("from".to_string()), "Expected 'from' after import list")?;
            if let Token::String(module) = self.current_token().clone() {
                self.advance();
                self.consume(Token::Semicolon, "Expected ';' after import")?;
                Ok(Statement::Import { module, items: Some(items) })
            } else {
                Err(CompilerError::Parser("Expected module name after 'from'".to_string()))
            }
        } else {
            Err(CompilerError::Parser("Expected string or '{' after 'import'".to_string()))
        }
    }

    /// Parses a function call expression
    fn parse_function_call(&mut self) -> Result<Expr, CompilerError> {
        let name = self.consume_function_name("Expected function name")?;
        self.consume(Token::LeftParen, "Expected '(' after function name")?;

        let args = self.parse_arguments()?;

        self.consume(Token::RightParen, "Expected ')' after function arguments")?;

        Ok(Expr::FunctionCall { name, args })
    }

    /// Parses function arguments
    fn parse_arguments(&mut self) -> Result<Vec<Expr>, CompilerError> {
        let mut args = Vec::new();

        if !self.check(Token::RightParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.match_token(Token::Semicolon) {
                    break;
                }
            }
        }

        Ok(args)
    }

    /// Parses an expression with operator precedence
    fn parse_expression(&mut self) -> Result<Expr, CompilerError> {
        self.parse_additive()
    }

    /// Parses additive expressions (+, -)
    fn parse_additive(&mut self) -> Result<Expr, CompilerError> {
        let mut left = self.parse_multiplicative()?;

        while self.match_tokens(&[Token::Plus, Token::Minus]) {
            let operator = match self.previous_token() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };

            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses multiplicative expressions (*, /)
    fn parse_multiplicative(&mut self) -> Result<Expr, CompilerError> {
        let mut left = self.parse_primary()?;

        while self.match_tokens(&[Token::Multiply, Token::Divide]) {
            let operator = match self.previous_token() {
                Token::Multiply => BinaryOp::Multiply,
                Token::Divide => BinaryOp::Divide,
                _ => unreachable!(),
            };

            let right = self.parse_primary()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses primary expressions (literals, identifiers, function calls, parentheses)
    fn parse_primary(&mut self) -> Result<Expr, CompilerError> {
        let token = self.current_token().clone();

        match token {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expr::String(s))
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();

                if self.match_token(Token::LeftParen) {
                    let args = self.parse_arguments()?;
                    self.consume(Token::RightParen, "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall { name, args })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Token::Escreva => {
                self.advance();
                self.consume(Token::LeftParen, "Expected '(' after 'escreva'")?;
                let args = self.parse_arguments()?;
                self.consume(Token::RightParen, "Expected ')' after function arguments")?;
                Ok(Expr::FunctionCall { name: "escreva".to_string(), args })
            }
            Token::Texto => {
                self.advance();
                self.consume(Token::LeftParen, "Expected '(' after 'texto'")?;
                let args = self.parse_arguments()?;
                self.consume(Token::RightParen, "Expected ')' after function arguments")?;
                Ok(Expr::FunctionCall { name: "texto".to_string(), args })
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            _ => Err(CompilerError::Parser(format!(
                "Unexpected token in expression: {:?}", token
            ))),
        }
    }

    // Helper methods

    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF)
    }

    fn previous_token(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }

    fn consume(&mut self, expected: Token, message: &str) -> Result<String, CompilerError> {
        if self.check(expected.clone()) {
            let token = self.current_token().clone();
            self.advance();
            match token {
                Token::Ident(name) => Ok(name),
                Token::String(s) => Ok(s),
                _ => Ok(String::new()),
            }
        } else {
            Err(CompilerError::Parser(message.to_string()))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String, CompilerError> {
        match self.current_token() {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(CompilerError::Parser(message.to_string())),
        }
    }

    fn consume_function_name(&mut self, message: &str) -> Result<String, CompilerError> {
        match self.current_token() {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            Token::Escreva => {
                self.advance();
                Ok("escreva".to_string())
            }
            Token::Texto => {
                self.advance();
                Ok("texto".to_string())
            }
            _ => Err(CompilerError::Parser(message.to_string())),
        }
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_tokens(&mut self, expected: &[Token]) -> bool {
        for token in expected {
            if self.check(token.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, expected: Token) -> bool {
        !self.is_at_end() && match (self.current_token(), &expected) {
            (Token::String(_), Token::String(_)) => true,
            (Token::Ident(_), Token::Ident(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            _ => std::mem::discriminant(self.current_token()) == std::mem::discriminant(&expected),
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Token::EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::lexer::Lexer;

    #[test]
    fn test_parse_variable_declaration() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var x = 42;");
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "x");
                assert_eq!(*value, Expr::Number(42));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_arithmetic_expression() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("2 + 3 * 4");
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression_only().unwrap();
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::Number(2)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Number(3)),
                operator: BinaryOp::Multiply,
                right: Box::new(Expr::Number(4)),
            }),
        });
    }

    #[test]
    fn test_parse_function_call() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("escreva(\"Hello\")");
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression_only().unwrap();
        assert_eq!(expr, Expr::FunctionCall {
            name: "escreva".to_string(),
            args: vec![Expr::String("Hello".to_string())],
        });
    }

    #[test]
    fn test_parse_complex_program() {
        let lexer = Lexer::new();
        let code = r#"
            var a = 10;
            var b = 5;
            escreva("Sum: " + texto(a + b));
        "#;
        let tokens = lexer.tokenize(code);
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 3);
    }
}
