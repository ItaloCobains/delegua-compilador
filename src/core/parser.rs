use crate::core::token::Token;
use crate::core::ast::{Expr, BinaryOp, Statement, Program};
use crate::core::error::CompilerError;
use crate::core::token::Position;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
    eof_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { 
            tokens, 
            current: 0,
            eof_token: Token::EOF(Position::default()),
        }
    }

    fn token_with_pos(token_type: fn(Position) -> Token<'a>) -> Token<'a> {
        token_type(Position { line: 0, column: 0, offset: 0 })
    }

    fn ident_token(s: &'a str) -> Token<'a> {
        Token::Ident(s, Position { line: 0, column: 0, offset: 0 })
    }

    pub fn parse(&mut self) -> Result<Program, CompilerError> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    errors.push(err.clone());
                    if errors.len() > 10 {
                        return Err(CompilerError::Parser(
                            format!("Too many parse errors ({}). First error: {}", 
                                   errors.len(), errors[0])
                        ));
                    }
                    self.synchronize();
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors.into_iter().next().unwrap());
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, CompilerError> {
        match self.current_token() {
            Token::Var(_) => self.parse_variable_declaration(),
            Token::Escreva(_) => self.parse_function_call_statement(),
            Token::Leia(_) => self.parse_function_call_statement(),
            Token::Import(_) => self.parse_import_statement(),
            Token::Se(_) => self.parse_if_statement(),
            Token::Escolha(_) => self.parse_switch_statement(),
            Token::Enquanto(_) => self.parse_while_statement(),
            Token::Fazer(_) => self.parse_do_while_statement(),
            Token::Para(_) => self.parse_for_statement(),
            Token::ParaCada(_) => self.parse_for_each_statement(),
            Token::Sustar(_) => self.parse_break_statement(),
            Token::Continua(_) => self.parse_continue_statement(),
            Token::Funcao(_) => {
                self.advance();
                if let Token::Ident(_, _) = self.current_token() {
                    self.parse_named_function_declaration()
                } else {
                    Err(CompilerError::Parser("Unexpected 'funcao' in statement context".to_string()))
                }
            }
            Token::Retorna(_) => self.parse_return_statement(),
            Token::Ident(_, _) => self.parse_assignment_or_call(),
            Token::Increment(_) => self.parse_prefix_increment_decrement(true),
            Token::Decrement(_) => self.parse_prefix_increment_decrement(false),
            _ => Err(CompilerError::Parser(format!(
                "Unexpected token: {:?}", self.current_token()
            ))),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Var), "Expected 'var' keyword")?;

        let name = self.consume_identifier("Expected variable name")?;
        self.consume(Self::token_with_pos(Token::Assign), "Expected '=' after variable name")?;
        let initializer = self.parse_expression()?;
        self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after variable declaration")?;

        Ok(Statement::VarDeclaration { name, value: initializer })
    }

    fn parse_assignment_or_call(&mut self) -> Result<Statement, CompilerError> {
        let name = self.consume_identifier("Expected identifier")?;

        if self.match_token(Self::token_with_pos(Token::Assign)) {
            let value = self.parse_expression()?;
            self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after assignment")?;
            Ok(Statement::Assignment { name, value })
        } else if self.match_token(Self::token_with_pos(Token::LeftParen)) {
            let args = self.parse_arguments()?;
            self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
            self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after function call")?;
            Ok(Statement::FunctionCall(Expr::FunctionCall {
                callee: Box::new(Expr::Identifier(name)),
                args,
            }))
        } else if matches!(self.current_token(), Token::Increment(_)) {
            self.advance(); // consume ++
            self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after increment")?;
            Ok(Statement::FunctionCall(Expr::Increment {
                operand: Box::new(Expr::Identifier(name)),
                prefix: false, // postfix: x++
            }))
        } else if matches!(self.current_token(), Token::Decrement(_)) {
            self.advance(); // consume --
            self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after decrement")?;
            Ok(Statement::FunctionCall(Expr::Decrement {
                operand: Box::new(Expr::Identifier(name)),
                prefix: false, // postfix: x--
            }))
        } else {
            Err(CompilerError::Parser(
                "Expected '=', '(', '++', or '--' after identifier".to_string()
            ))
        }
    }
    
    fn parse_prefix_increment_decrement(&mut self, is_increment: bool) -> Result<Statement, CompilerError> {
        self.advance(); // consume ++ or --
        
        let name = match self.current_token() {
            Token::Ident(name, _) => name.to_string(),
            _ => return Err(CompilerError::Parser("Expected identifier after ++ or --".to_string())),
        };
        self.advance();
        
        self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after prefix increment/decrement")?;
        
        if is_increment {
            Ok(Statement::FunctionCall(Expr::Increment {
                operand: Box::new(Expr::Identifier(name)),
                prefix: true, // prefix: ++x
            }))
        } else {
            Ok(Statement::FunctionCall(Expr::Decrement {
                operand: Box::new(Expr::Identifier(name)),
                prefix: true, // prefix: --x
            }))
        }
    }

    fn parse_function_call_statement(&mut self) -> Result<Statement, CompilerError> {
        let expr = self.parse_function_call()?;
        self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after function call")?;
        Ok(Statement::FunctionCall(expr))
    }

    fn parse_import_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Import), "Expected 'importar' keyword")?;

        if let Token::String(module, _) = self.current_token() {
            let module = module.to_string();
            self.advance();
            self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after import")?;
            Ok(Statement::Import { module, items: None })
        } else if self.match_token(Self::token_with_pos(Token::LeftBrace)) {
            let mut items = Vec::new();
            while !self.check(Self::token_with_pos(Token::RightBrace)) && !self.is_at_end() {
                if let Token::Ident(name, _) = self.current_token() {
                    let name = name.to_string();
                    items.push(name);
                    self.advance();
                } else {
                    return Err(CompilerError::Parser("Expected identifier in import list".to_string()));
                }
                if self.match_token(Self::token_with_pos(Token::Comma)) {
                    // continue
                } else if !self.check(Self::token_with_pos(Token::RightBrace)) {
                    return Err(CompilerError::Parser("Expected ',' or '}' in import list".to_string()));
                }
            }
            self.consume(Self::token_with_pos(Token::RightBrace), "Expected '}' after import list")?;
            self.consume(Self::ident_token("from"), "Expected 'from' after import list")?;
            if let Token::String(module, _) = self.current_token() {
                let module = module.to_string();
                self.advance();
                self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after import")?;
                Ok(Statement::Import { module, items: Some(items) })
            } else {
                Err(CompilerError::Parser("Expected module name after 'from'".to_string()))
            }
        } else {
            Err(CompilerError::Parser("Expected string or '{' after 'import'".to_string()))
        }
    }

    fn parse_function_call(&mut self) -> Result<Expr, CompilerError> {
        let callee = self.parse_primary()?;

        if let Expr::FunctionCall { .. } = callee {
            return Ok(callee);
        }

        self.consume(Self::token_with_pos(Token::LeftParen), "Expected '(' after function name")?;

        let args = self.parse_arguments()?;

        self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;

        Ok(Expr::FunctionCall { callee: Box::new(callee), args })
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expr>, CompilerError> {
        if matches!(self.current_token(), Token::RightParen(_)) {
            return Ok(Vec::new());
        }

        let mut args = Vec::with_capacity(4); // Pre-allocate for common case
        
        loop {
            args.push(self.parse_expression()?);
            
            if matches!(self.current_token(), Token::Comma(_)) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(args)
    }

    fn parse_parameters(&mut self) -> Result<Vec<String>, CompilerError> {
        if matches!(self.current_token(), Token::RightParen(_)) {
            return Ok(Vec::new());
        }

        let mut params = Vec::with_capacity(4); // Pre-allocate
        
        loop {
            match self.current_token() {
                Token::Ident(param, _) => {
                    params.push(param.to_string());
                    self.advance();
                }
                _ => {
                    let pos = self.current_position();
                    return Err(CompilerError::Parser(
                        format!("Expected parameter name at line {}, column {}", 
                               pos.line, pos.column)
                    ));
                }
            }

            if matches!(self.current_token(), Token::Comma(_)) {
                self.advance();
            } else {
                break;
            }
        }
        
        Ok(params)
    }

    fn parse_expression(&mut self) -> Result<Expr, CompilerError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, CompilerError> {
        let mut left = self.parse_logical_and()?;

        while matches!(self.current_token(), Token::Ou(_)) {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, CompilerError> {
        let mut left = self.parse_comparison()?;

        while matches!(self.current_token(), Token::E(_)) {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, CompilerError> {
        let mut left = self.parse_additive()?;

        while matches!(self.current_token(), 
            Token::Less(_) | Token::Greater(_) | Token::LessEqual(_) | 
            Token::GreaterEqual(_) | Token::Equal(_) | Token::NotEqual(_)
        ) {
            let operator = match self.current_token() {
                Token::Less(_) => BinaryOp::Less,
                Token::Greater(_) => BinaryOp::Greater,
                Token::LessEqual(_) => BinaryOp::LessEqual,
                Token::GreaterEqual(_) => BinaryOp::GreaterEqual,
                Token::Equal(_) => BinaryOp::Equal,
                Token::NotEqual(_) => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_additive()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, CompilerError> {
        let mut left = self.parse_multiplicative()?;

        while matches!(self.current_token(), Token::Plus(_) | Token::Minus(_)) {
            let operator = match self.current_token() {
                Token::Plus(_) => BinaryOp::Add,
                Token::Minus(_) => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, CompilerError> {
        let mut left = self.parse_power()?;

        while matches!(self.current_token(), Token::Multiply(_) | Token::Divide(_) | Token::Modulo(_)) {
            let operator = match self.current_token() {
                Token::Multiply(_) => BinaryOp::Multiply,
                Token::Divide(_) => BinaryOp::Divide,
                Token::Modulo(_) => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_power()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    
    fn parse_power(&mut self) -> Result<Expr, CompilerError> {
        let mut left = self.parse_unary()?;
        
        // Right associative: 2**3**2 = 2**(3**2) = 512
        if matches!(self.current_token(), Token::Power(_)) {
            self.advance();
            let right = self.parse_power()?; // Right associative recursion
            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOp::Power,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, CompilerError> {
        match self.current_token() {
            Token::Plus(_) => {
                self.advance();
                self.parse_unary()
            }
            Token::Minus(_) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    operator: BinaryOp::Subtract,
                    operand: Box::new(operand),
                })
            }
            Token::Nao(_) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    operator: BinaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            Token::Increment(_) => {
                self.advance();
                let operand = self.parse_postfix()?;
                Ok(Expr::Increment {
                    operand: Box::new(operand),
                    prefix: true,
                })
            }
            Token::Decrement(_) => {
                self.advance();
                let operand = self.parse_postfix()?;
                Ok(Expr::Decrement {
                    operand: Box::new(operand),
                    prefix: true,
                })
            }
            _ => self.parse_postfix()
        }
    }
    
    fn parse_postfix(&mut self) -> Result<Expr, CompilerError> {
        let mut expr = self.parse_primary()?;
        
        // Handle postfix increment/decrement
        loop {
            match self.current_token() {
                Token::Increment(_) => {
                    self.advance();
                    expr = Expr::Increment {
                        operand: Box::new(expr),
                        prefix: false,
                    };
                }
                Token::Decrement(_) => {
                    self.advance();
                    expr = Expr::Decrement {
                        operand: Box::new(expr),
                        prefix: false,
                    };
                }
                Token::LeftBracket(_) => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.consume(Self::token_with_pos(Token::RightBracket), "Expected ']' after array index")?;
                    expr = Expr::Index {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Token::Dot(_) => {
                    self.advance();
                    if let Token::Ident(name, _) = self.current_token() {
                        let property = name.to_string();
                        self.advance();
                        expr = Expr::PropertyAccess {
                            object: Box::new(expr),
                            property,
                        };
                    } else {
                        return Err(CompilerError::Parser("Expected property name after '.'".to_string()));
                    }
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, CompilerError> {
        match *self.current_token() {
            Token::Number(n, _) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::String(ref s, _) => {
                let s = s.to_string();
                self.advance();
                Ok(Expr::String(s))
            }
            Token::Verdadeiro(_) => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Token::Falso(_) => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            Token::Escreva(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("escreva".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("escreva".to_string()))
                }
            }
            Token::Ident(name, _) => {
                let name = name.to_string();
                self.advance();

                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier(name)),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Token::Funcao(_) => {
                self.advance();
                self.consume(Self::token_with_pos(Token::LeftParen), "Expected '(' after 'funcao'")?;
                let params = self.parse_parameters()?;
                self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after parameters")?;
                self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after function signature")?;
                let body = self.parse_block()?;
                Ok(Expr::Function { params, body })
            }
            Token::Texto(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("texto".to_string())),
                        args,
                    })
                } else {
                    // Treat as variable identifier
                    Ok(Expr::Identifier("texto".to_string()))
                }
            }
            Token::Leia(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("leia".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("leia".to_string()))
                }
            }
            Token::Comprimento(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("comprimento".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("comprimento".to_string()))
                }
            }
            Token::Maiuscula(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("maiuscula".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("maiuscula".to_string()))
                }
            }
            Token::Minuscula(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("minuscula".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("minuscula".to_string()))
                }
            }
            Token::Absoluto(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("absoluto".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("absoluto".to_string()))
                }
            }
            Token::Potencia(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("potencia".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("potencia".to_string()))
                }
            }
            Token::RaizQuadrada(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Token::LeftParen)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("raiz_quadrada".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("raiz_quadrada".to_string()))
                }
            }
            Token::LeftParen(_) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after expression")?;
                Ok(expr)
            }
            Token::LeftBracket(_) => {
                self.advance();
                let mut elements = Vec::new();

                if !matches!(self.current_token(), Token::RightBracket(_)) {
                    loop {
                        elements.push(self.parse_expression()?);

                        if matches!(self.current_token(), Token::Comma(_)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.consume(Self::token_with_pos(Token::RightBracket), "Expected ']' after array elements")?;
                Ok(Expr::Array { elements })
            }
            Token::LeftBrace(_) => {
                self.advance();
                let mut properties = Vec::new();

                if !matches!(self.current_token(), Token::RightBrace(_)) {
                    loop {
                        let key = match self.current_token() {
                            Token::Ident(name, _) => {
                                let key = name.to_string();
                                self.advance();
                                key
                            }
                            Token::String(name, _) => {
                                let key = name.to_string();
                                self.advance();
                                key
                            }
                            _ => return Err(CompilerError::Parser("Expected property name in object literal".to_string())),
                        };

                        self.consume(Self::token_with_pos(Token::Colon), "Expected ':' after property name")?;
                        let value = self.parse_expression()?;
                        properties.push((key, value));

                        if matches!(self.current_token(), Token::Comma(_)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.consume(Self::token_with_pos(Token::RightBrace), "Expected '}' after object properties")?;
                Ok(Expr::Object { properties })
            }
            _ => Err(CompilerError::Parser(format!(
                "Unexpected token in expression: {:?}", self.current_token()
            ))),
        }
    }


    #[inline]
    fn current_token(&self) -> &Token<'a> {
        self.tokens.get(self.current).unwrap_or(&self.eof_token)
    }

    #[inline]
    fn previous_token(&self) -> &Token<'a> {
        if self.current > 0 {
            self.tokens.get(self.current - 1).unwrap_or(&self.eof_token)
        } else {
            &self.eof_token
        }
    }

    #[inline]
    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }

    fn consume(&mut self, expected: Token, message: &str) -> Result<String, CompilerError> {
        if self.check(expected.clone()) {
            let result = match self.current_token() {
                Token::Ident(name, _) => name.to_string(),
                Token::String(s, _) => s.to_string(),
                _ => String::new(),
            };
            self.advance();
            Ok(result)
        } else {
            let pos = self.current_position();
            Err(CompilerError::Parser(
                format!("{} at line {}, column {}. Found: {:?}", 
                       message, pos.line, pos.column, self.current_token())
            ))
        }
    }

    fn token_to_identifier_name(&self, token: &Token) -> Option<String> {
        match token {
            Token::Ident(name, _) => Some(name.to_string()),
            Token::Escreva(_) => Some("escreva".to_string()),
            Token::Texto(_) => Some("texto".to_string()),
            Token::Leia(_) => Some("leia".to_string()),
            Token::Comprimento(_) => Some("comprimento".to_string()),
            Token::Maiuscula(_) => Some("maiuscula".to_string()),
            Token::Minuscula(_) => Some("minuscula".to_string()),
            Token::Absoluto(_) => Some("absoluto".to_string()),
            Token::Potencia(_) => Some("potencia".to_string()),
            Token::RaizQuadrada(_) => Some("raiz_quadrada".to_string()),
            _ => None,
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String, CompilerError> {
        if let Some(name) = self.token_to_identifier_name(&self.current_token()) {
            self.advance();
            Ok(name)
        } else {
            Err(CompilerError::Parser(message.to_string()))
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


    #[inline]
    fn check(&self, expected: Token) -> bool {
        !self.is_at_end() && self.token_matches(&expected)
    }
    
    #[inline]
    fn token_matches(&self, expected: &Token) -> bool {
        match (self.current_token(), expected) {
            (Token::String(_, _), Token::String(_, _)) => true,
            (Token::Ident(_, _), Token::Ident(_, _)) => true,
            (Token::Number(_, _), Token::Number(_, _)) => true,
            _ => std::mem::discriminant(self.current_token()) == std::mem::discriminant(expected),
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Token::EOF(_))
    }

    fn current_position(&self) -> Position {
        match self.current_token() {
            Token::Number(_, pos) | Token::String(_, pos) | Token::Ident(_, pos) |
            Token::Var(pos) | Token::Escreva(pos) | Token::Import(pos) |
            Token::Se(pos) | Token::Senao(pos) | Token::EOF(pos) => *pos,
            _ => Position::default(),
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        
        while !self.is_at_end() {
            if matches!(self.previous_token(), Token::Semicolon(_)) {
                return;
            }
            
            match self.current_token() {
                Token::Var(_) | Token::Funcao(_) | Token::Se(_) |
                Token::Para(_) | Token::Enquanto(_) | Token::Retorna(_) => return,
                _ => { self.advance(); }
            }
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Se), "Expected 'se' keyword")?;
        let condition = self.parse_expression()?;
        self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after condition")?;
        let then_branch = self.parse_block()?;

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;

        while self.match_token(Self::token_with_pos(Token::SenaoSe)) {
            let else_if_condition = self.parse_expression()?;
            self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after else-if condition")?;
            let else_if_statements = self.parse_block()?;
            else_if_branches.push((else_if_condition, else_if_statements));
        }

        if self.match_token(Self::token_with_pos(Token::Senao)) {
            self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after 'senao'")?;
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

    fn parse_switch_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Escolha), "Expected 'escolha' keyword")?;
        let value = self.parse_expression()?;
        self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after switch value")?;

        let mut cases = Vec::new();
        let mut default = None;

        while !self.check(Self::token_with_pos(Token::RightBrace)) && !self.is_at_end() {
            if self.match_token(Self::token_with_pos(Token::Caso)) {
                let case_value = self.parse_expression()?;
                self.consume(Self::token_with_pos(Token::Colon), "Expected ':' after case value")?;
                let mut case_statements = Vec::new();

                while !self.check(Self::token_with_pos(Token::Caso)) && !self.check(Self::token_with_pos(Token::Padrao)) && !self.check(Self::token_with_pos(Token::RightBrace)) && !self.is_at_end() {
                    case_statements.push(self.parse_statement()?);
                }

                cases.push((case_value, case_statements));
            } else if self.match_token(Self::token_with_pos(Token::Padrao)) {
                self.consume(Self::token_with_pos(Token::Colon), "Expected ':' after 'padrao'")?;
                let mut default_statements = Vec::new();

                while !self.check(Self::token_with_pos(Token::RightBrace)) && !self.is_at_end() {
                    default_statements.push(self.parse_statement()?);
                }

                default = Some(default_statements);
            } else {
                return Err(CompilerError::Parser("Expected 'caso' or 'padrao' in switch statement".to_string()));
            }
        }

        self.consume(Self::token_with_pos(Token::RightBrace), "Expected '}' after switch body")?;

        Ok(Statement::Switch {
            value,
            cases,
            default,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Enquanto), "Expected 'enquanto' keyword")?;
        let condition = self.parse_expression()?;
        self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after condition")?;
        let body = self.parse_block()?;

        Ok(Statement::While { condition, body })
    }

    fn parse_do_while_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Fazer), "Expected 'fazer' keyword")?;
        self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after 'fazer'")?;
        let body = self.parse_block()?;
        self.consume(Self::token_with_pos(Token::Enquanto), "Expected 'enquanto' after do block")?;
        let condition = self.parse_expression()?;

        Ok(Statement::DoWhile { body, condition })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Para), "Expected 'para' keyword")?;
        self.consume(Self::token_with_pos(Token::LeftParen), "Expected '(' after 'para'")?;

        let initializer = if self.match_token(Self::token_with_pos(Token::Var)) {
            let name = self.consume_identifier("Expected variable name")?;
            self.consume(Self::token_with_pos(Token::Assign), "Expected '=' after variable name")?;
            let value = self.parse_expression()?;
            Some(Box::new(Statement::VarDeclaration { name, value }))
        } else if self.match_token(Self::token_with_pos(Token::Semicolon)) {
            None
        } else {
            return Err(CompilerError::Parser("Expected variable declaration or ';' in for loop".to_string()));
        };
        self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after initializer")?;

        let condition = if !self.check(Self::token_with_pos(Token::Semicolon)) && !self.check(Self::token_with_pos(Token::LeftBrace)) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if self.check(Self::token_with_pos(Token::Semicolon)) {
            self.advance();
        }

        let increment = if !self.check(Self::token_with_pos(Token::LeftBrace)) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if self.check(Self::token_with_pos(Token::Semicolon)) {
            self.advance();
        }

        self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after for header")?;
        self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after for header")?;
        let body = self.parse_block()?;

        Ok(Statement::For { initializer, condition, increment, body })
    }

    fn parse_for_each_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::ParaCada), "Expected 'para cada' keyword")?;
        let variable = self.consume_identifier("Expected variable name")?;
        self.consume(Self::ident_token("de"), "Expected 'de' after variable")?;
        let iterable = self.parse_expression()?;
        self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after iterable")?;
        let body = self.parse_block()?;

        Ok(Statement::ForEach { variable, iterable, body })
    }

    fn parse_break_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Sustar), "Expected 'sustar' keyword")?;
        Ok(Statement::Break)
    }

    fn parse_continue_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Continua), "Expected 'continua' keyword")?;
        Ok(Statement::Continue)
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, CompilerError> {
        let mut statements = Vec::new();

        while !self.check(Self::token_with_pos(Token::RightBrace)) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(Self::token_with_pos(Token::RightBrace), "Expected '}' after block")?;
        Ok(statements)
    }

    fn parse_named_function_declaration(&mut self) -> Result<Statement, CompilerError> {
        let name = self.consume_identifier("Expected function name")?;
        self.consume(Self::token_with_pos(Token::LeftParen), "Expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(Self::token_with_pos(Token::RightParen)) {
            loop {
                params.push(self.consume_identifier("Expected parameter name")?);

                if self.match_token(Self::token_with_pos(Token::Comma)) {
                    continue;
                } else {
                    break;
                }
            }
        }

        self.consume(Self::token_with_pos(Token::RightParen), "Expected ')' after parameters")?;
        self.consume(Self::token_with_pos(Token::LeftBrace), "Expected '{' after function signature")?;

        let body = self.parse_block()?;

        Ok(Statement::FunctionDeclaration { name: Some(name), params, body })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Token::Retorna), "Expected 'retorna' keyword")?;

        let value = if self.check(Self::token_with_pos(Token::Semicolon)) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume(Self::token_with_pos(Token::Semicolon), "Expected ';' after return statement")?;

        Ok(Statement::Return(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::lexer::Lexer;

    #[test]
    fn test_parse_variable_declaration() {
        let mut lexer = Lexer::new();
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
    fn test_parse_complex_program() {
        let mut lexer = Lexer::new();
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
