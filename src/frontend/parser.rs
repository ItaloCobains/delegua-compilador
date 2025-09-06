use crate::frontend::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(i64),
    String(String),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
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
    FunctionCall(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.current_token()
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if std::mem::discriminant(self.current_token()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.current_token()))
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while *self.current_token() != Token::EOF {
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    pub fn parse_expression_only(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expression()?;
        if *self.current_token() != Token::EOF {
            return Err("Unexpected tokens after expression".to_string());
        }
        Ok(expr)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token() {
            Token::Var => self.parse_var_declaration(),
            Token::Escreva => {
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Statement::FunctionCall(expr))
            }
            Token::Ident(_) => {
                let name = if let Token::Ident(name) = self.current_token() {
                    name.clone()
                } else {
                    return Err("Expected identifier".to_string());
                };

                self.advance();

                if *self.current_token() == Token::Assign {
                    self.advance();
                    let value = self.parse_expression()?;
                    self.expect(Token::Semicolon)?;
                    Ok(Statement::Assignment { name, value })
                } else {
                    return Err("Expected assignment or function call".to_string());
                }
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token())),
        }
    }

    fn parse_var_declaration(&mut self) -> Result<Statement, String> {
        self.expect(Token::Var)?;

        let name = if let Token::Ident(name) = self.current_token() {
            name.clone()
        } else {
            return Err("Expected identifier after 'var'".to_string());
        };

        self.advance();
        self.expect(Token::Assign)?;

        let value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(Statement::VarDeclaration { name, value })
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let operator = match self.current_token() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
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

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while matches!(self.current_token(), Token::Multiply | Token::Divide) {
            let operator = match self.current_token() {
                Token::Multiply => BinaryOp::Multiply,
                Token::Divide => BinaryOp::Divide,
                _ => unreachable!(),
            };

            self.advance();
            let right = self.parse_primary()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current_token().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expr::String(s))
            }
            Token::Ident(name) => {
                self.advance();
                Ok(Expr::Identifier(name))
            }
            Token::Escreva => {
                self.advance();
                self.expect(Token::LeftParen)?;
                let mut args = Vec::new();

                if *self.current_token() != Token::RightParen {
                    args.push(self.parse_expression()?);
                }

                self.expect(Token::RightParen)?;

                Ok(Expr::FunctionCall {
                    name: "escreva".to_string(),
                    args,
                })
            }
            Token::Texto => {
                self.advance();
                self.expect(Token::LeftParen)?;
                let mut args = Vec::new();

                if *self.current_token() != Token::RightParen {
                    args.push(self.parse_expression()?);
                }

                self.expect(Token::RightParen)?;

                Ok(Expr::FunctionCall {
                    name: "texto".to_string(),
                    args,
                })
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.current_token())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let tokens = vec![Token::Number(42), Token::EOF];
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expr::Number(42));
    }

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::String("hello".to_string()), Token::EOF];
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expr::String("hello".to_string()));
    }

    #[test]
    fn test_parse_identifier() {
        let tokens = vec![Token::Ident("x".to_string()), Token::EOF];
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expr::Identifier("x".to_string()));
    }

    #[test]
    fn test_parse_binary_addition() {
        let tokens = vec![
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::Number(1)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Number(2)),
        });
    }

    #[test]
    fn test_parse_binary_multiplication() {
        let tokens = vec![
            Token::Number(3),
            Token::Multiply,
            Token::Number(4),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::Number(3)),
            operator: BinaryOp::Multiply,
            right: Box::new(Expr::Number(4)),
        });
    }

    #[test]
    fn test_operator_precedence() {
        let tokens = vec![
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            Token::Multiply,
            Token::Number(3),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::Number(1)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Number(2)),
                operator: BinaryOp::Multiply,
                right: Box::new(Expr::Number(3)),
            }),
        });
    }

    #[test]
    fn test_parse_var_declaration() {
        let tokens = vec![
            Token::Var,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Number(10),
            Token::Semicolon,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements, vec![
            Statement::VarDeclaration {
                name: "a".to_string(),
                value: Expr::Number(10),
            }
        ]);
    }

    #[test]
    fn test_parse_assignment() {
        let tokens = vec![
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Number(42),
            Token::Semicolon,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements, vec![
            Statement::Assignment {
                name: "x".to_string(),
                value: Expr::Number(42),
            }
        ]);
    }

    #[test]
    fn test_parse_function_call_escreva() {
        let tokens = vec![
            Token::Escreva,
            Token::LeftParen,
            Token::String("Hello".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements, vec![
            Statement::FunctionCall(Expr::FunctionCall {
                name: "escreva".to_string(),
                args: vec![Expr::String("Hello".to_string())],
            })
        ]);
    }

    #[test]
    fn test_parse_texto_function() {
        let tokens = vec![
            Token::Texto,
            Token::LeftParen,
            Token::Ident("a".to_string()),
            Token::RightParen,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expr::FunctionCall {
            name: "texto".to_string(),
            args: vec![Expr::Identifier("a".to_string())],
        });
    }

    #[test]
    fn test_complex_expression() {
        let tokens = vec![
            Token::String("Valor: ".to_string()),
            Token::Plus,
            Token::Texto,
            Token::LeftParen,
            Token::Ident("a".to_string()),
            Token::RightParen,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::String("Valor: ".to_string())),
            operator: BinaryOp::Add,
            right: Box::new(Expr::FunctionCall {
                name: "texto".to_string(),
                args: vec![Expr::Identifier("a".to_string())],
            }),
        });
    }

    #[test]
    fn test_multiple_statements() {
        let tokens = vec![
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
        ];
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements, vec![
            Statement::VarDeclaration {
                name: "a".to_string(),
                value: Expr::Number(10),
            },
            Statement::VarDeclaration {
                name: "b".to_string(),
                value: Expr::Number(20),
            }
        ]);
    }

    #[test]
    fn test_parentheses() {
        let tokens = vec![
            Token::LeftParen,
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            Token::RightParen,
            Token::Multiply,
            Token::Number(3),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Number(1)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Number(2)),
            }),
            operator: BinaryOp::Multiply,
            right: Box::new(Expr::Number(3)),
        });
    }

    #[test]
    fn test_integration_with_lexer() {
        let lexer = crate::frontend::lexer::Lexer::new();
        let tokens = lexer.tokenize("var soma = a + b * 2;");
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements, vec![
            Statement::VarDeclaration {
                name: "soma".to_string(),
                value: Expr::Binary {
                    left: Box::new(Expr::Identifier("a".to_string())),
                    operator: BinaryOp::Add,
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::Identifier("b".to_string())),
                        operator: BinaryOp::Multiply,
                        right: Box::new(Expr::Number(2)),
                    }),
                },
            }
        ]);
    }

    #[test]
    fn test_complete_program_parsing() {
        let lexer = crate::frontend::lexer::Lexer::new();
        let code = r#"
            var a = 10;
            var soma = a + 5;
            escreva("Resultado: " + texto(soma));
        "#;
        let tokens = lexer.tokenize(code);
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 3);

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "a");
                assert_eq!(*value, Expr::Number(10));
            }
            _ => panic!("Expected var declaration"),
        }

        match &program.statements[2] {
            Statement::FunctionCall(Expr::FunctionCall { name, args }) => {
                assert_eq!(name, "escreva");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected function call"),
        }
    }
}