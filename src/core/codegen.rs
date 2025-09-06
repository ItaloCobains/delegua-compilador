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

/// Represents the type of a variable
#[derive(Clone, Copy)]
enum VariableType<'ctx> {
    Int(IntType<'ctx>),
    String(PointerType<'ctx>),
    Function(PointerType<'ctx>),
}

impl<'ctx> VariableType<'ctx> {
    fn as_basic_type_enum(self) -> BasicTypeEnum<'ctx> {
        match self {
            VariableType::Int(t) => t.into(),
            VariableType::String(t) => t.into(),
            VariableType::Function(t) => t.into(),
        }
    }
}
use std::collections::HashMap;

use crate::core::ast::{Program, Statement, Expr, BinaryOp};
use crate::core::error::CompilerError;
use crate::modules::matematica::Matematica;

/// High-performance LLVM code generator for the DC language
pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    // Symbol table for variables - pre-allocated for performance
    variables: HashMap<String, (PointerValue<'ctx>, VariableType<'ctx>)>,

    // User-defined functions
    functions: HashMap<String, FunctionValue<'ctx>>,

    // Lazy declared built-in functions - cached for reuse
    built_in_functions: HashMap<String, FunctionValue<'ctx>>,

    // Module functions
    modules: HashMap<String, HashMap<String, FunctionValue<'ctx>>>,

    // Common LLVM types (cached for performance)
    i64_type: IntType<'ctx>,
    i8_ptr_type: PointerType<'ctx>,
    
    // Pre-allocated format strings cache to avoid recreation
    format_strings: HashMap<String, PointerValue<'ctx>>,
    
    // Loop context stack for break/continue
    loop_stack: Vec<LoopContext<'ctx>>,
}

