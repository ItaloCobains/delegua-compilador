//! # Code Generation Module
//!
//! Generates LLVM Intermediate Representation (IR) from the DC language AST.
//! This module handles the translation of high-level language constructs
//! into low-level LLVM instructions that can be compiled to machine code.

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{IntValue, PointerValue, FunctionValue, BasicValueEnum};
use inkwell::types::{IntType, PointerType, BasicTypeEnum};
use std::collections::HashMap;

use crate::core::ast::{Program, Statement, Expr, BinaryOp};
use crate::core::error::CompilerError;

/// LLVM code generator for the DC language
pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    // Symbol table for variables
    variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,

    // Lazy declared built-in functions
    built_in_functions: HashMap<String, FunctionValue<'ctx>>,

    // Module functions
    modules: HashMap<String, HashMap<String, FunctionValue<'ctx>>>,

    // Common LLVM types
    i64_type: IntType<'ctx>,
    i8_ptr_type: PointerType<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    /// Creates a new code generator for the given LLVM context and module name
    pub fn new(context: &'ctx Context, module_name: &str) -> Result<Self, CompilerError> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        let i64_type = context.i64_type();
        let i8_ptr_type = context.ptr_type(inkwell::AddressSpace::default());

        let codegen = CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
            built_in_functions: HashMap::new(),
            modules: HashMap::new(),
            i64_type,
            i8_ptr_type,
        };

        Ok(codegen)
    }

    /// Gets or declares a built-in function
    fn get_built_in_function(&mut self, name: &str) -> Result<FunctionValue<'ctx>, CompilerError> {
        if let Some(&fn_val) = self.built_in_functions.get(name) {
            return Ok(fn_val);
        }

        let fn_val = match name {
            "printf" => {
                let printf_type = self.i64_type.fn_type(&[self.i8_ptr_type.into()], true);
                self.module.add_function("printf", printf_type, None)
            }
            "malloc" => {
                let malloc_type = self.i8_ptr_type.fn_type(&[self.i64_type.into()], false);
                self.module.add_function("malloc", malloc_type, None)
            }
            "strlen" => {
                let strlen_type = self.i64_type.fn_type(&[self.i8_ptr_type.into()], false);
                self.module.add_function("strlen", strlen_type, None)
            }
            "strcpy" => {
                let strcpy_type = self.i8_ptr_type.fn_type(&[self.i8_ptr_type.into(), self.i8_ptr_type.into()], false);
                self.module.add_function("strcpy", strcpy_type, None)
            }
            "strcat" => {
                let strcat_type = self.i8_ptr_type.fn_type(&[self.i8_ptr_type.into(), self.i8_ptr_type.into()], false);
                self.module.add_function("strcat", strcat_type, None)
            }
            "sprintf" => {
                let sprintf_type = self.i64_type.fn_type(&[
                    self.i8_ptr_type.into(),
                    self.i8_ptr_type.into()
                ], true);
                self.module.add_function("sprintf", sprintf_type, None)
            }
            _ => return Err(CompilerError::CodeGen(format!("Unknown built-in function: {}", name))),
        };

        self.built_in_functions.insert(name.to_string(), fn_val);
        Ok(fn_val)
    }

    /// Generates LLVM IR for a complete DC program
    pub fn generate(&mut self, program: &Program) -> Result<(), CompilerError> {
        // Create main function
        let main_type = self.i64_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);
        let basic_block = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(basic_block);

        // Generate code for each statement
        for statement in &program.statements {
            self.generate_statement(statement)?;
        }

        // Return 0 from main
        let zero = self.i64_type.const_int(0, false);
        self.builder.build_return(Some(&zero))
            .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;

        Ok(())
    }

    /// Generates code for a single statement
    fn generate_statement(&mut self, statement: &Statement) -> Result<(), CompilerError> {
        match statement {
            Statement::VarDeclaration { name, value } => {
                self.generate_variable_declaration(name, value)
            }
            Statement::Assignment { name, value } => {
                self.generate_assignment(name, value)
            }
            Statement::Import { module, items } => {
                self.generate_import(module, items.as_ref())
            }
            Statement::FunctionCall(expr) => {
                self.generate_expression(expr)?;
                Ok(())
            }
        }
    }

    /// Generates code for import statement
    fn generate_import(&mut self, _module: &str, _items: Option<&Vec<String>>) -> Result<(), CompilerError> {
        // TODO: Implement module loading and function declaration
        Ok(())
    }

    /// Generates code for variable declaration
    fn generate_variable_declaration(&mut self, name: &str, value: &Expr) -> Result<(), CompilerError> {
        let val = self.generate_expression(value)?;

        match val {
            BasicValueEnum::IntValue(int_val) => {
                let alloca = self.builder.build_alloca(self.i64_type, name)
                    .map_err(|e| CompilerError::CodeGen(format!("Error building alloca: {:?}", e)))?;
                self.builder.build_store(alloca, int_val)
                    .map_err(|e| CompilerError::CodeGen(format!("Error building store: {:?}", e)))?;
                self.variables.insert(name.to_string(), (alloca, self.i64_type.into()));
            }
            BasicValueEnum::PointerValue(ptr_val) => {
                let alloca = self.builder.build_alloca(self.i8_ptr_type, name)
                    .map_err(|e| CompilerError::CodeGen(format!("Error building alloca: {:?}", e)))?;
                self.builder.build_store(alloca, ptr_val)
                    .map_err(|e| CompilerError::CodeGen(format!("Error building store: {:?}", e)))?;
                self.variables.insert(name.to_string(), (alloca, self.i8_ptr_type.into()));
            }
            _ => return Err(CompilerError::CodeGen("Unsupported value type in variable declaration".to_string())),
        }
        Ok(())
    }

    /// Generates code for variable assignment
    fn generate_assignment(&mut self, name: &str, value: &Expr) -> Result<(), CompilerError> {
        let val = self.generate_expression(value)?;

        if let Some(&(var_ptr, _)) = self.variables.get(name) {
            match val {
                BasicValueEnum::IntValue(int_val) => {
                    self.builder.build_store(var_ptr, int_val)
                        .map_err(|e| CompilerError::CodeGen(format!("Error building store: {:?}", e)))?;
                }
                BasicValueEnum::PointerValue(ptr_val) => {
                    self.builder.build_store(var_ptr, ptr_val)
                        .map_err(|e| CompilerError::CodeGen(format!("Error building store: {:?}", e)))?;
                }
                _ => return Err(CompilerError::CodeGen("Unsupported value type in assignment".to_string())),
            }
        } else {
            return Err(CompilerError::CodeGen(format!("Variable '{}' not declared", name)));
        }
        Ok(())
    }

    /// Generates code for expressions
    fn generate_expression(&mut self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        match expr {
            Expr::Number(n) => {
                let val = self.i64_type.const_int(*n as u64, false);
                Ok(val.into())
            }

            Expr::String(s) => {
                let string_val = self.context.const_string(s.as_bytes(), true);
                let global = self.module.add_global(string_val.get_type(), None, "string_literal");
                global.set_initializer(&string_val);
                let ptr = global.as_pointer_value();
                Ok(ptr.into())
            }

            Expr::Identifier(name) => {
                if let Some(&(var_ptr, var_type)) = self.variables.get(name) {
                    let loaded = self.builder.build_load(var_type, var_ptr, name)
                        .map_err(|e| CompilerError::CodeGen(format!("Error loading variable: {:?}", e)))?;
                    Ok(loaded)
                } else {
                    Err(CompilerError::CodeGen(format!("Variable '{}' not found", name)))
                }
            }

            Expr::Binary { left, operator, right } => {
                let left_val = self.generate_expression(left)?;
                let right_val = self.generate_expression(right)?;

                match (left_val, right_val) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        let result = match operator {
                            BinaryOp::Add => self.builder.build_int_add(l, r, "add"),
                            BinaryOp::Subtract => self.builder.build_int_sub(l, r, "sub"),
                            BinaryOp::Multiply => self.builder.build_int_mul(l, r, "mul"),
                            BinaryOp::Divide => self.builder.build_int_signed_div(l, r, "div"),
                        }.map_err(|e| CompilerError::CodeGen(format!("Error building binary operation: {:?}", e)))?;
                        Ok(result.into())
                    }

                    (BasicValueEnum::PointerValue(l), BasicValueEnum::PointerValue(r)) => {
                        if matches!(operator, BinaryOp::Add) {
                            self.generate_string_concat(l, r)
                        } else {
                            Err(CompilerError::CodeGen("Only concatenation (+) is supported for strings".to_string()))
                        }
                    }

                    (BasicValueEnum::PointerValue(l), BasicValueEnum::IntValue(r)) => {
                        if matches!(operator, BinaryOp::Add) {
                            let r_str = self.int_to_string(r)?;
                            self.generate_string_concat(l, r_str)
                        } else {
                            Err(CompilerError::CodeGen("Only concatenation (+) is supported between string and number".to_string()))
                        }
                    }

                    _ => Err(CompilerError::CodeGen("Unsupported operand types for binary operation".to_string())),
                }
            }

            Expr::FunctionCall { name, args } => {
                match name.as_str() {
                    "escreva" => self.generate_escreva_call(args),
                    "texto" => self.generate_texto_call(args),
                    _ => Err(CompilerError::CodeGen(format!("Unknown function: {}", name))),
                }
            }
        }
    }

    /// Generates code for escreva() function calls
    fn generate_escreva_call(&mut self, args: &[Expr]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen("escreva() expects exactly one argument".to_string()));
        }

        let arg = self.generate_expression(&args[0])?;

        match arg {
            BasicValueEnum::PointerValue(str_ptr) => {
                let printf_fn = self.get_built_in_function("printf")?;
                let format_str = self.context.const_string(b"%s\n\0", false);
                let format_global = self.module.add_global(format_str.get_type(), None, "format_str");
                format_global.set_initializer(&format_str);
                let format_ptr = format_global.as_pointer_value();

                self.builder.build_call(
                    printf_fn,
                    &[format_ptr.into(), str_ptr.into()],
                    "printf_call"
                ).map_err(|e| CompilerError::CodeGen(format!("Error calling printf: {:?}", e)))?;
            }
            BasicValueEnum::IntValue(int_val) => {
                let printf_fn = self.get_built_in_function("printf")?;
                let format_str = self.context.const_string(b"%lld\n\0", false);
                let format_global = self.module.add_global(format_str.get_type(), None, "format_int");
                format_global.set_initializer(&format_str);
                let format_ptr = format_global.as_pointer_value();

                self.builder.build_call(
                    printf_fn,
                    &[format_ptr.into(), int_val.into()],
                    "printf_call"
                ).map_err(|e| CompilerError::CodeGen(format!("Error calling printf: {:?}", e)))?;
            }
            _ => return Err(CompilerError::CodeGen("Unsupported argument type for escreva()".to_string())),
        }

        Ok(self.i64_type.const_int(0, false).into())
    }

    /// Generates code for texto() function calls
    fn generate_texto_call(&mut self, args: &[Expr]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen("texto() expects exactly one argument".to_string()));
        }

        let arg = self.generate_expression(&args[0])?;

        match arg {
            BasicValueEnum::IntValue(int_val) => {
                self.int_to_string(int_val).map(|ptr| ptr.into())
            }
            BasicValueEnum::PointerValue(ptr_val) => {
                Ok(ptr_val.into())
            }
            _ => Err(CompilerError::CodeGen("Unsupported argument type for texto()".to_string())),
        }
    }

    /// Converts an integer to its string representation
    fn int_to_string(&mut self, int_val: IntValue<'ctx>) -> Result<PointerValue<'ctx>, CompilerError> {
        let malloc_fn = self.get_built_in_function("malloc")?;
        let buffer_size = self.i64_type.const_int(20, false);
        let buffer_call = self.builder.build_call(malloc_fn, &[buffer_size.into()], "int_str_buffer")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling malloc: {:?}", e)))?;

        let buffer = buffer_call.try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        let sprintf_fn = self.get_built_in_function("sprintf")?;
        let format_str = self.context.const_string(b"%lld\0", false);
        let format_global = self.module.add_global(format_str.get_type(), None, "int_format");
        format_global.set_initializer(&format_str);
        let format_ptr = format_global.as_pointer_value();

        self.builder.build_call(
            sprintf_fn,
            &[buffer.into(), format_ptr.into(), int_val.into()],
            "sprintf_call"
        ).map_err(|e| CompilerError::CodeGen(format!("Error calling sprintf: {:?}", e)))?;

        Ok(buffer)
    }

    /// Concatenates two strings
    fn generate_string_concat(&mut self, left: PointerValue<'ctx>, right: PointerValue<'ctx>) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let strlen_fn = self.get_built_in_function("strlen")?;
        // Calculate total length
        let left_len_call = self.builder.build_call(strlen_fn, &[left.into()], "left_len")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling strlen: {:?}", e)))?;
        let left_len = left_len_call.try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value();

        let right_len_call = self.builder.build_call(strlen_fn, &[right.into()], "right_len")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling strlen: {:?}", e)))?;
        let right_len = right_len_call.try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value();

        let total_len = self.builder.build_int_add(left_len, right_len, "total_len")
            .map_err(|e| CompilerError::CodeGen(format!("Error building add: {:?}", e)))?;
        let total_len_plus_one = self.builder.build_int_add(
            total_len,
            self.i64_type.const_int(1, false),
            "total_len_plus_one"
        ).map_err(|e| CompilerError::CodeGen(format!("Error building add: {:?}", e)))?;

        // Allocate result buffer
        let malloc_fn = self.get_built_in_function("malloc")?;
        let result_buffer_call = self.builder.build_call(malloc_fn, &[total_len_plus_one.into()], "concat_buffer")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling malloc: {:?}", e)))?;
        let result_buffer = result_buffer_call.try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // Copy strings
        let strcpy_fn = self.get_built_in_function("strcpy")?;
        self.builder.build_call(strcpy_fn, &[result_buffer.into(), left.into()], "strcpy_first")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling strcpy: {:?}", e)))?;

        let strcat_fn = self.get_built_in_function("strcat")?;
        self.builder.build_call(strcat_fn, &[result_buffer.into(), right.into()], "strcat_second")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling strcat: {:?}", e)))?;

        Ok(result_buffer.into())
    }

    /// Checks if a variable exists in the symbol table
    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Prints the generated LLVM IR to stdout
    pub fn print_ir(&self) {
        println!("{}", self.module.print_to_string().to_string());
    }

    /// Returns the generated LLVM IR as a string
    pub fn get_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

    /// Generates IR for a single expression (for testing purposes)
    /// Creates a temporary main function and generates the expression
    pub fn generate_expression_only(&mut self, expr: &Expr) -> Result<String, CompilerError> {
        // Create a temporary main function
        let fn_type = self.i64_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", fn_type, None);
        let entry_bb = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry_bb);

        // Generate the expression
        let result = self.generate_expression(expr)?;

        // Return the result
        match result {
            BasicValueEnum::IntValue(val) => {
                let _ = self.builder.build_return(Some(&val));
            }
            _ => return Err(CompilerError::CodeGen("Expression must evaluate to integer".to_string())),
        };

        Ok(self.get_ir())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::lexer::Lexer;
    use crate::core::parser::Parser;

    #[test]
    fn test_simple_variable() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var a = 42;");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("alloca"));
        assert!(ir.contains("store"));
    }

    #[test]
    fn test_arithmetic() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var a = 10; var b = 5; var c = 2; var result = a + b * c;");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("mul"));
        assert!(ir.contains("add"));
    }

    #[test]
    fn test_string_literal() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize("var msg = \"Hello World\";");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("Hello World"));
    }

    #[test]
    fn test_complete_program() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let lexer = Lexer::new();
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
