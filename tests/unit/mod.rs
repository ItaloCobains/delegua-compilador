//! # Unit Tests
//!
//! Comprehensive unit tests for all DC language components.
//! Tests cover lexer, parser, code generator, and core functionality.

#[cfg(test)]
mod lexer_tests {
    use dc::core::lexer::Lexer;
    use dc::core::token::Token;

    #[test]
    fn test_tokenize_variable_declaration() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var x = 42;");
        // Just check that we have the right number of tokens and types
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0], Token::Var(_)));
        assert!(matches!(tokens[1], Token::Ident(_, _)));
        assert!(matches!(tokens[2], Token::Assign(_)));
        assert!(matches!(tokens[3], Token::Number(_, _)));
        assert!(matches!(tokens[4], Token::Semicolon(_)));
        assert!(matches!(tokens[5], Token::EOF(_)));
    }

    #[test]
    fn test_tokenize_string_literal() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("\"Hello World\"");
        // Just check that we have the right number of tokens and types
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0], Token::String(_, _)));
        assert!(matches!(tokens[1], Token::EOF(_)));
    }

    #[test]
    fn test_tokenize_arithmetic_expression() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("a + b * c");
        // Just check that we have the right number of tokens and types
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0], Token::Ident(_, _)));
        assert!(matches!(tokens[1], Token::Plus(_)));
        assert!(matches!(tokens[2], Token::Ident(_, _)));
        assert!(matches!(tokens[3], Token::Multiply(_)));
        assert!(matches!(tokens[4], Token::Ident(_, _)));
        assert!(matches!(tokens[5], Token::EOF(_)));
    }

    #[test]
    fn test_tokenize_function_call() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("escreva(\"test\")");
        // Just check that we have the right number of tokens and types
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Token::Escreva(_)));
        assert!(matches!(tokens[1], Token::LeftParen(_)));
        assert!(matches!(tokens[2], Token::String(_, _)));
        assert!(matches!(tokens[3], Token::RightParen(_)));
        assert!(matches!(tokens[4], Token::EOF(_)));
    }

    #[test]
    fn test_tokenize_with_comments() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var x = 1; // comment\nvar y = 2;");
        // Just check that we have the right number of tokens and types
        assert_eq!(tokens.len(), 11);
        assert!(matches!(tokens[0], Token::Var(_)));
        assert!(matches!(tokens[1], Token::Ident(_, _)));
        assert!(matches!(tokens[2], Token::Assign(_)));
        assert!(matches!(tokens[3], Token::Number(_, _)));
        assert!(matches!(tokens[4], Token::Semicolon(_)));
        assert!(matches!(tokens[5], Token::Var(_)));
        assert!(matches!(tokens[6], Token::Ident(_, _)));
        assert!(matches!(tokens[7], Token::Assign(_)));
        assert!(matches!(tokens[8], Token::Number(_, _)));
        assert!(matches!(tokens[9], Token::Semicolon(_)));
        assert!(matches!(tokens[10], Token::EOF(_)));
    }
}

#[cfg(test)]
mod parser_tests {
    use dc::core::lexer::Lexer;
    use dc::core::parser::Parser;
    use dc::core::ast::{Expr, BinaryOp, Statement};

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
    fn test_parse_arithmetic_expression() {
        let mut lexer = Lexer::new();
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
        let mut lexer = Lexer::new();
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

#[cfg(test)]
mod codegen_tests {
    use inkwell::context::Context;
    use dc::core::lexer::Lexer;
    use dc::core::parser::Parser;
    use dc::core::codegen::CodeGen;

    #[test]
    fn test_codegen_simple_variable() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var a = 42;");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("alloca"));
        assert!(ir.contains("store"));
    }

    #[test]
    fn test_codegen_arithmetic() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var a = 10; var b = 5; var c = 2; var result = a + b * c;");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("mul"));
        assert!(ir.contains("add"));
    }

    #[test]
    fn test_codegen_string_literal() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var msg = \"Hello World\";");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("Hello World"));
    }

    #[test]
    fn test_codegen_complete_program() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexer::new();
        let code = r#"
            var a = 10;
            var b = 5;
            var soma = a + b;
            escreva("Resultado: " + texto(soma));
        "#;
        let tokens = lexer.tokenize(code);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("main"));
        assert!(ir.contains("printf"));
        assert!(ir.contains("add"));
        assert!(ir.contains("Resultado:"));
    }
}

#[cfg(test)]
mod integration_tests {
    use inkwell::context::Context;
    use dc::core::lexer::Lexer;
    use dc::core::parser::Parser;
    use dc::core::codegen::CodeGen;