/// Context for nested loops to handle break/continue
#[derive(Debug, Clone, Copy)]
struct LoopContext<'ctx> {
    break_block: inkwell::basic_block::BasicBlock<'ctx>,
    continue_block: inkwell::basic_block::BasicBlock<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    /// Creates a new high-performance code generator
    pub fn new(context: &'ctx Context, module_name: &str) -> Result<Self, CompilerError> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        let i64_type = context.i64_type();
        let i8_ptr_type = context.ptr_type(inkwell::AddressSpace::default());

        let codegen = CodeGen {
            context,
            module,
            builder,
            // Pre-allocate hash maps for better performance
            variables: HashMap::with_capacity(32),
            functions: HashMap::with_capacity(16),
            built_in_functions: HashMap::with_capacity(8),
            modules: HashMap::with_capacity(4),
            format_strings: HashMap::with_capacity(8),
            loop_stack: Vec::with_capacity(8),
            i64_type,
            i8_ptr_type,
        };

        Ok(codegen)
    }

    /// Gets or declares a built-in function - cached for performance
    #[inline]
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
            _ => return Err(CompilerError::CodeGen(
                format!("Unknown built-in function '{}'. Available: printf, malloc, strlen, strcpy, strcat, sprintf", name)
            )),
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
            Statement::If { condition, then_branch, else_branch } => {
                self.generate_if_statement(condition, then_branch, else_branch.as_ref())
            }
            Statement::IfElseIf { condition, then_branch, else_if_branches, else_branch } => {
                self.generate_if_else_if_statement(condition, then_branch, else_if_branches, else_branch.as_ref())
            }
            Statement::Switch { value, cases, default } => {
                self.generate_switch_statement(value, cases, default.as_ref())
            }
            Statement::While { condition, body } => {
                self.generate_while_statement(condition, body)
            }
            Statement::DoWhile { body, condition } => {
                self.generate_do_while_statement(body, condition)
            }
            Statement::For { initializer, condition, increment, body } => {
                self.generate_for_statement(initializer.as_ref().map(|v| &**v), condition.as_ref(), increment.as_ref(), body)
            }
            Statement::ForEach { variable, iterable, body } => {
                self.generate_for_each_statement(variable, iterable, body)
            }
            Statement::Break => {
                self.generate_break_statement()
            }
            Statement::Continue => {
                self.generate_continue_statement()
            }
            Statement::FunctionCall(expr) => {
                self.generate_expression(expr)?;
                Ok(())
            }
            Statement::FunctionDeclaration { name, params, body } => {
                self.generate_function_declaration(name.as_ref(), params, body)
            }
            Statement::Return(value) => {
                self.generate_return_statement(value.as_ref())
            }
        }
    }

    /// Generates code for import statement
    fn generate_import(&mut self, module: &str, _items: Option<&Vec<String>>) -> Result<(), CompilerError> {
        match module {
            "matematica" => {
                let math_module = Matematica::new(self.context);
                let functions = math_module.declarar_funcoes(&self.module);
                math_module.gerar_implementacoes(&self.module);

                // Store the functions in the modules map
                let mut module_functions = HashMap::new();
                for (name, func) in functions {
                    module_functions.insert(name, func);
                }
                self.modules.insert("matematica".to_string(), module_functions);
            }
            _ => return Err(CompilerError::CodeGen(format!("Unknown module: {}", module))),
        }
        Ok(())
    }

    /// Generates code for variable declaration - optimized with better error handling
    fn generate_variable_declaration(&mut self, name: &str, value: &Expr) -> Result<(), CompilerError> {
        let val = self.generate_expression(value)?;

        let (alloca, var_type) = match val {
            BasicValueEnum::IntValue(int_val) => {
                let alloca = self.safe_build(
                    self.builder.build_alloca(self.i64_type, name),
                    &format!("alloca for variable '{}'", name)
                )?;
                self.safe_build(
                    self.builder.build_store(alloca, int_val),
                    &format!("store to variable '{}'", name)
                )?;
                (alloca, VariableType::Int(self.i64_type))
            }
            BasicValueEnum::PointerValue(ptr_val) => {
                match value {
                    Expr::Function { .. } => {
                        // Function pointer
                        let func_ptr_type = ptr_val.get_type();
                        let alloca = self.safe_build(
                            self.builder.build_alloca(func_ptr_type, name),
                            &format!("alloca for function '{}'", name)
                        )?;
                        self.safe_build(
                            self.builder.build_store(alloca, ptr_val),
                            &format!("store function to '{}'", name)
                        )?;
                        (alloca, VariableType::Function(func_ptr_type))
                    }
                    _ => {
                        // String pointer
                        let alloca = self.safe_build(
                            self.builder.build_alloca(self.i8_ptr_type, name),
                            &format!("alloca for string '{}'", name)
                        )?;
                        self.safe_build(
                            self.builder.build_store(alloca, ptr_val),
                            &format!("store string to '{}'", name)
                        )?;
                        (alloca, VariableType::String(self.i8_ptr_type))
                    }
                }
            }
            _ => return Err(CompilerError::CodeGen(
                format!("Unsupported value type for variable '{}': {:?}", name, val.get_type())
            )),
        };
        
        self.variables.insert(name.to_string(), (alloca, var_type));
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

            Expr::Bool(b) => {
                let val = self.i64_type.const_int(if *b { 1 } else { 0 }, false);
                Ok(val.into())
            }

            Expr::Identifier(name) => {
                if let Some(&(var_ptr, var_type)) = self.variables.get(name) {
                    let loaded = self.builder.build_load(var_type.as_basic_type_enum(), var_ptr, name)
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
                        let (result, _op_name) = match operator {
                            BinaryOp::Add => (self.safe_build(self.builder.build_int_add(l, r, "add"), "integer addition")?, "add"),
                            BinaryOp::Subtract => (self.safe_build(self.builder.build_int_sub(l, r, "sub"), "integer subtraction")?, "sub"),
                            BinaryOp::Multiply => (self.safe_build(self.builder.build_int_mul(l, r, "mul"), "integer multiplication")?, "mul"),
                            BinaryOp::Divide => (self.safe_build(self.builder.build_int_signed_div(l, r, "div"), "integer division")?, "div"),
                            BinaryOp::Equal => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::EQ, l, r, "eq"), "equality comparison")?, "eq"),
                            BinaryOp::NotEqual => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::NE, l, r, "ne"), "inequality comparison")?, "ne"),
                            BinaryOp::Less => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::SLT, l, r, "lt"), "less than comparison")?, "lt"),
                            BinaryOp::Greater => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::SGT, l, r, "gt"), "greater than comparison")?, "gt"),
                            BinaryOp::LessEqual => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::SLE, l, r, "le"), "less equal comparison")?, "le"),
                            BinaryOp::GreaterEqual => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::SGE, l, r, "ge"), "greater equal comparison")?, "ge"),
                        };
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

            Expr::Unary { operator, operand } => {
                let operand_val = self.generate_expression(operand)?;
                match operand_val {
                    BasicValueEnum::IntValue(val) => {
                        let result = match operator {
                            BinaryOp::Subtract => self.builder.build_int_neg(val, "neg")
                                .map_err(|e| CompilerError::CodeGen(format!("Error building unary operation: {:?}", e)))?,
                            BinaryOp::Add => val, // +x is just x
                            _ => return Err(CompilerError::CodeGen("Unsupported unary operator".to_string())),
                        };
                        Ok(result.into())
                    }
                    _ => Err(CompilerError::CodeGen("Unary operations only supported for integers".to_string())),
                }
            }

            Expr::FunctionCall { callee, args } => {
                match callee.as_ref() {
                    Expr::Identifier(name) => {
                        match name.as_str() {
                            "escreva" => self.generate_escreva_call(args),
                            "texto" => self.generate_texto_call(args),
                            _ => {
                                // Check if it's a user-defined function
                                if let Some(func) = self.functions.get(name) {
                                    let func = *func; // Copy the function value
                                    self.generate_user_function_call(&func, args)
                                } else if name.contains('.') {
                                    self.generate_module_function_call(name, args)
                                } else if self.variables.contains_key(name) {
                                    // It's a variable that contains a function
                                    if let Some(&(var_ptr, var_type)) = self.variables.get(name) {
                                        match var_type {
                                            VariableType::Function(func_ptr_type) => {
                                                // For indirect calls, we need the function type, not the pointer type
                                                // Assume all functions have the same signature: i64 return, i64 params
                                                let param_types = vec![self.i64_type.into(); args.len()];
                                                let fn_type = self.i64_type.fn_type(&param_types, false);

                                                // Load the function pointer from the variable
                                                let func_ptr = self.builder.build_load(func_ptr_type, var_ptr, &format!("load_func_{}", name))
                                                    .map_err(|e| CompilerError::CodeGen(format!("Error loading function pointer: {:?}", e)))?;

                                                // Generate arguments
                                                let mut arg_values = Vec::new();
                                                for arg in args {
                                                    arg_values.push(self.generate_expression(arg)?.into());
                                                }

                                                // Call the function indirectly
                                                let call = self.builder.build_indirect_call(
                                                    fn_type,
                                                    func_ptr.into_pointer_value(),
                                                    &arg_values,
                                                    &format!("call_{}", name)
                                                ).map_err(|e| CompilerError::CodeGen(format!("Error building indirect call: {:?}", e)))?;

                                                Ok(call.try_as_basic_value().left().unwrap())
                                            }
                                            _ => Err(CompilerError::CodeGen(format!("Variable '{}' is not a function", name))),
                                        }
                                    } else {
                                        Err(CompilerError::CodeGen(format!("Variable '{}' not found", name)))
                                    }
                                } else {
                                    Err(CompilerError::CodeGen(format!("Unknown function: {}", name)))
                                }
                            }
                        }
                    }
                    _ => Err(CompilerError::CodeGen("Function calls through expressions not yet supported".to_string())),
                }
            }
            Expr::Function { params, body } => {
                self.generate_anonymous_function(&params, &body)
            }
        }
    }

    /// Generates code for module function calls (e.g., matematica.absoluto)
    fn generate_module_function_call(&mut self, name: &str, args: &[Expr]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let parts: Vec<&str> = name.split('.').collect();
        if parts.len() != 2 {
            return Err(CompilerError::CodeGen(format!("Invalid module function call: {}", name)));
        }

        let module_name = parts[0];
        let function_name = parts[1];

        // Get the function first to avoid borrowing issues
        let func = if let Some(module_functions) = self.modules.get(module_name) {
            if let Some(func) = module_functions.get(function_name) {
                *func
            } else {
                return Err(CompilerError::CodeGen(format!("Function '{}' not found in module '{}'", function_name, module_name)));
            }
        } else {
            return Err(CompilerError::CodeGen(format!("Module '{}' not found", module_name)));
        };

        // Generate arguments
        let mut arg_values = Vec::new();
        for arg in args {
            let arg_val = self.generate_expression(arg)?;
            arg_values.push(arg_val.into());
        }

        // Call the function
        let call = self.builder.build_call(func, &arg_values, "module_call")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling module function: {:?}", e)))?;

        if let Some(return_val) = call.try_as_basic_value().left() {
            Ok(return_val)
        } else {
            // Void function, return a dummy value
            let zero = self.i64_type.const_int(0, false);
            Ok(zero.into())
        }
    }

    /// Generates code for user-defined function calls
    fn generate_user_function_call(&mut self, func: &FunctionValue<'ctx>, args: &[Expr]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.generate_expression(arg)?.into());
        }

        let call = self.builder.build_call(*func, &arg_values, "user_call")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling user function: {:?}", e)))?;

        if let Some(return_val) = call.try_as_basic_value().left() {
            Ok(return_val)
        } else {
            // Void function, return a dummy value
            let zero = self.i64_type.const_int(0, false);
            Ok(zero.into())
        }
    }

    /// Generates code for escreva() function calls - optimized with cached format strings
    fn generate_escreva_call(&mut self, args: &[Expr]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "escreva() expects exactly one argument".to_string()
            ));
        }

        let arg = self.generate_expression(&args[0])?;
        let printf_fn = self.get_built_in_function("printf")?;

        match arg {
            BasicValueEnum::PointerValue(str_ptr) => {
                let format_ptr = self.get_or_create_format_string("%s\n\0");
                self.safe_build(
                    self.builder.build_call(
                        printf_fn,
                        &[format_ptr.into(), str_ptr.into()],
                        "printf_str_call"
                    ),
                    "printf string call"
                )?;
            }
            BasicValueEnum::IntValue(int_val) => {
                let format_ptr = self.get_or_create_format_string("%lld\n\0");
                self.safe_build(
                    self.builder.build_call(
                        printf_fn,
                        &[format_ptr.into(), int_val.into()],
                        "printf_int_call"
                    ),
                    "printf integer call"
                )?;
            }
            _ => return Err(CompilerError::CodeGen(
                "escreva() supports only string and integer arguments".to_string()
            )),
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

    /// Converts an integer to its string representation - optimized with cached format strings
    fn int_to_string(&mut self, int_val: IntValue<'ctx>) -> Result<PointerValue<'ctx>, CompilerError> {
        // Pre-allocate functions and constants
        let malloc_fn = self.get_built_in_function("malloc")?;
        let sprintf_fn = self.get_built_in_function("sprintf")?;
        
        // Allocate buffer (20 bytes is enough for 64-bit integers)
        let buffer_size = self.i64_type.const_int(20, false);
        let buffer_call = self.safe_build(
            self.builder.build_call(malloc_fn, &[buffer_size.into()], "int_str_buffer"),
            "malloc integer to string buffer"
        )?;
        let buffer = buffer_call.try_as_basic_value()
            .left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to allocate integer string buffer".to_string()))?
            .into_pointer_value();

        // Use cached format string
        let format_ptr = self.get_or_create_format_string("%lld\0");

        // Convert integer to string
        self.safe_build(
            self.builder.build_call(
                sprintf_fn,
                &[buffer.into(), format_ptr.into(), int_val.into()],
                "sprintf_int"
            ),
            "sprintf integer conversion"
        )?;

        Ok(buffer)
    }

    /// Concatenates two strings - optimized with pre-allocated functions and better error handling
    fn generate_string_concat(&mut self, left: PointerValue<'ctx>, right: PointerValue<'ctx>) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        // Get all required functions upfront
        let strlen_fn = self.get_built_in_function("strlen")?;
        let malloc_fn = self.get_built_in_function("malloc")?;
        let strcpy_fn = self.get_built_in_function("strcpy")?;
        let strcat_fn = self.get_built_in_function("strcat")?;
        
        // Calculate string lengths
        let left_len_call = self.safe_build(
            self.builder.build_call(strlen_fn, &[left.into()], "left_len"),
            "strlen left string"
        )?;
        let left_len = left_len_call.try_as_basic_value()
            .left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get left string length".to_string()))?
            .into_int_value();

        let right_len_call = self.safe_build(
            self.builder.build_call(strlen_fn, &[right.into()], "right_len"),
            "strlen right string"
        )?;
        let right_len = right_len_call.try_as_basic_value()
            .left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get right string length".to_string()))?
            .into_int_value();

        // Calculate total length + null terminator
        let total_len = self.safe_build(
            self.builder.build_int_add(left_len, right_len, "total_len"),
            "add string lengths"
        )?;
        let total_len_plus_one = self.safe_build(
            self.builder.build_int_add(
                total_len,
                self.i64_type.const_int(1, false),
                "total_len_plus_one"
            ),
            "add null terminator space"
        )?;

        // Allocate result buffer
        let result_buffer_call = self.safe_build(
            self.builder.build_call(malloc_fn, &[total_len_plus_one.into()], "concat_buffer"),
            "malloc string concatenation buffer"
        )?;
        let result_buffer = result_buffer_call.try_as_basic_value()
            .left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to allocate concatenation buffer".to_string()))?
            .into_pointer_value();

        // Copy strings efficiently
        self.safe_build(
            self.builder.build_call(strcpy_fn, &[result_buffer.into(), left.into()], "strcpy_first"),
            "copy first string"
        )?;
        self.safe_build(
            self.builder.build_call(strcat_fn, &[result_buffer.into(), right.into()], "strcat_second"),
            "concatenate second string"
        )?;

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
    
    // Helper methods for better performance and code reuse
    
    /// Creates or reuses a format string global
    fn get_or_create_format_string(&mut self, format: &str) -> PointerValue<'ctx> {
        if let Some(&ptr) = self.format_strings.get(format) {
            return ptr;
        }
        
        let format_str = self.context.const_string(format.as_bytes(), false);
        let global = self.module.add_global(format_str.get_type(), None, "format_str");
        global.set_initializer(&format_str);
        let ptr = global.as_pointer_value();
        
        self.format_strings.insert(format.to_string(), ptr);
        ptr
    }
    
    /// Converts any value to boolean for conditions - optimized
    #[inline]
    fn value_to_bool(&mut self, val: BasicValueEnum<'ctx>, name: &str) -> Result<inkwell::values::IntValue<'ctx>, CompilerError> {
        match val {
            BasicValueEnum::IntValue(int_val) => {
                if int_val.get_type().get_bit_width() == 1 {
                    Ok(int_val)
                } else {
                    let zero = self.i64_type.const_int(0, false);
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::NE,
                        int_val,
                        zero,
                        name
                    ).map_err(|e| CompilerError::CodeGen(
                        format!("Error converting to boolean: {:?}", e)
                    ))
                }
            }
            _ => Err(CompilerError::CodeGen(
                "Condition must be integer type".to_string()
            )),
        }
    }
    
    /// Safe LLVM operation wrapper with better error context
    #[inline]
    fn safe_build<T>(&self, result: Result<T, inkwell::builder::BuilderError>, operation: &str) -> Result<T, CompilerError> {
        result.map_err(|e| CompilerError::CodeGen(
            format!("Error in {}: {:?}", operation, e)
        ))
    }



    /// Generates code for if statement - optimized with helper methods
    fn generate_if_statement(&mut self, condition: &Expr, then_branch: &[Statement], else_branch: Option<&Vec<Statement>>) -> Result<(), CompilerError> {
        let condition_val = self.generate_expression(condition)?;
        let condition_bool = self.value_to_bool(condition_val, "if_condition")?;

        // Get current function from builder
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        
        let then_bb = self.context.append_basic_block(current_function, "then");
        let merge_bb = self.context.append_basic_block(current_function, "merge");

        let else_bb = if else_branch.is_some() {
            Some(self.context.append_basic_block(current_function, "else"))
        } else {
            None
        };

        self.safe_build(
            self.builder.build_conditional_branch(condition_bool, then_bb, else_bb.unwrap_or(merge_bb)),
            "if conditional branch"
        )?;

        // Generate then branch
        self.builder.position_at_end(then_bb);
        for stmt in then_branch {
            self.generate_statement(stmt)?;
        }
        self.safe_build(
            self.builder.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;

        // Generate else branch if present
        if let Some(else_stmts) = else_branch {
            if let Some(else_block) = else_bb {
                self.builder.position_at_end(else_block);
                for stmt in else_stmts {
                    self.generate_statement(stmt)?;
                }
                self.safe_build(
            self.builder.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;
            }
        }

        self.builder.position_at_end(merge_bb);
        Ok(())
    }

    /// Generates code for if-else-if statement
    fn generate_if_else_if_statement(&mut self, condition: &Expr, then_branch: &[Statement], else_if_branches: &[(Expr, Vec<Statement>)], else_branch: Option<&Vec<Statement>>) -> Result<(), CompilerError> {
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();

        // Generate the main if condition
        let condition_val = self.generate_expression(condition)?;
        let condition_bool = match condition_val {
            BasicValueEnum::IntValue(val) => {
                if val.get_type().get_bit_width() == 1 {
                    val
                } else {
                    let zero = self.i64_type.const_int(0, false);
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::NE,
                        val,
                        zero,
                        "condition_bool"
                    ).unwrap()
                }
            }
            _ => return Err(CompilerError::CodeGen("If condition must be integer".to_string())),
        };

        let then_bb = self.context.append_basic_block(current_function, "then");
        let current_else_bb = self.context.append_basic_block(current_function, "else_if_start");
        let merge_bb = self.context.append_basic_block(current_function, "merge");

        self.builder.build_conditional_branch(condition_bool, then_bb, current_else_bb).unwrap();

        // Generate then branch
        self.builder.position_at_end(then_bb);
        for stmt in then_branch {
            self.generate_statement(stmt)?;
        }
        self.safe_build(
            self.builder.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;

        // Generate else-if chain
        let mut previous_else_bb = current_else_bb;
        for (i, (else_if_condition, else_if_statements)) in else_if_branches.iter().enumerate() {
            self.builder.position_at_end(previous_else_bb);

            let else_if_condition_val = self.generate_expression(else_if_condition)?;
            let else_if_condition_bool = match else_if_condition_val {
                BasicValueEnum::IntValue(val) => {
                    if val.get_type().get_bit_width() == 1 {
                        val
                    } else {
                        let zero = self.i64_type.const_int(0, false);
                        self.builder.build_int_compare(
                            inkwell::IntPredicate::NE,
                            val,
                            zero,
                            &format!("else_if_condition_bool_{}", i)
                        ).unwrap()
                    }
                }
                _ => return Err(CompilerError::CodeGen("Else-if condition must be integer".to_string())),
            };

            let else_if_then_bb = self.context.append_basic_block(current_function, &format!("else_if_then_{}", i));
            let next_else_bb = if i < else_if_branches.len() - 1 {
                self.context.append_basic_block(current_function, &format!("else_if_{}", i + 1))
            } else if else_branch.is_some() {
                self.context.append_basic_block(current_function, "final_else")
            } else {
                merge_bb
            };

            self.builder.build_conditional_branch(else_if_condition_bool, else_if_then_bb, next_else_bb).unwrap();

            // Generate else-if then branch
            self.builder.position_at_end(else_if_then_bb);
            for stmt in else_if_statements {
                self.generate_statement(stmt)?;
            }
            self.safe_build(
            self.builder.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;

            previous_else_bb = next_else_bb;
        }

        // Generate final else branch if present
        if let Some(else_stmts) = else_branch {
            self.builder.position_at_end(previous_else_bb);
            for stmt in else_stmts {
                self.generate_statement(stmt)?;
            }
            self.safe_build(
            self.builder.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;
        } else if !else_if_branches.is_empty() {
            // If no final else but we have else-ifs, the last else-if's false branch should go to merge
            self.builder.position_at_end(previous_else_bb);
            self.safe_build(
            self.builder.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;
        }

        self.builder.position_at_end(merge_bb);
        Ok(())
    }

    /// Generates code for switch statement
    fn generate_switch_statement(&mut self, _value: &Expr, cases: &[(Expr, Vec<Statement>)], default: Option<&Vec<Statement>>) -> Result<(), CompilerError> {
        // For now, implement as if-else chain
        // TODO: Implement proper switch with jump table
        if let Some((first_case_val, first_case_stmts)) = cases.first() {
            let mut else_stmts = Vec::new();
            
            // Add remaining cases as else-if
            for (_case_val, _case_stmts) in &cases[1..] {
                // TODO: Generate comparison and statements
            }
            
            // Add default as else
            if let Some(default_stmts) = default {
                else_stmts.extend_from_slice(default_stmts);
            }
            
            self.generate_if_statement(first_case_val, first_case_stmts, if else_stmts.is_empty() { None } else { Some(&else_stmts) })
        } else {
            Ok(())
        }
    }

    /// Generates code for while statement
    fn generate_while_statement(&mut self, condition: &Expr, body: &[Statement]) -> Result<(), CompilerError> {
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        
        let loop_bb = self.context.append_basic_block(current_function, "loop");
        let body_bb = self.context.append_basic_block(current_function, "loop_body");
        let after_bb = self.context.append_basic_block(current_function, "after_loop");
        
        // Jump to loop condition
        self.builder.build_unconditional_branch(loop_bb).unwrap();
        
        // Generate loop condition
        self.builder.position_at_end(loop_bb);
        let condition_val = self.generate_expression(condition)?;
        let condition_bool = match condition_val {
            BasicValueEnum::IntValue(val) => {
                if val.get_type().get_bit_width() == 1 {
                    val
                } else {
                    let zero = self.i64_type.const_int(0, false);
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::NE,
                        val,
                        zero,
                        "loop_condition"
                    ).unwrap()
                }
            }
            _ => return Err(CompilerError::CodeGen("While condition must be integer".to_string())),
        };
        
        self.builder.build_conditional_branch(condition_bool, body_bb, after_bb).unwrap();
        
        // Generate loop body
        self.builder.position_at_end(body_bb);
        for stmt in body {
            self.generate_statement(stmt)?;
        }
        self.builder.build_unconditional_branch(loop_bb).unwrap();
        
        self.builder.position_at_end(after_bb);
        Ok(())
    }

    /// Generates code for do-while statement
    fn generate_do_while_statement(&mut self, body: &[Statement], condition: &Expr) -> Result<(), CompilerError> {
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        
        let body_bb = self.context.append_basic_block(current_function, "do_body");
        let condition_bb = self.context.append_basic_block(current_function, "do_condition");
        let after_bb = self.context.append_basic_block(current_function, "after_do");
        
        // Jump to body
        self.builder.build_unconditional_branch(body_bb).unwrap();
        
        // Generate loop body
        self.builder.position_at_end(body_bb);
        for stmt in body {
            self.generate_statement(stmt)?;
        }
        self.builder.build_unconditional_branch(condition_bb).unwrap();
        
        // Generate condition
        self.builder.position_at_end(condition_bb);
        let condition_val = self.generate_expression(condition)?;
        let condition_bool = match condition_val {
            BasicValueEnum::IntValue(val) => {
                if val.get_type().get_bit_width() == 1 {
                    val
                } else {
                    let zero = self.i64_type.const_int(0, false);
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::NE,
                        val,
                        zero,
                        "do_condition"
                    ).unwrap()
                }
            }
            _ => return Err(CompilerError::CodeGen("Do-while condition must be integer".to_string())),
        };
        
        self.builder.build_conditional_branch(condition_bool, body_bb, after_bb).unwrap();
        
        self.builder.position_at_end(after_bb);
        Ok(())
    }

    /// Generates code for for statement
    fn generate_for_statement(&mut self, initializer: Option<&Statement>, condition: Option<&Expr>, increment: Option<&Expr>, body: &[Statement]) -> Result<(), CompilerError> {
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        
        let init_bb = self.context.append_basic_block(current_function, "for_init");
        let condition_bb = self.context.append_basic_block(current_function, "for_condition");
        let body_bb = self.context.append_basic_block(current_function, "for_body");
        let increment_bb = self.context.append_basic_block(current_function, "for_increment");
        let after_bb = self.context.append_basic_block(current_function, "after_for");
        
        // Jump to initializer
        self.builder.build_unconditional_branch(init_bb).unwrap();
        
        // Generate initializer
        self.builder.position_at_end(init_bb);
        if let Some(init_stmt) = initializer {
            self.generate_statement(init_stmt)?;
        }
        self.builder.build_unconditional_branch(condition_bb).unwrap();
        
        // Generate condition
        self.builder.position_at_end(condition_bb);
        let condition_bool = if let Some(cond_expr) = condition {
            let condition_val = self.generate_expression(cond_expr)?;
            match condition_val {
                BasicValueEnum::IntValue(val) => {
                    if val.get_type().get_bit_width() == 1 {
                        val
                    } else {
                        let zero = self.i64_type.const_int(0, false);
                        self.builder.build_int_compare(
                            inkwell::IntPredicate::NE,
                            val,
                            zero,
                            "for_condition"
                        ).unwrap()
                    }
                }
                _ => return Err(CompilerError::CodeGen("For condition must be integer".to_string())),
            }
        } else {
            // No condition means infinite loop
            self.context.bool_type().const_int(1, false)
        };
        
        self.builder.build_conditional_branch(condition_bool, body_bb, after_bb).unwrap();
        
        // Generate body
        self.builder.position_at_end(body_bb);
        for stmt in body {
            self.generate_statement(stmt)?;
        }
        self.builder.build_unconditional_branch(increment_bb).unwrap();
        
        // Generate increment
        self.builder.position_at_end(increment_bb);
        if let Some(inc_expr) = increment {
            self.generate_expression(inc_expr)?;
        }
        self.builder.build_unconditional_branch(condition_bb).unwrap();
        
        self.builder.position_at_end(after_bb);
        Ok(())
    }

    /// Generates code for for-each statement
    fn generate_for_each_statement(&mut self, _variable: &str, _iterable: &Expr, _body: &[Statement]) -> Result<(), CompilerError> {
        // TODO: Implement for-each loops
        // For now, just skip
        Ok(())
    }

    /// Generates code for break statement - now with proper loop context
    fn generate_break_statement(&mut self) -> Result<(), CompilerError> {
        if let Some(loop_ctx) = self.loop_stack.last() {
            self.safe_build(
                self.builder.build_unconditional_branch(loop_ctx.break_block),
                "break statement jump"
            )?;
        } else {
            return Err(CompilerError::CodeGen(
                "Break statement outside of loop context".to_string()
            ));
        }
        Ok(())
    }

    /// Generates code for continue statement - now with proper loop context
    fn generate_continue_statement(&mut self) -> Result<(), CompilerError> {
        if let Some(loop_ctx) = self.loop_stack.last() {
            self.safe_build(
                self.builder.build_unconditional_branch(loop_ctx.continue_block),
                "continue statement jump"
            )?;
        } else {
            return Err(CompilerError::CodeGen(
                "Continue statement outside of loop context".to_string()
            ));
        }
        Ok(())
    }

    /// Generates code for function declaration
    fn generate_function_declaration(&mut self, name: Option<&String>, params: &Vec<String>, body: &Vec<Statement>) -> Result<(), CompilerError> {
        if let Some(name) = name {
            // Create function type: i64 return, i64 params
            let param_types = vec![self.i64_type.into(); params.len()];
            let fn_type = self.i64_type.fn_type(&param_types, false);

            let func = self.module.add_function(name, fn_type, None);

            // Store the function
            self.functions.insert(name.clone(), func);

            // Save current builder position and variables
            let current_block = self.builder.get_insert_block();
            let saved_variables = self.variables.clone();

            // Create entry block for the function
            let entry_block = self.context.append_basic_block(func, "entry");
            self.builder.position_at_end(entry_block);

            // Clear variables for function scope (except global functions)
            self.variables.clear();
            // Re-add functions to variables? No, functions are separate.

            // Set up parameters
            for (i, param) in params.iter().enumerate() {
                let param_value = func.get_nth_param(i as u32).unwrap().into_int_value();
                let param_ptr = self.builder.build_alloca(self.i64_type, param)
                    .map_err(|e| CompilerError::CodeGen(format!("Error allocating parameter: {:?}", e)))?;
                self.builder.build_store(param_ptr, param_value)
                    .map_err(|e| CompilerError::CodeGen(format!("Error storing parameter: {:?}", e)))?;
                self.variables.insert(param.clone(), (param_ptr, VariableType::Int(self.i64_type)));
            }

            // Generate body
            for stmt in body {
                self.generate_statement(stmt)?;
            }

            // If no return, add a default return
            if !body.iter().any(|stmt| matches!(stmt, Statement::Return(_))) {
                self.builder.build_return(Some(&self.i64_type.const_int(0, false)))
                    .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;
            }

            // Restore variables and builder position
            self.variables = saved_variables;
            if let Some(block) = current_block {
                self.builder.position_at_end(block);
            }
        }

        Ok(())
    }

    /// Generates code for return statement
    fn generate_return_statement(&mut self, value: Option<&Expr>) -> Result<(), CompilerError> {
        if let Some(expr) = value {
            let val = self.generate_expression(expr)?;
            match val {
                BasicValueEnum::IntValue(int_val) => {
                    self.builder.build_return(Some(&int_val))
                        .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;
                }
                _ => return Err(CompilerError::CodeGen("Return value must be an integer".to_string())),
            }
        } else {
            self.builder.build_return(Some(&self.i64_type.const_int(0, false)))
                .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;
        }
        Ok(())
    }

    /// Generates code for anonymous function
    fn generate_anonymous_function(&mut self, params: &[String], body: &[Statement]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        // Generate a unique name for the anonymous function
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
        }
        let func_name = format!("__anon_func_{}", unsafe { COUNTER });

        // Create function type: i64 return, i64 params
        let param_types = vec![self.i64_type.into(); params.len()];
        let fn_type = self.i64_type.fn_type(&param_types, false);

        let func = self.module.add_function(&func_name, fn_type, None);

        // Save current builder position and variables
        let current_block = self.builder.get_insert_block();
        let saved_variables = self.variables.clone();

        // Create entry block for the function
        let entry_block = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry_block);

        // Clear variables for function scope
        self.variables.clear();

        // Set up parameters
        for (i, param) in params.iter().enumerate() {
            let param_value = func.get_nth_param(i as u32).unwrap().into_int_value();
            let param_ptr = self.builder.build_alloca(self.i64_type, param)
                .map_err(|e| CompilerError::CodeGen(format!("Error allocating parameter: {:?}", e)))?;
            self.builder.build_store(param_ptr, param_value)
                .map_err(|e| CompilerError::CodeGen(format!("Error storing parameter: {:?}", e)))?;
            self.variables.insert(param.clone(), (param_ptr, VariableType::Int(self.i64_type)));
        }

        // Generate body
        for stmt in body {
            self.generate_statement(stmt)?;
        }

        // If no return, add a default return
        if !body.iter().any(|stmt| matches!(stmt, Statement::Return(_))) {
            self.builder.build_return(Some(&self.i64_type.const_int(0, false)))
                .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;
        }

        // Restore variables and builder position
        self.variables = saved_variables;
        if let Some(block) = current_block {
            self.builder.position_at_end(block);
        }

        // Return the function as a pointer
        Ok(func.as_global_value().as_pointer_value().into())
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
    fn test_arithmetic() {
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
    fn test_string_literal() {
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
    fn test_complete_program() {
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
