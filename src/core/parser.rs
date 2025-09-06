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
            Token::Se => self.parse_if_statement(),
            Token::Escolha => self.parse_switch_statement(),
            Token::Enquanto => self.parse_while_statement(),
            Token::Fazer => self.parse_do_while_statement(),
            Token::Para => self.parse_for_statement(),
            Token::ParaCada => self.parse_for_each_statement(),
            Token::Sustar => self.parse_break_statement(),
            Token::Continua => self.parse_continue_statement(),
            Token::Funcao => {
                // Check if this is a named function declaration (funcao name(...))
                // or an anonymous function in expression context
                self.advance(); // consume funcao
                if let Token::Ident(_) = self.current_token() {
                    // Named function declaration
                    self.parse_named_function_declaration()
                } else {
                    // This might be an anonymous function, but in statement context it's an error
                    Err(CompilerError::Parser("Unexpected 'funcao' in statement context".to_string()))
                }
            }
            Token::Retorna => self.parse_return_statement(),
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
            Ok(Statement::FunctionCall(Expr::FunctionCall {
                callee: Box::new(Expr::Identifier(name)),
                args,
            }))
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

    /// Parses an import statement: importar "module" or importar {item1, item2} from "module"
    fn parse_import_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Import, "Expected 'importar' keyword")?;

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
        let callee = self.parse_primary()?;

        // If parse_primary already returned a complete function call (for built-ins like escreva),
        // just return it as-is
        if let Expr::FunctionCall { .. } = callee {
            return Ok(callee);
        }

        self.consume(Token::LeftParen, "Expected '(' after function name")?;

        let args = self.parse_arguments()?;

        self.consume(Token::RightParen, "Expected ')' after function arguments")?;

        Ok(Expr::FunctionCall { callee: Box::new(callee), args })
    }

    /// Parses function arguments
    fn parse_arguments(&mut self) -> Result<Vec<Expr>, CompilerError> {
        let mut args = Vec::new();

        if !self.check(Token::RightParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
        }

        Ok(args)
    }

    /// Parses function parameters: (param1, param2, ...)
    fn parse_parameters(&mut self) -> Result<Vec<String>, CompilerError> {
        let mut params = Vec::new();
        if !self.check(Token::RightParen) {
            loop {
                if let Token::Ident(param) = self.current_token() {
                    params.push(param.clone());
                    self.advance();
                } else {
                    return Err(CompilerError::Parser("Expected parameter name".to_string()));
                }

                if self.match_token(Token::Comma) {
                    continue;
                } else {
                    break;
                }
            }
        }
        Ok(params)
    }

    /// Parses an expression with operator precedence
    fn parse_expression(&mut self) -> Result<Expr, CompilerError> {
        self.parse_comparison()
    }

    /// Parses comparison expressions (<, >, <=, >=, ==, !=)
    fn parse_comparison(&mut self) -> Result<Expr, CompilerError> {
        let mut left = self.parse_additive()?;

        while self.match_tokens(&[Token::Less, Token::Greater, Token::LessEqual, Token::GreaterEqual, Token::Equal, Token::NotEqual]) {
            let operator = match self.previous_token() {
                Token::Less => BinaryOp::Less,
                Token::Greater => BinaryOp::Greater,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };

            let right = self.parse_additive()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
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
        let mut left = self.parse_unary()?;

        while self.match_tokens(&[Token::Multiply, Token::Divide]) {
            let operator = match self.previous_token() {
                Token::Multiply => BinaryOp::Multiply,
                Token::Divide => BinaryOp::Divide,
                _ => unreachable!(),
            };

            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses unary expressions (-, +)
    fn parse_unary(&mut self) -> Result<Expr, CompilerError> {
        if self.match_tokens(&[Token::Plus, Token::Minus]) {
            let operator = match self.previous_token() {
                Token::Plus => BinaryOp::Add, // +x is just x
                Token::Minus => BinaryOp::Subtract, // -x
                _ => unreachable!(),
            };

            let right = self.parse_unary()?;
            Ok(Expr::Unary {
                operator,
                operand: Box::new(right),
            })
        } else {
            self.parse_primary()
        }
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
            Token::Verdadeiro => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Token::Falso => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            Token::Escreva => {
                self.advance();
                self.consume(Token::LeftParen, "Expected '(' after 'escreva'")?;
                let args = self.parse_arguments()?;
                self.consume(Token::RightParen, "Expected ')' after function arguments")?;
                Ok(Expr::FunctionCall {
                    callee: Box::new(Expr::Identifier("escreva".to_string())),
                    args,
                })
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();

                if self.match_token(Token::LeftParen) {
                    let args = self.parse_arguments()?;
                    self.consume(Token::RightParen, "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier(name)),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Token::Funcao => {
                self.advance();
                self.consume(Token::LeftParen, "Expected '(' after 'funcao'")?;
                let params = self.parse_parameters()?;
                self.consume(Token::RightParen, "Expected ')' after parameters")?;
                self.consume(Token::LeftBrace, "Expected '{' after function signature")?;
                let body = self.parse_block()?;
                Ok(Expr::Function { params, body })
            }
            Token::Texto => {
                self.advance();
                self.consume(Token::LeftParen, "Expected '(' after 'texto'")?;
                let args = self.parse_arguments()?;
                self.consume(Token::RightParen, "Expected ')' after function arguments")?;
                Ok(Expr::FunctionCall {
                    callee: Box::new(Expr::Identifier("texto".to_string())),
                    args,
                })
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

    /// Parses an if statement: se condition { statements } [senao se condition { statements }]* [senao { statements }]
    fn parse_if_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Se, "Expected 'se' keyword")?;
        let condition = self.parse_expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after condition")?;
        let then_branch = self.parse_block()?;

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;

        // Check for else-if branches
        while self.match_token(Token::SenaoSe) {
            let else_if_condition = self.parse_expression()?;
            self.consume(Token::LeftBrace, "Expected '{' after else-if condition")?;
            let else_if_statements = self.parse_block()?;
            else_if_branches.push((else_if_condition, else_if_statements));
        }

        // Check for else branch
        if self.match_token(Token::Senao) {
            self.consume(Token::LeftBrace, "Expected '{' after 'senao'")?;
            else_branch = Some(self.parse_block()?);
        }

        if else_if_branches.is_empty() {
            Ok(Statement::If {
                condition,
                then_branch,
                else_branch,
            })
        } else {
            Ok(Statement::IfElseIf {
                condition,
                then_branch,
                else_if_branches,
                else_branch,
            })
        }
    }

    /// Parses a switch statement: escolha value { caso value: statements* [padrao: statements] }
    fn parse_switch_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Escolha, "Expected 'escolha' keyword")?;
        let value = self.parse_expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after switch value")?;

        let mut cases = Vec::new();
        let mut default = None;

        while !self.check(Token::RightBrace) && !self.is_at_end() {
            if self.match_token(Token::Caso) {
                let case_value = self.parse_expression()?;
                self.consume(Token::Colon, "Expected ':' after case value")?;
                let mut case_statements = Vec::new();

                // Parse multiple statements until next case, default, or end of switch
                while !self.check(Token::Caso) && !self.check(Token::Padrao) && !self.check(Token::RightBrace) && !self.is_at_end() {
                    case_statements.push(self.parse_statement()?);
                }

                cases.push((case_value, case_statements));
            } else if self.match_token(Token::Padrao) {
                self.consume(Token::Colon, "Expected ':' after 'padrao'")?;
                let mut default_statements = Vec::new();

                while !self.check(Token::RightBrace) && !self.is_at_end() {
                    default_statements.push(self.parse_statement()?);
                }

                default = Some(default_statements);
            } else {
                return Err(CompilerError::Parser("Expected 'caso' or 'padrao' in switch statement".to_string()));
            }
        }

        self.consume(Token::RightBrace, "Expected '}' after switch body")?;

        Ok(Statement::Switch {
            value,
            cases,
            default,
        })
    }

    /// Parses a while statement: enquanto condition { statements }
    fn parse_while_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Enquanto, "Expected 'enquanto' keyword")?;
        let condition = self.parse_expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after condition")?;
        let body = self.parse_block()?;

        Ok(Statement::While { condition, body })
    }

    /// Parses a do-while statement: fazer { statements } enquanto condition
    fn parse_do_while_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Fazer, "Expected 'fazer' keyword")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'fazer'")?;
        let body = self.parse_block()?;
        self.consume(Token::Enquanto, "Expected 'enquanto' after do block")?;
        let condition = self.parse_expression()?;

        Ok(Statement::DoWhile { body, condition })
    }

    /// Parses a for statement: para [initializer]; [condition]; [increment] { statements }
    fn parse_for_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Para, "Expected 'para' keyword")?;

        let initializer = if self.match_token(Token::Var) {
            let name = self.consume_identifier("Expected variable name")?;
            self.consume(Token::Assign, "Expected '=' after variable name")?;
            let value = self.parse_expression()?;
            Some(Box::new(Statement::VarDeclaration { name, value }))
        } else if self.match_token(Token::Semicolon) {
            None
        } else {
            return Err(CompilerError::Parser("Expected variable declaration or ';' in for loop".to_string()));
        };
        self.consume(Token::Semicolon, "Expected ';' after initializer")?;

        let condition = if !self.check(Token::Semicolon) && !self.check(Token::LeftBrace) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        // Optional semicolon after condition
        if self.check(Token::Semicolon) {
            self.advance();
        }

        let increment = if !self.check(Token::LeftBrace) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        // Optional semicolon after increment
        if self.check(Token::Semicolon) {
            self.advance();
        }

        self.consume(Token::LeftBrace, "Expected '{' after for header")?;
        let body = self.parse_block()?;

        Ok(Statement::For { initializer, condition, increment, body })
    }

    /// Parses a for-each statement: para cada variable in iterable { statements }
    fn parse_for_each_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::ParaCada, "Expected 'para cada' keyword")?;
        let variable = self.consume_identifier("Expected variable name")?;
        self.consume(Token::Ident("de".to_string()), "Expected 'de' after variable")?;
        let iterable = self.parse_expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after iterable")?;
        let body = self.parse_block()?;

        Ok(Statement::ForEach { variable, iterable, body })
    }

    /// Parses a break statement: sustar
    fn parse_break_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Sustar, "Expected 'sustar' keyword")?;
        Ok(Statement::Break)
    }

    /// Parses a continue statement: continua
    fn parse_continue_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Continua, "Expected 'continua' keyword")?;
        Ok(Statement::Continue)
    }

    /// Parses a block of statements enclosed in braces
    fn parse_block(&mut self) -> Result<Vec<Statement>, CompilerError> {
        let mut statements = Vec::new();

        while !self.check(Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}' after block")?;
        Ok(statements)
    }

    /// Parses a named function declaration: funcao name(params) { body }
    fn parse_named_function_declaration(&mut self) -> Result<Statement, CompilerError> {
        // At this point, 'funcao' has already been consumed
        let name = self.consume_identifier("Expected function name")?;
        self.consume(Token::LeftParen, "Expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(Token::RightParen) {
            loop {
                params.push(self.consume_identifier("Expected parameter name")?);

                if self.match_token(Token::Comma) {
                    continue;
                } else {
                    break;
                }
            }
        }

        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        self.consume(Token::LeftBrace, "Expected '{' after function signature")?;

        let body = self.parse_block()?;

        Ok(Statement::FunctionDeclaration { name: Some(name), params, body })
    }

    /// Parses a return statement: retorna [expression];
    fn parse_return_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Token::Retorna, "Expected 'retorna' keyword")?;

        let value = if self.check(Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume(Token::Semicolon, "Expected ';' after return statement")?;

        Ok(Statement::Return(value))
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
            callee: Box::new(Expr::Identifier("escreva".to_string())),
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
