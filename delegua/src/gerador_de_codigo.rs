
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{IntValue, PointerValue, FunctionValue, BasicValueEnum};
use inkwell::types::{IntType, PointerType, BasicTypeEnum};
use inkwell::AddressSpace;

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

use crate::ast::{Programa, Declaracao, Expressoes, OperacaoBinaria};
use crate::error::CompilerError;
use crate::modules::matematica::Matematica;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    variables: HashMap<String, (PointerValue<'ctx>, VariableType<'ctx>)>,

    functions: HashMap<String, FunctionValue<'ctx>>,

    built_in_functions: HashMap<String, FunctionValue<'ctx>>,

    modules: HashMap<String, HashMap<String, FunctionValue<'ctx>>>,

    i64_type: IntType<'ctx>,
    i8_ptr_type: PointerType<'ctx>,
    
    format_strings: HashMap<String, PointerValue<'ctx>>,
    
    loop_stack: Vec<LoopContext<'ctx>>,
}

#[derive(Debug, Clone, Copy)]
struct LoopContext<'ctx> {
    break_block: inkwell::basic_block::BasicBlock<'ctx>,
    continue_block: inkwell::basic_block::BasicBlock<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Result<Self, CompilerError> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        let i64_type = context.i64_type();
        let i8_ptr_type = context.ptr_type(inkwell::AddressSpace::default());

        let codegen = CodeGen {
            context,
            module,
            builder,
            variables: HashMap::with_capacity(32),
            functions: HashMap::with_capacity(16),
            built_in_functions: HashMap::with_capacity(8),
            modules: HashMap::with_capacity(4),
            format_strings: HashMap::with_capacity(8),
            loop_stack: Vec::with_capacity(8),
            i64_type,
            i8_ptr_type,
        };

        let math_module = Matematica::new(context);
        math_module.declarar_funcoes(&codegen.module);
        math_module.gerar_implementacoes(&codegen.module);

        Ok(codegen)
    }

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
            "scanf" => {
                let scanf_type = self.i64_type.fn_type(&[self.i8_ptr_type.into()], true);
                self.module.add_function("scanf", scanf_type, None)
            }
            _ => return Err(CompilerError::CodeGen(
                format!("Unknown built-in function '{}'. Available: printf, malloc, strlen, strcpy, strcat, sprintf, scanf", name)
            )),
        };

        self.built_in_functions.insert(name.to_string(), fn_val);
        Ok(fn_val)
    }

    pub fn generate(&mut self, program: &Programa) -> Result<(), CompilerError> {
        let main_type = self.i64_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);
        let basic_block = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(basic_block);

        for statement in &program.declaracoes {
            self.generate_statement(statement)?;
        }

        let zero = self.i64_type.const_int(0, false);
        self.builder.build_return(Some(&zero))
            .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;

        Ok(())
    }

    fn generate_statement(&mut self, statement: &Declaracao) -> Result<(), CompilerError> {
        match statement {
            Declaracao::Variavel { nome: name, valor: value } => {
                self.generate_variable_declaration(name, value)
            }
            Declaracao::Atribuicao { nome: name, valor: value } => {
                self.generate_assignment(name, value)
            }
            Declaracao::Importacao { modulo: module, itens: items } => {
                self.generate_import(module, items.as_ref())
            }
            Declaracao::Se { condicao: condition, ramificacao_entao: then_branch, ramificacao_outro: else_branch } => {
                self.generate_if_statement(condition, then_branch, else_branch.as_ref())
            }
            Declaracao::SeSenao { condicao: condition, ramificacao_entao: then_branch, ramificacao_se_outro: else_if_branches, ramificacao_outro: else_branch } => {
                self.generate_if_else_if_statement(condition, then_branch, else_if_branches, else_branch.as_ref())
            }
            Declaracao::Selecao { valor: value, casos: cases, padrao: default } => {
                self.generate_switch_statement(value, cases, default.as_ref())
            }
            Declaracao::Enquanto { condicao: condition, corpo: body } => {
                self.generate_while_statement(condition, body)
            }
            Declaracao::FacaEnquanto { corpo: body, condicao: condition } => {
                self.generate_do_while_statement(body, condition)
            }
            Declaracao::Para { inicializador: initializer, condicao: condition, incremento: increment, corpo: body } => {
                self.generate_for_statement(initializer.as_ref().map(|v| &**v), condition.as_ref(), increment.as_ref(), body)
            }
            Declaracao::ParaCada { variavel: variable, iteravel: iterable, corpo: body } => {
                self.generate_for_each_statement(variable, iterable, body)
            }
            Declaracao::Sustar => {
                self.generate_break_statement()
            }
            Declaracao::Continua => {
                self.generate_continue_statement()
            }
            Declaracao::ChamadaDeFuncao(expr) => {
                self.generate_expression(expr)?;
                Ok(())
            }
            Declaracao::DeclaracaoDeFuncao { nome: name, parametros: params, corpo: body } => {
                self.generate_function_declaration(name.as_ref(), params, body)
            }
            Declaracao::Retorna(value) => {
                self.generate_return_statement(value.as_ref())
            }
        }
    }

    fn generate_import(&mut self, module: &str, _items: Option<&Vec<String>>) -> Result<(), CompilerError> {
        match module {
            "matematica" => {
                let math_module = Matematica::new(self.context);
                let functions = math_module.declarar_funcoes(&self.module);
                math_module.gerar_implementacoes(&self.module);

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

    fn generate_variable_declaration(&mut self, name: &str, value: &Expressoes) -> Result<(), CompilerError> {
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
                    Expressoes::Funcao { .. } => {
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

    fn generate_assignment(&mut self, name: &str, value: &Expressoes) -> Result<(), CompilerError> {
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

    fn generate_expression(&mut self, expr: &Expressoes) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        match expr {
            Expressoes::Numero(n) => {
                let val = self.i64_type.const_int(*n as u64, false);
                Ok(val.into())
            }

            Expressoes::Texto(s) => {
                let string_val = self.context.const_string(s.as_bytes(), true);
                let global = self.module.add_global(string_val.get_type(), None, "string_literal");
                global.set_initializer(&string_val);
                let ptr = global.as_pointer_value();
                Ok(ptr.into())
            }

            Expressoes::Logico(b) => {
                let val = self.i64_type.const_int(if *b { 1 } else { 0 }, false);
                Ok(val.into())
            }

            Expressoes::Identificador(name) => {
                if let Some(&(var_ptr, var_type)) = self.variables.get(name) {
                    let loaded = self.builder.build_load(var_type.as_basic_type_enum(), var_ptr, name)
                        .map_err(|e| CompilerError::CodeGen(format!("Error loading variable: {:?}", e)))?;
                    Ok(loaded)
                } else {
                    Err(CompilerError::CodeGen(format!("Variable '{}' not found", name)))
                }
            }

            Expressoes::Binario { esquerda: left, operador: operator, direita: right } => {
                let left_val = self.generate_expression(left)?;
                let right_val = self.generate_expression(right)?;

                match (left_val, right_val) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        let (result, _op_name) = match operator {
                            OperacaoBinaria::Adicao => (self.safe_build(self.builder.build_int_add(l, r, "add"), "integer addition")?, "add"),
                            OperacaoBinaria::Subtracao => (self.safe_build(self.builder.build_int_sub(l, r, "sub"), "integer subtraction")?, "sub"),
                            OperacaoBinaria::Multiplicacao => (self.safe_build(self.builder.build_int_mul(l, r, "mul"), "integer multiplication")?, "mul"),
                            OperacaoBinaria::Divisao => (self.safe_build(self.builder.build_int_signed_div(l, r, "div"), "integer division")?, "div"),
                            OperacaoBinaria::Modulo => (self.safe_build(self.builder.build_int_signed_rem(l, r, "mod"), "integer modulo")?, "mod"),
                            OperacaoBinaria::Potencia => (self.generate_integer_power(l, r)?, "pow"),
                            OperacaoBinaria::Igual => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::EQ, l, r, "eq"), "equality comparison")?, "eq"),
                            OperacaoBinaria::NaoIgual => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::NE, l, r, "ne"), "inequality comparison")?, "ne"),
                            OperacaoBinaria::Menor => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::SLT, l, r, "lt"), "less than comparison")?, "lt"),
                            OperacaoBinaria::Maior => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::SGT, l, r, "gt"), "greater than comparison")?, "gt"),
                            OperacaoBinaria::MenorIgual => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::SLE, l, r, "le"), "less equal comparison")?, "le"),
                            OperacaoBinaria::MaiorIgual => (self.safe_build(self.builder.build_int_compare(inkwell::IntPredicate::SGE, l, r, "ge"), "greater equal comparison")?, "ge"),
                            OperacaoBinaria::E => (self.safe_build(self.builder.build_and(l, r, "and"), "logical and")?, "and"),
                            OperacaoBinaria::Ou => (self.safe_build(self.builder.build_or(l, r, "or"), "logical or")?, "or"),
                            _ => return Err(CompilerError::CodeGen(format!("Unsupported binary operator for integers: {:?}", operator))),
                        };
                        Ok(result.into())
                    }

                    (BasicValueEnum::PointerValue(l), BasicValueEnum::PointerValue(r)) => {
                        if matches!(operator, OperacaoBinaria::Adicao) {
                            self.generate_string_concat(l, r)
                        } else {
                            Err(CompilerError::CodeGen("Only concatenation (+) is supported for strings".to_string()))
                        }
                    }

                    (BasicValueEnum::PointerValue(l), BasicValueEnum::IntValue(r)) => {
                        if matches!(operator, OperacaoBinaria::Adicao) {
                            let r_str = self.int_to_string(r)?;
                            self.generate_string_concat(l, r_str)
                        } else {
                            Err(CompilerError::CodeGen("Only concatenation (+) is supported between string and number".to_string()))
                        }
                    }

                    _ => Err(CompilerError::CodeGen("Unsupported operand types for binary operation".to_string())),
                }
            }

            Expressoes::Unario { operador: operator, operando: operand } => {
                let operand_val = self.generate_expression(operand)?;
                match operand_val {
                    BasicValueEnum::IntValue(val) => {
                        let result = match operator {
                            OperacaoBinaria::Subtracao => self.builder.build_int_neg(val, "neg")
                                .map_err(|e| CompilerError::CodeGen(format!("Error building unary operation: {:?}", e)))?,
                            OperacaoBinaria::Nao => self.builder.build_not(val, "not")
                                .map_err(|e| CompilerError::CodeGen(format!("Error building logical not operation: {:?}", e)))?,
                            _ => return Err(CompilerError::CodeGen("Unsupported unary operator".to_string())),
                        };
                        Ok(result.into())
                    }
                    _ => Err(CompilerError::CodeGen("Unary operations only supported for integers".to_string())),
                }
            }

            Expressoes::ChamadaFuncao { chamado: callee, argumentos: args } => {
                match callee.as_ref() {
                    Expressoes::Identificador(name) => {
                        match name.as_str() {
                            "escreva" => self.generate_escreva_call(args),
                            "texto" => self.generate_texto_call(args),
                            "leia" => self.generate_leia_call(args),
                            "comprimento" => self.generate_comprimento_call(args),
                            "maiuscula" => self.generate_maiuscula_call(args),
                            "minuscula" => self.generate_minuscula_call(args),
                            "absoluto" => self.generate_absoluto_call(args),
                            "potencia" => self.generate_potencia_call(args),
                            "raiz_quadrada" => self.generate_raiz_quadrada_call(args),
                            _ => {
                                if let Some(func) = self.functions.get(name) {
                                    let func = *func;
                                    self.generate_user_function_call(&func, args)
                                } else if name.contains('.') {
                                    self.generate_module_function_call(name, args)
                                } else if self.variables.contains_key(name) {
                                    if let Some(&(var_ptr, var_type)) = self.variables.get(name) {
                                        match var_type {
                                            VariableType::Function(func_ptr_type) => {
                                                let param_types = vec![self.i64_type.into(); args.len()];
                                                let fn_type = self.i64_type.fn_type(&param_types, false);

                                                let func_ptr = self.builder.build_load(func_ptr_type, var_ptr, &format!("load_func_{}", name))
                                                    .map_err(|e| CompilerError::CodeGen(format!("Error loading function pointer: {:?}", e)))?;

                                                let mut arg_values = Vec::new();
                                                for arg in args {
                                                    arg_values.push(self.generate_expression(arg)?.into());
                                                }

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
            Expressoes::Funcao { paramentros: params, corpo: body } => {
                self.generate_anonymous_function(&params, &body)
            }
            
            Expressoes::Incremento { operando: operand, prefixo: prefix } => {
                self.generate_increment_decrement(operand, true, *prefix)
            }
            
            Expressoes::Decremento { operando: operand, prefixo: prefix } => {
                self.generate_increment_decrement(operand, false, *prefix)
            }

            Expressoes::Lista { elementos: elements } => {
                self.generate_array_literal(elements)
            }

            Expressoes::Indice { lista: array, indice: index } => {
                self.generate_array_index(array, index)
            }

            Expressoes::Objeto { propriedades: properties } => {
                self.generate_object_literal(properties)
            }

            Expressoes::PropriedadeAcesso { objeto: object, propriedade: property } => {
                self.generate_property_access(object, property)
            }
        }
    }

    fn generate_module_function_call(&mut self, name: &str, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let parts: Vec<&str> = name.split('.').collect();
        if parts.len() != 2 {
            return Err(CompilerError::CodeGen(format!("Invalid module function call: {}", name)));
        }

        let module_name = parts[0];
        let function_name = parts[1];

        let func = if let Some(module_functions) = self.modules.get(module_name) {
            if let Some(func) = module_functions.get(function_name) {
                *func
            } else {
                return Err(CompilerError::CodeGen(format!("Function '{}' not found in module '{}'", function_name, module_name)));
            }
        } else {
            return Err(CompilerError::CodeGen(format!("Module '{}' not found", module_name)));
        };

        let mut arg_values = Vec::new();
        for arg in args {
            let arg_val = self.generate_expression(arg)?;
            arg_values.push(arg_val.into());
        }

        let call = self.builder.build_call(func, &arg_values, "module_call")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling module function: {:?}", e)))?;

        if let Some(return_val) = call.try_as_basic_value().left() {
            Ok(return_val)
        } else {
            let zero = self.i64_type.const_int(0, false);
            Ok(zero.into())
        }
    }

    fn generate_user_function_call(&mut self, func: &FunctionValue<'ctx>, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.generate_expression(arg)?.into());
        }

        let call = self.builder.build_call(*func, &arg_values, "user_call")
            .map_err(|e| CompilerError::CodeGen(format!("Error calling user function: {:?}", e)))?;

        if let Some(return_val) = call.try_as_basic_value().left() {
            Ok(return_val)
        } else {
            let zero = self.i64_type.const_int(0, false);
            Ok(zero.into())
        }
    }

    fn generate_escreva_call(&mut self, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
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

    fn generate_texto_call(&mut self, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
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

    fn generate_leia_call(&mut self, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let scanf_fn = self.get_built_in_function("scanf")?;
        let malloc_fn = self.get_built_in_function("malloc")?;

        if args.is_empty() {
            let buffer_size = self.i64_type.const_int(256, false);
            let buffer_call = self.safe_build(
                self.builder.build_call(malloc_fn, &[buffer_size.into()], "input_buffer"),
                "malloc for input buffer"
            )?;
            let buffer = buffer_call.try_as_basic_value().left()
                .ok_or_else(|| CompilerError::CodeGen("Failed to get buffer from malloc".to_string()))?
                .into_pointer_value();

            let format_ptr = self.get_or_create_format_string("%255s\0");
            self.safe_build(
                self.builder.build_call(
                    scanf_fn,
                    &[format_ptr.into(), buffer.into()],
                    "scanf_call"
                ),
                "scanf call"
            )?;

            Ok(buffer.into())
        } else if args.len() == 1 {
            let prompt = self.generate_expression(&args[0])?;
            if let BasicValueEnum::PointerValue(prompt_ptr) = prompt {
                let printf_fn = self.get_built_in_function("printf")?;
                let format_ptr = self.get_or_create_format_string("%s\0");
                self.safe_build(
                    self.builder.build_call(
                        printf_fn,
                        &[format_ptr.into(), prompt_ptr.into()],
                        "printf_prompt"
                    ),
                    "printf prompt call"
                )?;
            }

            let buffer_size = self.i64_type.const_int(256, false);
            let buffer_call = self.safe_build(
                self.builder.build_call(malloc_fn, &[buffer_size.into()], "input_buffer"),
                "malloc for input buffer"
            )?;
            let buffer = buffer_call.try_as_basic_value().left()
                .ok_or_else(|| CompilerError::CodeGen("Failed to get buffer from malloc".to_string()))?
                .into_pointer_value();

            let format_ptr = self.get_or_create_format_string("%255s\0");
            self.safe_build(
                self.builder.build_call(
                    scanf_fn,
                    &[format_ptr.into(), buffer.into()],
                    "scanf_call"
                ),
                "scanf call"
            )?;

            Ok(buffer.into())
        } else {
            Err(CompilerError::CodeGen(
                "leia() expects 0 or 1 argument".to_string()
            ))
        }
    }

    fn generate_comprimento_call(&mut self, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "comprimento() expects exactly 1 argument".to_string()
            ));
        }

        let string_arg = self.generate_expression(&args[0])?;

        let strlen_fn = self.get_built_in_function("strlen")?;

        let length_call = self.safe_build(
            self.builder.build_call(strlen_fn, &[string_arg.into()], "strlen_call"),
            "call strlen function"
        )?;

        let length_value = length_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get length from strlen".to_string()))?;

        Ok(length_value)
    }

    fn generate_maiuscula_call(&mut self, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "maiuscula() expects exactly 1 argument".to_string()
            ));
        }

        let string_arg = self.generate_expression(&args[0])?;

        let malloc_fn = self.get_built_in_function("malloc")?;
        let strlen_fn = self.get_built_in_function("strlen")?;
        let strcpy_fn = self.get_built_in_function("strcpy")?;

        let length_call = self.safe_build(
            self.builder.build_call(strlen_fn, &[string_arg.into()], "strlen_call"),
            "call strlen for maiuscula"
        )?;
        let length = length_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get length".to_string()))?
            .into_int_value();

        let one = self.i64_type.const_int(1, false);
        let buffer_size = self.safe_build(
            self.builder.build_int_add(length, one, "buffer_size"),
            "calculate buffer size"
        )?;

        let buffer_call = self.safe_build(
            self.builder.build_call(malloc_fn, &[buffer_size.into()], "upper_buffer"),
            "malloc for uppercase buffer"
        )?;
        let buffer = buffer_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to allocate buffer".to_string()))?
            .into_pointer_value();

        self.safe_build(
            self.builder.build_call(strcpy_fn, &[buffer.into(), string_arg.into()], "strcpy_call"),
            "copy string to buffer"
        )?;

        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let loop_bb = self.context.append_basic_block(current_function, "upper_loop");
        let loop_body_bb = self.context.append_basic_block(current_function, "upper_body");
        let after_loop_bb = self.context.append_basic_block(current_function, "upper_done");

        let counter = self.safe_build(
            self.builder.build_alloca(self.i64_type, "upper_counter"),
            "allocate counter"
        )?;
        let zero = self.i64_type.const_int(0, false);
        self.safe_build(
            self.builder.build_store(counter, zero),
            "initialize counter"
        )?;

        self.safe_build(
            self.builder.build_unconditional_branch(loop_bb),
            "branch to loop"
        )?;

        self.builder.position_at_end(loop_bb);
        let current_counter = self.safe_build(
            self.builder.build_load(self.i64_type, counter, "load_counter"),
            "load counter"
        )?.into_int_value();

        let is_end = self.safe_build(
            self.builder.build_int_compare(inkwell::IntPredicate::EQ, current_counter, length, "is_end"),
            "check if end reached"
        )?;

        self.safe_build(
            self.builder.build_conditional_branch(is_end, after_loop_bb, loop_body_bb),
            "conditional branch"
        )?;

        self.builder.position_at_end(loop_body_bb);

        let char_ptr = unsafe {
            self.safe_build(
                self.builder.build_gep(self.context.i8_type(), buffer, &[current_counter], "char_ptr"),
                "get character pointer"
            )?
        };

        let char_val = self.safe_build(
            self.builder.build_load(self.context.i8_type(), char_ptr, "char_val"),
            "load character"
        )?.into_int_value();

        let char_as_i64 = self.safe_build(
            self.builder.build_int_z_extend(char_val, self.i64_type, "char_ext"),
            "extend character to i64"
        )?;

        let a_val = self.i64_type.const_int(97, false);  // 'a'
        let z_val = self.i64_type.const_int(122, false); // 'z'

        let is_ge_a = self.safe_build(
            self.builder.build_int_compare(inkwell::IntPredicate::SGE, char_as_i64, a_val, "is_ge_a"),
            "check if >= 'a'"
        )?;

        let is_le_z = self.safe_build(
            self.builder.build_int_compare(inkwell::IntPredicate::SLE, char_as_i64, z_val, "is_le_z"),
            "check if <= 'z'"
        )?;

        let is_lowercase = self.safe_build(
            self.builder.build_and(is_ge_a, is_le_z, "is_lowercase"),
            "check if lowercase"
        )?;

        let diff = self.i64_type.const_int(32, false);
        let upper_char = self.safe_build(
            self.builder.build_int_sub(char_as_i64, diff, "upper_char"),
            "convert to uppercase"
        )?;

        let final_char = self.safe_build(
            self.builder.build_select(is_lowercase, upper_char, char_as_i64, "final_char"),
            "select final character"
        )?;

        let final_char_i8 = self.safe_build(
            self.builder.build_int_truncate(final_char.into_int_value(), self.context.i8_type(), "final_char_i8"),
            "truncate to i8"
        )?;

        self.safe_build(
            self.builder.build_store(char_ptr, final_char_i8),
            "store uppercase character"
        )?;

        let incremented = self.safe_build(
            self.builder.build_int_add(current_counter, one, "increment"),
            "increment counter"
        )?;

        self.safe_build(
            self.builder.build_store(counter, incremented),
            "store incremented counter"
        )?;

        self.safe_build(
            self.builder.build_unconditional_branch(loop_bb),
            "branch back to loop"
        )?;

        self.builder.position_at_end(after_loop_bb);
        Ok(buffer.into())
    }

    fn generate_minuscula_call(&mut self, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "minuscula() expects exactly 1 argument".to_string()
            ));
        }

        let string_arg = self.generate_expression(&args[0])?;

        let malloc_fn = self.get_built_in_function("malloc")?;
        let strlen_fn = self.get_built_in_function("strlen")?;
        let strcpy_fn = self.get_built_in_function("strcpy")?;

        let length_call = self.safe_build(
            self.builder.build_call(strlen_fn, &[string_arg.into()], "strlen_call"),
            "call strlen for minuscula"
        )?;
        let length = length_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get length".to_string()))?
            .into_int_value();

        let one = self.i64_type.const_int(1, false);
        let buffer_size = self.safe_build(
            self.builder.build_int_add(length, one, "buffer_size"),
            "calculate buffer size"
        )?;

        let buffer_call = self.safe_build(
            self.builder.build_call(malloc_fn, &[buffer_size.into()], "lower_buffer"),
            "malloc for lowercase buffer"
        )?;
        let buffer = buffer_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to allocate buffer".to_string()))?
            .into_pointer_value();

        self.safe_build(
            self.builder.build_call(strcpy_fn, &[buffer.into(), string_arg.into()], "strcpy_call"),
            "copy string to buffer"
        )?;

        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let loop_bb = self.context.append_basic_block(current_function, "lower_loop");
        let loop_body_bb = self.context.append_basic_block(current_function, "lower_body");
        let after_loop_bb = self.context.append_basic_block(current_function, "lower_done");

        let counter = self.safe_build(
            self.builder.build_alloca(self.i64_type, "lower_counter"),
            "allocate counter"
        )?;
        let zero = self.i64_type.const_int(0, false);
        self.safe_build(
            self.builder.build_store(counter, zero),
            "initialize counter"
        )?;

        self.safe_build(
            self.builder.build_unconditional_branch(loop_bb),
            "branch to loop"
        )?;

        self.builder.position_at_end(loop_bb);
        let current_counter = self.safe_build(
            self.builder.build_load(self.i64_type, counter, "load_counter"),
            "load counter"
        )?.into_int_value();

        let is_end = self.safe_build(
            self.builder.build_int_compare(inkwell::IntPredicate::EQ, current_counter, length, "is_end"),
            "check if end reached"
        )?;

        self.safe_build(
            self.builder.build_conditional_branch(is_end, after_loop_bb, loop_body_bb),
            "conditional branch"
        )?;

        self.builder.position_at_end(loop_body_bb);

        let char_ptr = unsafe {
            self.safe_build(
                self.builder.build_gep(self.context.i8_type(), buffer, &[current_counter], "char_ptr"),
                "get character pointer"
            )?
        };

        let char_val = self.safe_build(
            self.builder.build_load(self.context.i8_type(), char_ptr, "char_val"),
            "load character"
        )?.into_int_value();

        let char_as_i64 = self.safe_build(
            self.builder.build_int_z_extend(char_val, self.i64_type, "char_ext"),
            "extend character to i64"
        )?;

        let a_val = self.i64_type.const_int(65, false);  // 'A'
        let z_val = self.i64_type.const_int(90, false);  // 'Z'

        let is_ge_a = self.safe_build(
            self.builder.build_int_compare(inkwell::IntPredicate::SGE, char_as_i64, a_val, "is_ge_A"),
            "check if >= 'A'"
        )?;

        let is_le_z = self.safe_build(
            self.builder.build_int_compare(inkwell::IntPredicate::SLE, char_as_i64, z_val, "is_le_Z"),
            "check if <= 'Z'"
        )?;

        let is_uppercase = self.safe_build(
            self.builder.build_and(is_ge_a, is_le_z, "is_uppercase"),
            "check if uppercase"
        )?;

        let diff = self.i64_type.const_int(32, false);
        let lower_char = self.safe_build(
            self.builder.build_int_add(char_as_i64, diff, "lower_char"),
            "convert to lowercase"
        )?;

        let final_char = self.safe_build(
            self.builder.build_select(is_uppercase, lower_char, char_as_i64, "final_char"),
            "select final character"
        )?;

        let final_char_i8 = self.safe_build(
            self.builder.build_int_truncate(final_char.into_int_value(), self.context.i8_type(), "final_char_i8"),
            "truncate to i8"
        )?;

        self.safe_build(
            self.builder.build_store(char_ptr, final_char_i8),
            "store lowercase character"
        )?;

        let incremented = self.safe_build(
            self.builder.build_int_add(current_counter, one, "increment"),
            "increment counter"
        )?;

        self.safe_build(
            self.builder.build_store(counter, incremented),
            "store incremented counter"
        )?;

        self.safe_build(
            self.builder.build_unconditional_branch(loop_bb),
            "branch back to loop"
        )?;

        self.builder.position_at_end(after_loop_bb);
        Ok(buffer.into())
    }

    fn generate_absoluto_call(&mut self, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "absoluto() expects exactly 1 argument".to_string()
            ));
        }

        let arg_value = self.generate_expression(&args[0])?;

        let absoluto_fn = self.module.get_function("matematica_absoluto")
            .ok_or_else(|| CompilerError::CodeGen("matematica_absoluto function not found".to_string()))?;

        let call_result = self.safe_build(
            self.builder.build_call(absoluto_fn, &[arg_value.into()], "absoluto_call"),
            "call absoluto function"
        )?;

        call_result.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get result from absoluto".to_string()))
    }

    fn generate_potencia_call(&mut self, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 2 {
            return Err(CompilerError::CodeGen(
                "potencia() expects exactly 2 arguments".to_string()
            ));
        }

        let base_value = self.generate_expression(&args[0])?;
        let exp_value = self.generate_expression(&args[1])?;

        let potencia_fn = self.module.get_function("matematica_potencia")
            .ok_or_else(|| CompilerError::CodeGen("matematica_potencia function not found".to_string()))?;

        let call_result = self.safe_build(
            self.builder.build_call(potencia_fn, &[base_value.into(), exp_value.into()], "potencia_call"),
            "call potencia function"
        )?;

        call_result.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get result from potencia".to_string()))
    }

    fn generate_raiz_quadrada_call(&mut self, args: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "raiz_quadrada() expects exactly 1 argument".to_string()
            ));
        }

        let arg_value = self.generate_expression(&args[0])?;

        let raiz_fn = self.module.get_function("matematica_raiz_quadrada")
            .ok_or_else(|| CompilerError::CodeGen("matematica_raiz_quadrada function not found".to_string()))?;

        let call_result = self.safe_build(
            self.builder.build_call(raiz_fn, &[arg_value.into()], "raiz_quadrada_call"),
            "call raiz_quadrada function"
        )?;

        call_result.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get result from raiz_quadrada".to_string()))
    }

    fn int_to_string(&mut self, int_val: IntValue<'ctx>) -> Result<PointerValue<'ctx>, CompilerError> {
        let malloc_fn = self.get_built_in_function("malloc")?;
        let sprintf_fn = self.get_built_in_function("sprintf")?;
        
        let buffer_size = self.i64_type.const_int(20, false);
        let buffer_call = self.safe_build(
            self.builder.build_call(malloc_fn, &[buffer_size.into()], "int_str_buffer"),
            "malloc integer to string buffer"
        )?;
        let buffer = buffer_call.try_as_basic_value()
            .left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to allocate integer string buffer".to_string()))?
            .into_pointer_value();

        let format_ptr = self.get_or_create_format_string("%lld\0");

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

    fn generate_string_concat(&mut self, left: PointerValue<'ctx>, right: PointerValue<'ctx>) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let strlen_fn = self.get_built_in_function("strlen")?;
        let malloc_fn = self.get_built_in_function("malloc")?;
        let strcpy_fn = self.get_built_in_function("strcpy")?;
        let strcat_fn = self.get_built_in_function("strcat")?;
        
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

        let result_buffer_call = self.safe_build(
            self.builder.build_call(malloc_fn, &[total_len_plus_one.into()], "concat_buffer"),
            "malloc string concatenation buffer"
        )?;
        let result_buffer = result_buffer_call.try_as_basic_value()
            .left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to allocate concatenation buffer".to_string()))?
            .into_pointer_value();

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

    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn print_ir(&self) {
        println!("{}", self.module.print_to_string().to_string());
    }

    pub fn get_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
    
    
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

    fn get_or_create_string_literal(&mut self, text: &str) -> PointerValue<'ctx> {
        if let Some(&ptr) = self.format_strings.get(text) {
            return ptr;
        }

        let string_val = self.context.const_string(text.as_bytes(), true);
        let global = self.module.add_global(string_val.get_type(), None, "string_literal");
        global.set_initializer(&string_val);
        let ptr = global.as_pointer_value();

        self.format_strings.insert(text.to_string(), ptr);
        ptr
    }

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

    fn value_to_int(&mut self, val: BasicValueEnum<'ctx>, name: &str) -> Result<inkwell::values::IntValue<'ctx>, CompilerError> {
        match val {
            BasicValueEnum::IntValue(int_val) => Ok(int_val),
            _ => Err(CompilerError::CodeGen(
                format!("Expected integer value for {}", name)
            )),
        }
    }

    #[inline]
    fn safe_build<T>(&self, result: Result<T, inkwell::builder::BuilderError>, operation: &str) -> Result<T, CompilerError> {
        result.map_err(|e| CompilerError::CodeGen(
            format!("Error in {}: {:?}", operation, e)
        ))
    }
    
    fn generate_integer_power(&mut self, base: inkwell::values::IntValue<'ctx>, exp: inkwell::values::IntValue<'ctx>) -> Result<inkwell::values::IntValue<'ctx>, CompilerError> {
        
        let current_function = self.builder.get_insert_block()
            .ok_or_else(|| CompilerError::CodeGen("No current basic block for power operation".to_string()))?
            .get_parent()
            .ok_or_else(|| CompilerError::CodeGen("No current function for power operation".to_string()))?;
            
        let _entry_bb = self.builder.get_insert_block().unwrap();
        let loop_bb = self.context.append_basic_block(current_function, "pow_loop");
        let exit_bb = self.context.append_basic_block(current_function, "pow_exit");
        
        let result_ptr = self.safe_build(
            self.builder.build_alloca(self.i64_type, "pow_result"),
            "alloca pow result"
        )?;
        let counter_ptr = self.safe_build(
            self.builder.build_alloca(self.i64_type, "pow_counter"), 
            "alloca pow counter"
        )?;
        
        let one = self.i64_type.const_int(1, false);
        let zero = self.i64_type.const_int(0, false);
        
        self.safe_build(
            self.builder.build_store(result_ptr, one),
            "store initial pow result"
        )?;
        self.safe_build(
            self.builder.build_store(counter_ptr, exp),
            "store initial pow counter"
        )?;
        
        self.safe_build(
            self.builder.build_unconditional_branch(loop_bb),
            "jump to pow loop"
        )?;
        
        self.builder.position_at_end(loop_bb);
        let counter_val = self.safe_build(
            self.builder.build_load(self.i64_type, counter_ptr, "load_counter"),
            "load pow counter"
        )?.into_int_value();
        
        let is_positive = self.safe_build(
            self.builder.build_int_compare(inkwell::IntPredicate::SGT, counter_val, zero, "counter_positive"),
            "compare counter with zero"
        )?;
        
        let multiply_bb = self.context.append_basic_block(current_function, "pow_multiply");
        self.safe_build(
            self.builder.build_conditional_branch(is_positive, multiply_bb, exit_bb),
            "pow loop condition"
        )?;
        
        self.builder.position_at_end(multiply_bb);
        let current_result = self.safe_build(
            self.builder.build_load(self.i64_type, result_ptr, "load_result"),
            "load pow result"
        )?.into_int_value();
        
        let new_result = self.safe_build(
            self.builder.build_int_mul(current_result, base, "pow_multiply"),
            "multiply in pow loop"
        )?;
        let decremented_counter = self.safe_build(
            self.builder.build_int_sub(counter_val, one, "decrement_counter"),
            "decrement pow counter"
        )?;
        
        self.safe_build(
            self.builder.build_store(result_ptr, new_result),
            "store updated pow result"
        )?;
        self.safe_build(
            self.builder.build_store(counter_ptr, decremented_counter),
            "store updated pow counter"
        )?;
        
        self.safe_build(
            self.builder.build_unconditional_branch(loop_bb),
            "jump back to pow loop"
        )?;
        
        self.builder.position_at_end(exit_bb);
        let final_result = self.safe_build(
            self.builder.build_load(self.i64_type, result_ptr, "final_pow_result"),
            "load final pow result"
        )?.into_int_value();
        
        Ok(final_result)
    }
    
    fn generate_increment_decrement(&mut self, operand: &Expressoes, is_increment: bool, prefix: bool) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if let Expressoes::Identificador(name) = operand {
            if let Some(&(var_ptr, var_type)) = self.variables.get(name) {
                match var_type {
                    VariableType::Int(_) => {
                        let current_val = self.safe_build(
                            self.builder.build_load(self.i64_type, var_ptr, &format!("load_{}", name)),
                            &format!("load variable {} for increment/decrement", name)
                        )?.into_int_value();
                        
                        let one = self.i64_type.const_int(1, false);
                        let new_val = if is_increment {
                            self.safe_build(
                                self.builder.build_int_add(current_val, one, &format!("inc_{}", name)),
                                &format!("increment variable {}", name)
                            )?
                        } else {
                            self.safe_build(
                                self.builder.build_int_sub(current_val, one, &format!("dec_{}", name)),
                                &format!("decrement variable {}", name)
                            )?
                        };
                        
                        self.safe_build(
                            self.builder.build_store(var_ptr, new_val),
                            &format!("store {}cremented value to {}", 
                                   if is_increment { "in" } else { "de" }, name)
                        )?;
                        
                        let return_val = if prefix {
                            new_val
                        } else {
                            current_val 
                        };
                        
                        Ok(return_val.into())
                    }
                    _ => Err(CompilerError::CodeGen(
                        format!("Cannot increment/decrement non-integer variable '{}'", name)
                    ))
                }
            } else {
                Err(CompilerError::CodeGen(
                    format!("Variable '{}' not found for increment/decrement", name)
                ))
            }
        } else {
            Err(CompilerError::CodeGen(
                "Increment/decrement only supported on variables for now".to_string()
            ))
        }
    }

    fn generate_array_literal(&mut self, elements: &[Expressoes]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let array_size = elements.len();
        let size_value = self.i64_type.const_int(array_size as u64, false);

        let total_size = self.i64_type.const_int(((array_size + 1) * 8) as u64, false);

        let malloc_fn = self.get_built_in_function("malloc")?;

        let array_ptr = self.safe_build(
            self.builder.build_call(malloc_fn, &[total_size.into()], "array_alloc"),
            "allocate array memory"
        )?;

        let array_ptr = array_ptr.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("malloc call failed".to_string()))?
            .into_pointer_value();

        let array_ptr = self.safe_build(
            self.builder.build_pointer_cast(array_ptr, self.context.ptr_type(AddressSpace::default()), "array_cast"),
            "cast array pointer"
        )?;

        self.safe_build(
            self.builder.build_store(array_ptr, size_value),
            "store array size"
        )?;

        for (i, element) in elements.iter().enumerate() {
            let element_val = self.generate_expression(element)?;
            let element_int = self.value_to_int(element_val, &format!("array_element_{}", i))?;

            let index = self.i64_type.const_int((i + 1) as u64, false);
            let element_ptr = unsafe {
                self.safe_build(
                    self.builder.build_gep(self.i64_type, array_ptr, &[index], &format!("array_elem_ptr_{}", i)),
                    &format!("get pointer to array element {}", i)
                )?
            };

            self.safe_build(
                self.builder.build_store(element_ptr, element_int),
                &format!("store array element {}", i)
            )?;
        }

        Ok(array_ptr.into())
    }

    fn generate_array_index(&mut self, array: &Expressoes, index: &Expressoes) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let array_val = self.generate_expression(array)?;
        let index_val = self.generate_expression(index)?;

        let array_ptr = array_val.into_pointer_value();
        let index_int = self.value_to_int(index_val, "array_index")?;

        let one = self.i64_type.const_int(1, false);
        let adjusted_index = self.safe_build(
            self.builder.build_int_add(index_int, one, "adjusted_index"),
            "adjust array index"
        )?;

        let element_ptr = unsafe {
            self.safe_build(
                self.builder.build_gep(self.i64_type, array_ptr, &[adjusted_index], "array_element_ptr"),
                "get array element pointer"
            )?
        };

        let element_val = self.safe_build(
            self.builder.build_load(self.i64_type, element_ptr, "array_element"),
            "load array element"
        )?;

        Ok(element_val)
    }

    fn generate_object_literal(&mut self, properties: &[(String, Expressoes)]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let obj_size = properties.len();

        let total_size = self.i64_type.const_int(((obj_size * 2 + 1) * 8) as u64, false);

        let malloc_fn = self.get_built_in_function("malloc")?;
        let obj_ptr = self.safe_build(
            self.builder.build_call(malloc_fn, &[total_size.into()], "object_alloc"),
            "allocate object memory"
        )?;

        let obj_ptr = obj_ptr.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("malloc call failed".to_string()))?
            .into_pointer_value();

        let obj_ptr = self.safe_build(
            self.builder.build_pointer_cast(obj_ptr, self.context.ptr_type(AddressSpace::default()), "object_cast"),
            "cast object pointer"
        )?;

        let size_value = self.i64_type.const_int(obj_size as u64, false);
        self.safe_build(
            self.builder.build_store(obj_ptr, size_value),
            "store object size"
        )?;

        for (i, (key, value)) in properties.iter().enumerate() {
            let key_str = self.get_or_create_string_literal(key);

            let key_index = self.i64_type.const_int((i * 2 + 1) as u64, false);
            let value_index = self.i64_type.const_int((i * 2 + 2) as u64, false);

            let key_ptr = unsafe {
                self.safe_build(
                    self.builder.build_gep(self.i64_type, obj_ptr, &[key_index], &format!("obj_key_ptr_{}", i)),
                    &format!("get pointer to object key {}", i)
                )?
            };

            let key_as_int = self.safe_build(
                self.builder.build_ptr_to_int(key_str, self.i64_type, "key_as_int"),
                "convert key string to int"
            )?;

            self.safe_build(
                self.builder.build_store(key_ptr, key_as_int),
                &format!("store object key {}", i)
            )?;

            let value_val = self.generate_expression(value)?;
            let value_int = self.value_to_int(value_val, &format!("object_value_{}", i))?;

            let value_ptr = unsafe {
                self.safe_build(
                    self.builder.build_gep(self.i64_type, obj_ptr, &[value_index], &format!("obj_value_ptr_{}", i)),
                    &format!("get pointer to object value {}", i)
                )?
            };

            self.safe_build(
                self.builder.build_store(value_ptr, value_int),
                &format!("store object value {}", i)
            )?;
        }

        Ok(obj_ptr.into())
    }

    fn generate_property_access(&mut self, object: &Expressoes, property: &str) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let obj_val = self.generate_expression(object)?;
        let obj_ptr = obj_val.into_pointer_value();

        let size_ptr = obj_ptr;
        let size_val = self.safe_build(
            self.builder.build_load(self.i64_type, size_ptr, "object_size"),
            "load object size"
        )?.into_int_value();

        let prop_str = self.get_or_create_string_literal(property);
        let prop_as_int = self.safe_build(
            self.builder.build_ptr_to_int(prop_str, self.i64_type, "prop_as_int"),
            "convert property string to int"
        )?;

        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let loop_bb = self.context.append_basic_block(current_function, "prop_search");
        let found_bb = self.context.append_basic_block(current_function, "prop_found");
        let not_found_bb = self.context.append_basic_block(current_function, "prop_not_found");

        let counter = self.safe_build(
            self.builder.build_alloca(self.i64_type, "counter"),
            "allocate counter"
        )?;
        let zero = self.i64_type.const_int(0, false);
        self.safe_build(
            self.builder.build_store(counter, zero),
            "initialize counter"
        )?;

        self.safe_build(
            self.builder.build_unconditional_branch(loop_bb),
            "branch to loop"
        )?;

        self.builder.position_at_end(loop_bb);
        let current_counter = self.safe_build(
            self.builder.build_load(self.i64_type, counter, "load_counter"),
            "load counter"
        )?.into_int_value();

        let is_end = self.safe_build(
            self.builder.build_int_compare(inkwell::IntPredicate::EQ, current_counter, size_val, "is_end"),
            "check if end reached"
        )?;

        self.safe_build(
            self.builder.build_conditional_branch(is_end, not_found_bb, found_bb),
            "conditional branch"
        )?;

        self.builder.position_at_end(not_found_bb);
        let not_found_val = self.i64_type.const_int(0, false);

        self.builder.position_at_end(found_bb);
        let found_val = self.i64_type.const_int(42, false); // Placeholder value

        Ok(found_val.into())
    }

    fn generate_if_statement(&mut self, condition: &Expressoes, then_branch: &[Declaracao], else_branch: Option<&Vec<Declaracao>>) -> Result<(), CompilerError> {
        let condition_val = self.generate_expression(condition)?;
        let condition_bool = self.value_to_bool(condition_val, "if_condition")?;

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

        self.builder.position_at_end(then_bb);
        for stmt in then_branch {
            self.generate_statement(stmt)?;
        }
        self.safe_build(
            self.builder.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;

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

    fn generate_if_else_if_statement(&mut self, condition: &Expressoes, then_branch: &[Declaracao], else_if_branches: &[(Expressoes, Vec<Declaracao>)], else_branch: Option<&Vec<Declaracao>>) -> Result<(), CompilerError> {
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();

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

        self.builder.position_at_end(then_bb);
        for stmt in then_branch {
            self.generate_statement(stmt)?;
        }
        self.safe_build(
            self.builder.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;

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
            self.builder.position_at_end(previous_else_bb);
            self.safe_build(
            self.builder.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;
        }

        self.builder.position_at_end(merge_bb);
        Ok(())
    }

    fn generate_switch_statement(&mut self, _value: &Expressoes, cases: &[(Expressoes, Vec<Declaracao>)], default: Option<&Vec<Declaracao>>) -> Result<(), CompilerError> {
        if let Some((first_case_val, first_case_stmts)) = cases.first() {
            let mut else_stmts = Vec::new();
            
            for (_case_val, _case_stmts) in &cases[1..] {
            }
            
            if let Some(default_stmts) = default {
                else_stmts.extend_from_slice(default_stmts);
            }
            
            self.generate_if_statement(first_case_val, first_case_stmts, if else_stmts.is_empty() { None } else { Some(&else_stmts) })
        } else {
            Ok(())
        }
    }

    fn generate_while_statement(&mut self, condition: &Expressoes, body: &[Declaracao]) -> Result<(), CompilerError> {
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        
        let loop_bb = self.context.append_basic_block(current_function, "loop");
        let body_bb = self.context.append_basic_block(current_function, "loop_body");
        let after_bb = self.context.append_basic_block(current_function, "after_loop");
        
        self.builder.build_unconditional_branch(loop_bb).unwrap();
        
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
        
        self.builder.position_at_end(body_bb);
        for stmt in body {
            self.generate_statement(stmt)?;
        }
        self.builder.build_unconditional_branch(loop_bb).unwrap();
        
        self.builder.position_at_end(after_bb);
        Ok(())
    }

    fn generate_do_while_statement(&mut self, body: &[Declaracao], condition: &Expressoes) -> Result<(), CompilerError> {
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        
        let body_bb = self.context.append_basic_block(current_function, "do_body");
        let condition_bb = self.context.append_basic_block(current_function, "do_condition");
        let after_bb = self.context.append_basic_block(current_function, "after_do");
        
        self.builder.build_unconditional_branch(body_bb).unwrap();
        
        self.builder.position_at_end(body_bb);
        for stmt in body {
            self.generate_statement(stmt)?;
        }
        self.builder.build_unconditional_branch(condition_bb).unwrap();
        
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

    fn generate_for_statement(&mut self, initializer: Option<&Declaracao>, condition: Option<&Expressoes>, increment: Option<&Expressoes>, body: &[Declaracao]) -> Result<(), CompilerError> {
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        
        let init_bb = self.context.append_basic_block(current_function, "for_init");
        let condition_bb = self.context.append_basic_block(current_function, "for_condition");
        let body_bb = self.context.append_basic_block(current_function, "for_body");
        let increment_bb = self.context.append_basic_block(current_function, "for_increment");
        let after_bb = self.context.append_basic_block(current_function, "after_for");
        
        self.builder.build_unconditional_branch(init_bb).unwrap();
        
        self.builder.position_at_end(init_bb);
        if let Some(init_stmt) = initializer {
            self.generate_statement(init_stmt)?;
        }
        self.builder.build_unconditional_branch(condition_bb).unwrap();
        
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
            self.context.bool_type().const_int(1, false)
        };
        
        self.builder.build_conditional_branch(condition_bool, body_bb, after_bb).unwrap();
        
        self.builder.position_at_end(body_bb);
        for stmt in body {
            self.generate_statement(stmt)?;
        }
        self.builder.build_unconditional_branch(increment_bb).unwrap();
        
        self.builder.position_at_end(increment_bb);
        if let Some(inc_expr) = increment {
            self.generate_expression(inc_expr)?;
        }
        self.builder.build_unconditional_branch(condition_bb).unwrap();
        
        self.builder.position_at_end(after_bb);
        Ok(())
    }

    fn generate_for_each_statement(&mut self, _variable: &str, _iterable: &Expressoes, _body: &[Declaracao]) -> Result<(), CompilerError> {
        Ok(())
    }

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

    fn generate_function_declaration(&mut self, name: Option<&String>, params: &Vec<String>, body: &Vec<Declaracao>) -> Result<(), CompilerError> {
        if let Some(name) = name {
            let param_types = vec![self.i64_type.into(); params.len()];
            let fn_type = self.i64_type.fn_type(&param_types, false);

            let func = self.module.add_function(name, fn_type, None);

            self.functions.insert(name.clone(), func);

            let current_block = self.builder.get_insert_block();
            let saved_variables = self.variables.clone();

            let entry_block = self.context.append_basic_block(func, "entry");
            self.builder.position_at_end(entry_block);

            self.variables.clear();

            for (i, param) in params.iter().enumerate() {
                let param_value = func.get_nth_param(i as u32).unwrap().into_int_value();
                let param_ptr = self.builder.build_alloca(self.i64_type, param)
                    .map_err(|e| CompilerError::CodeGen(format!("Error allocating parameter: {:?}", e)))?;
                self.builder.build_store(param_ptr, param_value)
                    .map_err(|e| CompilerError::CodeGen(format!("Error storing parameter: {:?}", e)))?;
                self.variables.insert(param.clone(), (param_ptr, VariableType::Int(self.i64_type)));
            }

            for stmt in body {
                self.generate_statement(stmt)?;
            }

            if !body.iter().any(|stmt| matches!(stmt, Declaracao::Retorna(_))) {
                self.builder.build_return(Some(&self.i64_type.const_int(0, false)))
                    .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;
            }

            self.variables = saved_variables;
            if let Some(block) = current_block {
                self.builder.position_at_end(block);
            }
        }

        Ok(())
    }

    fn generate_return_statement(&mut self, value: Option<&Expressoes>) -> Result<(), CompilerError> {
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

    fn generate_anonymous_function(&mut self, params: &[String], body: &[Declaracao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
        }
        let func_name = format!("__anon_func_{}", unsafe { COUNTER });

        let param_types = vec![self.i64_type.into(); params.len()];
        let fn_type = self.i64_type.fn_type(&param_types, false);

        let func = self.module.add_function(&func_name, fn_type, None);

        let current_block = self.builder.get_insert_block();
        let saved_variables = self.variables.clone();

        let entry_block = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry_block);

        self.variables.clear();

        for (i, param) in params.iter().enumerate() {
            let param_value = func.get_nth_param(i as u32).unwrap().into_int_value();
            let param_ptr = self.builder.build_alloca(self.i64_type, param)
                .map_err(|e| CompilerError::CodeGen(format!("Error allocating parameter: {:?}", e)))?;
            self.builder.build_store(param_ptr, param_value)
                .map_err(|e| CompilerError::CodeGen(format!("Error storing parameter: {:?}", e)))?;
            self.variables.insert(param.clone(), (param_ptr, VariableType::Int(self.i64_type)));
        }

        for stmt in body {
            self.generate_statement(stmt)?;
        }

        if !body.iter().any(|stmt| matches!(stmt, Declaracao::Retorna(_))) {
            self.builder.build_return(Some(&self.i64_type.const_int(0, false)))
                .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;
        }

        self.variables = saved_variables;
        if let Some(block) = current_block {
            self.builder.position_at_end(block);
        }

        Ok(func.as_global_value().as_pointer_value().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexador::Lexador;
    use crate::analise_sintatica::AnaliseSintatica;

    #[test]
    fn test_simple_variable() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var a = 42;");
        let mut parser = AnaliseSintatica::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("alloca"));
        assert!(ir.contains("store"));
    }

    #[test]
    fn test_arithmetic() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var a = 10; var b = 5; var c = 2; var result = a + b * c;");
        let mut parser = AnaliseSintatica::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("mul"));
        assert!(ir.contains("add"));
    }

    #[test]
    fn test_string_literal() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var msg = \"Hello World\";");
        let mut parser = AnaliseSintatica::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("Hello World"));
    }

    #[test]
    fn test_complete_program() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let code = r#"
            var a = 10;
            var b = 5;
            var soma = a + b;
            escreva("Resultado: " + texto(soma));
        "#;
        let tokens = lexer.analisar(code);
        let mut parser = AnaliseSintatica::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("main"));
        assert!(ir.contains("printf"));
        assert!(ir.contains("add"));
        assert!(ir.contains("Resultado:"));
    }

    #[test]
    fn test_array_literal() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var arr = [1, 2, 3];");
        let mut parser = AnaliseSintatica::new(tokens);
        let program = parser.analisar().unwrap();

        let result = codegen.generate(&program);
        if let Err(ref e) = result {
            println!("Error: {:?}", e);
        }
        assert!(result.is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("malloc"));
        assert!(ir.contains("array_alloc"));
    }

    #[test]
    fn test_array_indexing() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var arr = [10, 20, 30]; var x = arr[1];");
        let mut parser = AnaliseSintatica::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("malloc"));
        assert!(ir.contains("array_element"));
    }

    #[test]
    fn test_empty_array() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var arr = [];");
        let mut parser = AnaliseSintatica::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("malloc"));
    }

    #[test]
    fn test_array_with_expressions() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var a = 5; var b = 10; var arr = [a + b, a * b];");
        let mut parser = AnaliseSintatica::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.generate(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("malloc"));
        assert!(ir.contains("add"));
        assert!(ir.contains("mul"));
    }
}
