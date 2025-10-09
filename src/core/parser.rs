use crate::core::token::Simbolo;
use crate::core::ast::{Expr, BinaryOp, Statement, Program};
use crate::core::error::CompilerError;
use crate::core::token::Posicao;

pub struct Parser<'a> {
    tokens: Vec<Simbolo<'a>>,
    current: usize,
    eof_token: Simbolo<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Simbolo<'a>>) -> Self {
        Parser { 
            tokens, 
            current: 0,
            eof_token: Simbolo::EOF(Posicao::default()),
        }
    }

    fn token_with_pos(token_type: fn(Posicao) -> Simbolo<'a>) -> Simbolo<'a> {
        token_type(Posicao { linha: 0, coluna: 0, deslocamento: 0 })
    }

    fn ident_token(s: &'a str) -> Simbolo<'a> {
        Simbolo::Identificador(s, Posicao { linha: 0, coluna: 0, deslocamento: 0 })
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
            Simbolo::Variavel(_) => self.parse_variable_declaration(),
            Simbolo::Escreva(_) => self.parse_function_call_statement(),
            Simbolo::Leia(_) => self.parse_function_call_statement(),
            Simbolo::Importacao(_) => self.parse_import_statement(),
            Simbolo::Se(_) => self.parse_if_statement(),
            Simbolo::Escolha(_) => self.parse_switch_statement(),
            Simbolo::Enquanto(_) => self.parse_while_statement(),
            Simbolo::Fazer(_) => self.parse_do_while_statement(),
            Simbolo::Para(_) => self.parse_for_statement(),
            Simbolo::ParaCada(_) => self.parse_for_each_statement(),
            Simbolo::Sustar(_) => self.parse_break_statement(),
            Simbolo::Continua(_) => self.parse_continue_statement(),
            Simbolo::Funcao(_) => {
                self.advance();
                if let Simbolo::Identificador(_, _) = self.current_token() {
                    self.parse_named_function_declaration()
                } else {
                    Err(CompilerError::Parser("Unexpected 'funcao' in statement context".to_string()))
                }
            }
            Simbolo::Retorna(_) => self.parse_return_statement(),
            Simbolo::Identificador(_, _) => self.parse_assignment_or_call(),
            Simbolo::Incremento(_) => self.parse_prefix_increment_decrement(true),
            Simbolo::Decremento(_) => self.parse_prefix_increment_decrement(false),
            _ => Err(CompilerError::Parser(format!(
                "Unexpected token: {:?}", self.current_token()
            ))),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::Variavel), "Expected 'var' keyword")?;

        let name = self.consume_identifier("Expected variable name")?;
        self.consume(Self::token_with_pos(Simbolo::Atribuir), "Expected '=' after variable name")?;
        let initializer = self.parse_expression()?;
        self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after variable declaration")?;

        Ok(Statement::VarDeclaration { name, value: initializer })
    }

    fn parse_assignment_or_call(&mut self) -> Result<Statement, CompilerError> {
        let name = self.consume_identifier("Expected identifier")?;

        if self.match_token(Self::token_with_pos(Simbolo::Atribuir)) {
            let value = self.parse_expression()?;
            self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after assignment")?;
            Ok(Statement::Assignment { name, value })
        } else if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
            let args = self.parse_arguments()?;
            self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
            self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after function call")?;
            Ok(Statement::FunctionCall(Expr::FunctionCall {
                callee: Box::new(Expr::Identifier(name)),
                args,
            }))
        } else if matches!(self.current_token(), Simbolo::Incremento(_)) {
            self.advance(); // consume ++
            self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after increment")?;
            Ok(Statement::FunctionCall(Expr::Increment {
                operand: Box::new(Expr::Identifier(name)),
                prefix: false, // postfix: x++
            }))
        } else if matches!(self.current_token(), Simbolo::Decremento(_)) {
            self.advance(); // consume --
            self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after decrement")?;
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
            Simbolo::Identificador(name, _) => name.to_string(),
            _ => return Err(CompilerError::Parser("Expected identifier after ++ or --".to_string())),
        };
        self.advance();
        
        self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after prefix increment/decrement")?;
        
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
        self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after function call")?;
        Ok(Statement::FunctionCall(expr))
    }

    fn parse_import_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::Importacao), "Expected 'importar' keyword")?;

        if let Simbolo::Texto(module, _) = self.current_token() {
            let module = module.to_string();
            self.advance();
            self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after import")?;
            Ok(Statement::Import { module, items: None })
        } else if self.match_token(Self::token_with_pos(Simbolo::ChaveEsquerda)) {
            let mut items = Vec::new();
            while !self.check(Self::token_with_pos(Simbolo::ChaveDireita)) && !self.is_at_end() {
                if let Simbolo::Identificador(name, _) = self.current_token() {
                    let name = name.to_string();
                    items.push(name);
                    self.advance();
                } else {
                    return Err(CompilerError::Parser("Expected identifier in import list".to_string()));
                }
                if self.match_token(Self::token_with_pos(Simbolo::Virgula)) {
                    // continue
                } else if !self.check(Self::token_with_pos(Simbolo::ChaveDireita)) {
                    return Err(CompilerError::Parser("Expected ',' or '}' in import list".to_string()));
                }
            }
            self.consume(Self::token_with_pos(Simbolo::ChaveDireita), "Expected '}' after import list")?;
            self.consume(Self::ident_token("from"), "Expected 'from' after import list")?;
            if let Simbolo::Texto(module, _) = self.current_token() {
                let module = module.to_string();
                self.advance();
                self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after import")?;
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

        self.consume(Self::token_with_pos(Simbolo::ParenteseEsquerdo), "Expected '(' after function name")?;

        let args = self.parse_arguments()?;

        self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;

        Ok(Expr::FunctionCall { callee: Box::new(callee), args })
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expr>, CompilerError> {
        if matches!(self.current_token(), Simbolo::ParenteseDireito(_)) {
            return Ok(Vec::new());
        }

        let mut args = Vec::with_capacity(4); // Pre-allocate for common case
        
        loop {
            args.push(self.parse_expression()?);
            
            if matches!(self.current_token(), Simbolo::Virgula(_)) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(args)
    }

    fn parse_parameters(&mut self) -> Result<Vec<String>, CompilerError> {
        if matches!(self.current_token(), Simbolo::ParenteseDireito(_)) {
            return Ok(Vec::new());
        }

        let mut params = Vec::with_capacity(4); // Pre-allocate
        
        loop {
            match self.current_token() {
                Simbolo::Identificador(param, _) => {
                    params.push(param.to_string());
                    self.advance();
                }
                _ => {
                    let pos = self.current_position();
                    return Err(CompilerError::Parser(
                        format!("Expected parameter name at line {}, column {}", 
                               pos.linha, pos.coluna)
                    ));
                }
            }

            if matches!(self.current_token(), Simbolo::Virgula(_)) {
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

        while matches!(self.current_token(), Simbolo::Ou(_)) {
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

        while matches!(self.current_token(), Simbolo::E(_)) {
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
            Simbolo::Menor(_) | Simbolo::Maior(_) | Simbolo::MenorIgual(_) | 
            Simbolo::MaiorIgual(_) | Simbolo::Igual(_) | Simbolo::NaoIgual(_)
        ) {
            let operator = match self.current_token() {
                Simbolo::Menor(_) => BinaryOp::Less,
                Simbolo::Maior(_) => BinaryOp::Greater,
                Simbolo::MenorIgual(_) => BinaryOp::LessEqual,
                Simbolo::MaiorIgual(_) => BinaryOp::GreaterEqual,
                Simbolo::Igual(_) => BinaryOp::Equal,
                Simbolo::NaoIgual(_) => BinaryOp::NotEqual,
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

        while matches!(self.current_token(), Simbolo::Adicao(_) | Simbolo::Subtracao(_)) {
            let operator = match self.current_token() {
                Simbolo::Adicao(_) => BinaryOp::Add,
                Simbolo::Subtracao(_) => BinaryOp::Subtract,
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

        while matches!(self.current_token(), Simbolo::Multiplicacao(_) | Simbolo::Divisao(_) | Simbolo::Modulo(_)) {
            let operator = match self.current_token() {
                Simbolo::Multiplicacao(_) => BinaryOp::Multiply,
                Simbolo::Divisao(_) => BinaryOp::Divide,
                Simbolo::Modulo(_) => BinaryOp::Modulo,
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
        if matches!(self.current_token(), Simbolo::Potencia(_)) {
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
            Simbolo::Adicao(_) => {
                self.advance();
                self.parse_unary()
            }
            Simbolo::Subtracao(_) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    operator: BinaryOp::Subtract,
                    operand: Box::new(operand),
                })
            }
            Simbolo::Nao(_) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    operator: BinaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            Simbolo::Incremento(_) => {
                self.advance();
                let operand = self.parse_postfix()?;
                Ok(Expr::Increment {
                    operand: Box::new(operand),
                    prefix: true,
                })
            }
            Simbolo::Decremento(_) => {
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
                Simbolo::Incremento(_) => {
                    self.advance();
                    expr = Expr::Increment {
                        operand: Box::new(expr),
                        prefix: false,
                    };
                }
                Simbolo::Decremento(_) => {
                    self.advance();
                    expr = Expr::Decrement {
                        operand: Box::new(expr),
                        prefix: false,
                    };
                }
                Simbolo::ColcheteEsquerdo(_) => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.consume(Self::token_with_pos(Simbolo::ColcheteDireito), "Expected ']' after array index")?;
                    expr = Expr::Index {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Simbolo::Ponto(_) => {
                    self.advance();
                    if let Simbolo::Identificador(name, _) = self.current_token() {
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
            Simbolo::Number(n, _) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Simbolo::Texto(ref s, _) => {
                let s = s.to_string();
                self.advance();
                Ok(Expr::String(s))
            }
            Simbolo::Verdadeiro(_) => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Simbolo::Falso(_) => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            Simbolo::Escreva(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("escreva".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("escreva".to_string()))
                }
            }
            Simbolo::Identificador(name, _) => {
                let name = name.to_string();
                self.advance();

                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier(name)),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Simbolo::Funcao(_) => {
                self.advance();
                self.consume(Self::token_with_pos(Simbolo::ParenteseEsquerdo), "Expected '(' after 'funcao'")?;
                let params = self.parse_parameters()?;
                self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after parameters")?;
                self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after function signature")?;
                let body = self.parse_block()?;
                Ok(Expr::Function { params, body })
            }
            Simbolo::TextoFuncao(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("texto".to_string())),
                        args,
                    })
                } else {
                    // Treat as variable identifier
                    Ok(Expr::Identifier("texto".to_string()))
                }
            }
            Simbolo::Leia(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("leia".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("leia".to_string()))
                }
            }
            Simbolo::Comprimento(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("comprimento".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("comprimento".to_string()))
                }
            }
            Simbolo::Maiuscula(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("maiuscula".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("maiuscula".to_string()))
                }
            }
            Simbolo::Minuscula(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("minuscula".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("minuscula".to_string()))
                }
            }
            Simbolo::Absoluto(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("absoluto".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("absoluto".to_string()))
                }
            }
            Simbolo::PotenciaFuncao(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("potencia".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("potencia".to_string()))
                }
            }
            Simbolo::RaizQuadrada(_) => {
                self.advance();
                if self.match_token(Self::token_with_pos(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expr::FunctionCall {
                        callee: Box::new(Expr::Identifier("raiz_quadrada".to_string())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier("raiz_quadrada".to_string()))
                }
            }
            Simbolo::ParenteseEsquerdo(_) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after expression")?;
                Ok(expr)
            }
            Simbolo::ColcheteEsquerdo(_) => {
                self.advance();
                let mut elements = Vec::new();

                if !matches!(self.current_token(), Simbolo::ColcheteDireito(_)) {
                    loop {
                        elements.push(self.parse_expression()?);

                        if matches!(self.current_token(), Simbolo::Virgula(_)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.consume(Self::token_with_pos(Simbolo::ColcheteDireito), "Expected ']' after array elements")?;
                Ok(Expr::Array { elements })
            }
            Simbolo::ChaveEsquerda(_) => {
                self.advance();
                let mut properties = Vec::new();

                if !matches!(self.current_token(), Simbolo::ChaveDireita(_)) {
                    loop {
                        let key = match self.current_token() {
                            Simbolo::Identificador(name, _) => {
                                let key = name.to_string();
                                self.advance();
                                key
                            }
                            Simbolo::Texto(name, _) => {
                                let key = name.to_string();
                                self.advance();
                                key
                            }
                            _ => return Err(CompilerError::Parser("Expected property name in object literal".to_string())),
                        };

                        self.consume(Self::token_with_pos(Simbolo::DoisPontos), "Expected ':' after property name")?;
                        let value = self.parse_expression()?;
                        properties.push((key, value));

                        if matches!(self.current_token(), Simbolo::Virgula(_)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.consume(Self::token_with_pos(Simbolo::ChaveDireita), "Expected '}' after object properties")?;
                Ok(Expr::Object { properties })
            }
            _ => Err(CompilerError::Parser(format!(
                "Unexpected token in expression: {:?}", self.current_token()
            ))),
        }
    }


    #[inline]
    fn current_token(&self) -> &Simbolo<'a> {
        self.tokens.get(self.current).unwrap_or(&self.eof_token)
    }

    #[inline]
    fn previous_token(&self) -> &Simbolo<'a> {
        if self.current > 0 {
            self.tokens.get(self.current - 1).unwrap_or(&self.eof_token)
        } else {
            &self.eof_token
        }
    }

    #[inline]
    fn advance(&mut self) -> &Simbolo<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }

    fn consume(&mut self, expected: Simbolo, message: &str) -> Result<String, CompilerError> {
        if self.check(expected.clone()) {
            let result = match self.current_token() {
                Simbolo::Identificador(name, _) => name.to_string(),
                Simbolo::Texto(s, _) => s.to_string(),
                _ => String::new(),
            };
            self.advance();
            Ok(result)
        } else {
            let pos = self.current_position();
            Err(CompilerError::Parser(
                format!("{} at line {}, column {}. Found: {:?}", 
                       message, pos.linha, pos.coluna, self.current_token())
            ))
        }
    }

    fn token_to_identifier_name(&self, token: &Simbolo) -> Option<String> {
        match token {
            Simbolo::Identificador(name, _) => Some(name.to_string()),
            Simbolo::Escreva(_) => Some("escreva".to_string()),
            Simbolo::TextoFuncao(_) => Some("texto".to_string()),
            Simbolo::Leia(_) => Some("leia".to_string()),
            Simbolo::Comprimento(_) => Some("comprimento".to_string()),
            Simbolo::Maiuscula(_) => Some("maiuscula".to_string()),
            Simbolo::Minuscula(_) => Some("minuscula".to_string()),
            Simbolo::Absoluto(_) => Some("absoluto".to_string()),
            Simbolo::PotenciaFuncao(_) => Some("potencia".to_string()),
            Simbolo::RaizQuadrada(_) => Some("raiz_quadrada".to_string()),
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



    fn match_token(&mut self, expected: Simbolo) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }


    #[inline]
    fn check(&self, expected: Simbolo) -> bool {
        !self.is_at_end() && self.token_matches(&expected)
    }
    
    #[inline]
    fn token_matches(&self, expected: &Simbolo) -> bool {
        match (self.current_token(), expected) {
            (Simbolo::Texto(_, _), Simbolo::Texto(_, _)) => true,
            (Simbolo::Identificador(_, _), Simbolo::Identificador(_, _)) => true,
            (Simbolo::Number(_, _), Simbolo::Number(_, _)) => true,
            _ => std::mem::discriminant(self.current_token()) == std::mem::discriminant(expected),
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Simbolo::EOF(_))
    }

    fn current_position(&self) -> Posicao {
        match self.current_token() {
            Simbolo::Number(_, pos) | Simbolo::Texto(_, pos) | Simbolo::Identificador(_, pos) |
            Simbolo::Variavel(pos) | Simbolo::Escreva(pos) | Simbolo::Importacao(pos) |
            Simbolo::Se(pos) | Simbolo::Senao(pos) | Simbolo::EOF(pos) => *pos,
            _ => Posicao::default(),
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        
        while !self.is_at_end() {
            if matches!(self.previous_token(), Simbolo::PontoEVirgula(_)) {
                return;
            }
            
            match self.current_token() {
                Simbolo::Variavel(_) | Simbolo::Funcao(_) | Simbolo::Se(_) |
                Simbolo::Para(_) | Simbolo::Enquanto(_) | Simbolo::Retorna(_) => return,
                _ => { self.advance(); }
            }
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::Se), "Expected 'se' keyword")?;
        let condition = self.parse_expression()?;
        self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after condition")?;
        let then_branch = self.parse_block()?;

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;

        while self.match_token(Self::token_with_pos(Simbolo::SenaoSe)) {
            let else_if_condition = self.parse_expression()?;
            self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after else-if condition")?;
            let else_if_statements = self.parse_block()?;
            else_if_branches.push((else_if_condition, else_if_statements));
        }

        if self.match_token(Self::token_with_pos(Simbolo::Senao)) {
            self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after 'senao'")?;
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
        self.consume(Self::token_with_pos(Simbolo::Escolha), "Expected 'escolha' keyword")?;
        let value = self.parse_expression()?;
        self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after switch value")?;

        let mut cases = Vec::new();
        let mut default = None;

        while !self.check(Self::token_with_pos(Simbolo::ChaveDireita)) && !self.is_at_end() {
            if self.match_token(Self::token_with_pos(Simbolo::Caso)) {
                let case_value = self.parse_expression()?;
                self.consume(Self::token_with_pos(Simbolo::DoisPontos), "Expected ':' after case value")?;
                let mut case_statements = Vec::new();

                while !self.check(Self::token_with_pos(Simbolo::Caso)) && !self.check(Self::token_with_pos(Simbolo::Padrao)) && !self.check(Self::token_with_pos(Simbolo::ChaveDireita)) && !self.is_at_end() {
                    case_statements.push(self.parse_statement()?);
                }

                cases.push((case_value, case_statements));
            } else if self.match_token(Self::token_with_pos(Simbolo::Padrao)) {
                self.consume(Self::token_with_pos(Simbolo::DoisPontos), "Expected ':' after 'padrao'")?;
                let mut default_statements = Vec::new();

                while !self.check(Self::token_with_pos(Simbolo::ChaveDireita)) && !self.is_at_end() {
                    default_statements.push(self.parse_statement()?);
                }

                default = Some(default_statements);
            } else {
                return Err(CompilerError::Parser("Expected 'caso' or 'padrao' in switch statement".to_string()));
            }
        }

        self.consume(Self::token_with_pos(Simbolo::ChaveDireita), "Expected '}' after switch body")?;

        Ok(Statement::Switch {
            value,
            cases,
            default,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::Enquanto), "Expected 'enquanto' keyword")?;
        let condition = self.parse_expression()?;
        self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after condition")?;
        let body = self.parse_block()?;

        Ok(Statement::While { condition, body })
    }

    fn parse_do_while_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::Fazer), "Expected 'fazer' keyword")?;
        self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after 'fazer'")?;
        let body = self.parse_block()?;
        self.consume(Self::token_with_pos(Simbolo::Enquanto), "Expected 'enquanto' after do block")?;
        let condition = self.parse_expression()?;

        Ok(Statement::DoWhile { body, condition })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::Para), "Expected 'para' keyword")?;
        self.consume(Self::token_with_pos(Simbolo::ParenteseEsquerdo), "Expected '(' after 'para'")?;

        let initializer = if self.match_token(Self::token_with_pos(Simbolo::Variavel)) {
            let name = self.consume_identifier("Expected variable name")?;
            self.consume(Self::token_with_pos(Simbolo::Atribuir), "Expected '=' after variable name")?;
            let value = self.parse_expression()?;
            Some(Box::new(Statement::VarDeclaration { name, value }))
        } else if self.match_token(Self::token_with_pos(Simbolo::PontoEVirgula)) {
            None
        } else {
            return Err(CompilerError::Parser("Expected variable declaration or ';' in for loop".to_string()));
        };
        self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after initializer")?;

        let condition = if !self.check(Self::token_with_pos(Simbolo::PontoEVirgula)) && !self.check(Self::token_with_pos(Simbolo::ChaveEsquerda)) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if self.check(Self::token_with_pos(Simbolo::PontoEVirgula)) {
            self.advance();
        }

        let increment = if !self.check(Self::token_with_pos(Simbolo::ChaveEsquerda)) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if self.check(Self::token_with_pos(Simbolo::PontoEVirgula)) {
            self.advance();
        }

        self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after for header")?;
        self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after for header")?;
        let body = self.parse_block()?;

        Ok(Statement::For { initializer, condition, increment, body })
    }

    fn parse_for_each_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::ParaCada), "Expected 'para cada' keyword")?;
        let variable = self.consume_identifier("Expected variable name")?;
        self.consume(Self::ident_token("de"), "Expected 'de' after variable")?;
        let iterable = self.parse_expression()?;
        self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after iterable")?;
        let body = self.parse_block()?;

        Ok(Statement::ForEach { variable, iterable, body })
    }

    fn parse_break_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::Sustar), "Expected 'sustar' keyword")?;
        Ok(Statement::Break)
    }

    fn parse_continue_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::Continua), "Expected 'continua' keyword")?;
        Ok(Statement::Continue)
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, CompilerError> {
        let mut statements = Vec::new();

        while !self.check(Self::token_with_pos(Simbolo::ChaveDireita)) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(Self::token_with_pos(Simbolo::ChaveDireita), "Expected '}' after block")?;
        Ok(statements)
    }

    fn parse_named_function_declaration(&mut self) -> Result<Statement, CompilerError> {
        let name = self.consume_identifier("Expected function name")?;
        self.consume(Self::token_with_pos(Simbolo::ParenteseEsquerdo), "Expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(Self::token_with_pos(Simbolo::ParenteseDireito)) {
            loop {
                params.push(self.consume_identifier("Expected parameter name")?);

                if self.match_token(Self::token_with_pos(Simbolo::Virgula)) {
                    continue;
                } else {
                    break;
                }
            }
        }

        self.consume(Self::token_with_pos(Simbolo::ParenteseDireito), "Expected ')' after parameters")?;
        self.consume(Self::token_with_pos(Simbolo::ChaveEsquerda), "Expected '{' after function signature")?;

        let body = self.parse_block()?;

        Ok(Statement::FunctionDeclaration { name: Some(name), params, body })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, CompilerError> {
        self.consume(Self::token_with_pos(Simbolo::Retorna), "Expected 'retorna' keyword")?;

        let value = if self.check(Self::token_with_pos(Simbolo::PontoEVirgula)) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume(Self::token_with_pos(Simbolo::PontoEVirgula), "Expected ';' after return statement")?;

        Ok(Statement::Return(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::lexer::Lexador;

    #[test]
    fn test_parse_variable_declaration() {
        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var x = 42;");
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
        let mut lexer = Lexador::new();
        let code = r#"
            var a = 10;
            var b = 5;
            escreva("Sum: " + texto(a + b));
        "#;
        let tokens = lexer.analisar(code);
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 3);
    }

}
