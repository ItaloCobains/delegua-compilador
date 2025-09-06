//! # Integration Tests
//!
//! End-to-end integration tests that verify the complete functionality
//! of the DC language compiler, from source code to executable output.

#[cfg(test)]
mod integration_tests {
    use inkwell::context::Context;
    use dc::core::lexer::Lexer;
    use dc::core::parser::Parser;
    use dc::core::codegen::CodeGen;

    #[test]
    fn test_complete_program_execution() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "complete_test").unwrap();

        let program = r#"
            var x = 10;
            var y = 5;
            var sum = x + y;
            var product = x * y;
            escreva("Sum: " + texto(sum));
            escreva("Product: " + texto(product));
        "#;

        // Full pipeline test
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        codegen.generate(&ast).unwrap();
        let ir = codegen.get_ir();

        // Verify IR contains expected elements
        assert!(ir.contains("main"));
        assert!(ir.contains("printf"));
        assert!(ir.contains("add"));
        assert!(ir.contains("mul"));
        assert!(ir.contains("Sum:"));
        assert!(ir.contains("Product:"));
    }

    #[test]
    fn test_function_calls_and_expressions() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "function_test").unwrap();

        let program = r#"
            var a = 42;
            escreva("Value: " + texto(a));
            var b = a + 8;
            escreva("New value: " + texto(b));
        "#;

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        codegen.generate(&ast).unwrap();
        let ir = codegen.get_ir();

        assert!(ir.contains("Value:"));
        assert!(ir.contains("New value:"));
        assert!(ir.contains("42"));
    }

    #[test]
    fn test_complex_expressions() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "complex_expr_test").unwrap();

        let program = r#"
            var a = 10;
            var b = 5;
            var c = 2;
            var d = 3;
            var result = (a + b) * c - d;
            escreva("Complex result: " + texto(result));
        "#;

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        codegen.generate(&ast).unwrap();
        let ir = codegen.get_ir();

        assert!(ir.contains("Complex result:"));
        assert!(ir.contains("mul"));
        assert!(ir.contains("sub"));
    }

    #[test]
    fn test_string_operations() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "string_test").unwrap();

        let program = r#"
            var greeting = "Hello";
            var name = "World";
            var message = greeting + " " + name + "!";
            escreva(message);
        "#;

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        codegen.generate(&ast).unwrap();
        let ir = codegen.get_ir();

        assert!(ir.contains("Hello"));
        assert!(ir.contains("World"));
        assert!(ir.contains("!"));
    }

    #[test]
    fn test_variable_scoping_and_reuse() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "scope_test").unwrap();

        let program = r#"
            var x = 1;
            var y = x + 1;
            x = y + 1;
            escreva("Final x: " + texto(x));
        "#;

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        codegen.generate(&ast).unwrap();
        let ir = codegen.get_ir();

        assert!(ir.contains("Final x:"));
        assert!(codegen.has_variable("x"));
    }

    #[test]
    fn test_error_handling() {
        let mut lexer = Lexer::new();

        // Test invalid syntax
        let invalid_code = "var x = ;";
        let tokens = lexer.tokenize(invalid_code);
        let mut parser = Parser::new(tokens);

        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_empty_program() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "empty_test").unwrap();

        let program = "";
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        codegen.generate(&ast).unwrap();
        let ir = codegen.get_ir();

        // Should still generate a valid main function
        assert!(ir.contains("main"));
        assert!(ir.contains("ret"));
    }

    #[test]
    fn test_large_numbers() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "large_num_test").unwrap();

        let program = r#"
            var big_num = 999999;
            escreva("Large number: " + texto(big_num));
        "#;

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        codegen.generate(&ast).unwrap();
        let ir = codegen.get_ir();

        assert!(ir.contains("999999"));
        assert!(ir.contains("Large number:"));
    }

    #[test]
    fn test_multiple_function_calls() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "multi_call_test").unwrap();

        let program = r#"
            escreva("First message");
            escreva("Second message");
            escreva("Third message");
        "#;

        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        codegen.generate(&ast).unwrap();
        let ir = codegen.get_ir();

        // Should contain multiple printf calls
        let printf_count = ir.matches("printf").count();
        assert!(printf_count >= 3);
    }
}