    #[test]
    fn test_full_compilation_pipeline() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "integration_test").unwrap();

        let source_code = r#"
            var a = 10;
            var b = 20;
            var result = a + b;
            escreva("Sum: " + texto(result));
        "#;

        // Lexical analysis
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(source_code);
        assert!(!tokens.is_empty());

        // Parsing
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast.statements.len(), 4);

        // Code generation
        codegen.generate(&ast).unwrap();
        let ir = codegen.get_ir();
        assert!(ir.contains("main"));
        assert!(ir.contains("printf"));
    }

}

#[cfg(test)]
mod logical_operator_tests {
    use dc::core::lexer::Lexer;
    use dc::core::parser::Parser;
    use dc::core::ast::{Expr, BinaryOp};
    use inkwell::context::Context;
    use dc::core::codegen::CodeGen;

    #[test]
    fn test_tokenize_logical_operators() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("verdadeiro e falso ou não verdadeiro");
        assert_eq!(tokens.len(), 6);
    }

    #[test]
    fn test_parse_logical_and() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("verdadeiro e falso");
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression_only().unwrap();
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::Bool(true)),
            operator: BinaryOp::And,
            right: Box::new(Expr::Bool(false)),
        });
    }

    #[test]
    fn test_parse_logical_or() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("verdadeiro ou falso");
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression_only().unwrap();
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::Bool(true)),
            operator: BinaryOp::Or,
            right: Box::new(Expr::Bool(false)),
        });
    }

    #[test]
    fn test_parse_logical_not() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("não verdadeiro");
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression_only().unwrap();
        assert_eq!(expr, Expr::Unary {
            operator: BinaryOp::Not,
            operand: Box::new(Expr::Bool(true)),
        });
    }

    #[test]
    fn test_logical_operator_precedence() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("verdadeiro ou falso e verdadeiro");
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression_only().unwrap();
        assert_eq!(expr, Expr::Binary {
            left: Box::new(Expr::Bool(true)),
            operator: BinaryOp::Or,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Bool(false)),
                operator: BinaryOp::And,
                right: Box::new(Expr::Bool(true)),
            }),
        });
    }

    #[test]
    fn test_codegen_logical_and() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test_logical").unwrap();

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var resultado = 1 e 0;");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());
        let ir = codegen.get_ir();
        assert!(ir.contains("and"));
    }

    #[test]
    fn test_codegen_logical_or() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test_logical").unwrap();

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var resultado = 1 ou 0;");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());
        let ir = codegen.get_ir();
        assert!(ir.contains("or"));
    }

    #[test]
    fn test_codegen_logical_not() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test_logical").unwrap();

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var resultado = não 1;");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());
        let ir = codegen.get_ir();
        assert!(ir.contains("not"));
    }
}

#[cfg(test)]
mod leia_function_tests {
    use dc::core::lexer::Lexer;
    use dc::core::parser::Parser;
    use dc::core::ast::{Expr, Statement};
    use inkwell::context::Context;
    use dc::core::codegen::CodeGen;

    #[test]
    fn test_tokenize_leia() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("leia()");
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn test_parse_leia_no_args() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("leia()");
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression_only().unwrap();
        assert_eq!(expr, Expr::FunctionCall {
            callee: Box::new(Expr::Identifier("leia".to_string())),
            args: vec![],
        });
    }

    #[test]
    fn test_parse_leia_with_prompt() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("leia(\"Digite seu nome: \")");
        let mut parser = Parser::new(tokens);

        let expr = parser.parse_expression_only().unwrap();
        assert_eq!(expr, Expr::FunctionCall {
            callee: Box::new(Expr::Identifier("leia".to_string())),
            args: vec![Expr::String("Digite seu nome: ".to_string())],
        });
    }

    #[test]
    fn test_parse_leia_assignment() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var nome = leia(\"Nome: \");");
        let mut parser = Parser::new(tokens);

        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::VarDeclaration { name, value } => {
                assert_eq!(name, "nome");
                assert_eq!(*value, Expr::FunctionCall {
                    callee: Box::new(Expr::Identifier("leia".to_string())),
                    args: vec![Expr::String("Nome: ".to_string())],
                });
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_codegen_leia() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test_leia").unwrap();

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var entrada = leia();");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());
        let ir = codegen.get_ir();
        assert!(ir.contains("scanf"));
        assert!(ir.contains("malloc"));
    }

    #[test]
    fn test_codegen_leia_with_prompt() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test_leia").unwrap();

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var entrada = leia(\"Digite: \");");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());
        let ir = codegen.get_ir();
        assert!(ir.contains("scanf"));
        assert!(ir.contains("printf"));
        assert!(ir.contains("Digite:"));
    }
}
