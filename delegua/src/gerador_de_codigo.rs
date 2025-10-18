use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, GlobalValue, IntValue, PointerValue};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, IntType, PointerType};
use inkwell::AddressSpace;
use std::collections::HashMap;
use crate::ast::{Programa, Declaracao, Espressao, OperacaoBinaria};
use crate::error::CompilerError;

#[derive(Clone, Copy)]
enum VariavelTipo<'ctx> {
    Numero(IntType<'ctx>),
    Texto(PointerType<'ctx>),
    Funcao(PointerType<'ctx>),
}

impl<'ctx> VariavelTipo<'ctx> {
    fn convert_para_basic_type_enum(self) -> BasicTypeEnum<'ctx> {
        match self {
            VariavelTipo::Numero(t) => t.into(),
            VariavelTipo::Texto(t) => t.into(),
            VariavelTipo::Funcao(t) => t.into(),
        }
    }
}

/// Estrutura principal para geração de código LLVM
/// Contém o contexto, módulo, construtor e tabelas de símbolos
/// para variáveis e funções
pub struct GeradorDeCodigo<'ctx> {
    /// Contexto LLVM
    contexto: &'ctx Context,
    /// Módulo LLVM
    modulo: Module<'ctx>,
    /// Construtor LLVM
    construtor: Builder<'ctx>,
    /// Tabela de símbolos para variáveis
    variaveis: HashMap<String, (PointerValue<'ctx>, VariavelTipo<'ctx>)>,
    /// Tabela de símbolos para funções
    funcoes: HashMap<String, FunctionValue<'ctx>>,
    /// Tabela de símbolos para funções nativas
    funcoes_nativas: HashMap<String, FunctionValue<'ctx>>,
    /// Tabela de módulos importados
    modulos: HashMap<String, HashMap<String, FunctionValue<'ctx>>>,
    /// Tipo inteiro de 64 bits
    i64_tipo: IntType<'ctx>,
    /// Tipo inteiro de 32 bits
    i32_tipo: IntType<'ctx>,
    /// Tipo Ponteiro
    i8_tipo: PointerType<'ctx>,
    /// Tabela de textos literais
    textos_literais: HashMap<String, GlobalValue<'ctx>>,
    /// Pilha de contextos de loop para controle de break/continue
    loop_pilha: Vec<LoopContexto<'ctx>>,
    /// Buffers alocados que precisam ser liberados
    buffers_para_liberar: Vec<PointerValue<'ctx>>,
    /// Buffers temporários de concatenação que podem ser liberados imediatamente
    buffers_concatenacao_temporarios: Vec<PointerValue<'ctx>>,
    /// Mapa de literais (globalvalue) para seus tamanhos conhecidos
    tamanhos_literais_globais: HashMap<GlobalValue<'ctx>, usize>,
}

/// Contexto de loop para controle de break/continue
/// Contém os blocos básicos para sustar e continuar o loop
#[derive(Debug, Clone, Copy)]
struct LoopContexto<'ctx> {
    /// Bloco básico para sustar o loop
    sustar_bloco: inkwell::basic_block::BasicBlock<'ctx>,
    /// Bloco básico para continuar o loop
    continua_bloco: inkwell::basic_block::BasicBlock<'ctx>,
}

impl<'ctx> GeradorDeCodigo<'ctx> {
    pub fn new(contexto: &'ctx Context, modulo_nome: &str) -> Result<Self, CompilerError> {
        let modulo = contexto.create_module(modulo_nome);
        let construtor = contexto.create_builder();

        let i64_tipo = contexto.i64_type();
        let i32_tipo = contexto.i32_type();
        let i8_ponteiro_tipo = contexto.ptr_type(inkwell::AddressSpace::default());

        let codegen = GeradorDeCodigo {
            contexto,
            modulo,
            construtor,
            variaveis: HashMap::with_capacity(32),
            funcoes: HashMap::with_capacity(16),
            funcoes_nativas: HashMap::with_capacity(8),
            modulos: HashMap::with_capacity(4),
            textos_literais: HashMap::with_capacity(8),
            loop_pilha: Vec::with_capacity(8),
            buffers_para_liberar: Vec::new(),
            buffers_concatenacao_temporarios: Vec::new(),
            tamanhos_literais_globais: HashMap::new(),
            i64_tipo,
            i32_tipo,
            i8_tipo: i8_ponteiro_tipo,
        };

        Ok(codegen)
    }

    #[inline]
    fn pega_funcoes_nativas(&mut self, nome: &str) -> Result<FunctionValue<'ctx>, CompilerError> {
        if let Some(&fn_val) = self.funcoes_nativas.get(nome) {
            return Ok(fn_val);
        }

        let adicionar_funcao = |fn_name: &str,
                                retorno: &dyn BasicType<'ctx>,
                                parametros: &[BasicMetadataTypeEnum<'ctx>],
                                is_var_args: bool| -> FunctionValue<'ctx> {
            let fn_tipo = retorno.fn_type(parametros, is_var_args);
            self.modulo.add_function(fn_name, fn_tipo, None)
        };

        let fn_val = match nome {
            "printf" => {
                let params = &[self.i8_tipo.into()];
                adicionar_funcao("printf", &self.i32_tipo, params, true)
            }
            "malloc" => {
                let params = &[self.i64_tipo.into()];
                adicionar_funcao("malloc", &self.i8_tipo, params, false)
            }
            "strlen" => {
                let params = &[self.i8_tipo.into()];
                adicionar_funcao("strlen", &self.i64_tipo, params, false)
            }
            "strcpy" | "strcat" => {
                let params = &[
                    self.i8_tipo.into(),
                    self.i8_tipo.into(),
                ];
                adicionar_funcao(nome, &self.i8_tipo, params, false)
            }
            "sprintf" => {
                let params = &[
                    self.i8_tipo.into(),
                    self.i8_tipo.into(),
                ];
                adicionar_funcao("sprintf", &self.i64_tipo, params, true)
            }
            "scanf" => {
                let params = &[self.i8_tipo.into()];
                adicionar_funcao("scanf", &self.i64_tipo, params, true)
            }
            "puts" => {
                let params = &[self.i8_tipo.into()];
                adicionar_funcao("puts", &self.i32_tipo, params, false)
            }
            "exit" => {
                let params = &[self.i32_tipo.into()];
                adicionar_funcao("exit", &self.i32_tipo, params, false)
            }
            "free" => {
                let params = &[self.i8_tipo.into()];
                adicionar_funcao("free", &self.i32_tipo, params, false)
            }
            _ => {
                return Err(CompilerError::CodeGen(format!(
                    "Função '{}' não encontrada. Disponíveis: printf, malloc, strlen, strcpy, strcat, sprintf, scanf, puts, exit, free",
                    nome
                )))
            }
        };

        self.funcoes_nativas.insert(nome.to_string(), fn_val);
        Ok(fn_val)
    }

    fn verificar_alocacao(&mut self, ponteiro: PointerValue<'ctx>) -> Result<(), CompilerError> {
        let funcao_atual = self.construtor.get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        let bloco_falhou = self.contexto.append_basic_block(funcao_atual, "alocacao_falhou");
        let bloco_sucesso = self.contexto.append_basic_block(funcao_atual, "alocacao_sucesso");

        let e_nulo = self.construtor.build_is_null(ponteiro, "e_nulo")
            .map_err(|e| CompilerError::CodeGen(format!("Erro ao verificar null: {:?}", e)))?;

        self.construtor.build_conditional_branch(e_nulo, bloco_falhou, bloco_sucesso)
            .map_err(|e| CompilerError::CodeGen(format!("Erro ao criar branch condicional: {:?}", e)))?;

        // Bloco de falha
        self.construtor.position_at_end(bloco_falhou);
        let exit_fn = self.pega_funcoes_nativas("exit")?;
        let codigo_erro = self.i32_tipo.const_int(1, false);
        self.construtor.build_call(exit_fn, &[codigo_erro.into()], "exit_call")
            .map_err(|e| CompilerError::CodeGen(format!("Erro ao chamar exit: {:?}", e)))?;
        self.construtor.build_unreachable()
            .map_err(|e| CompilerError::CodeGen(format!("Erro ao criar unreachable: {:?}", e)))?;

        // Continuar no bloco de sucesso
        self.construtor.position_at_end(bloco_sucesso);
        Ok(())
    }

    fn obter_tamanho_string(
        &mut self,
        ponteiro: PointerValue<'ctx>,
        nome: &str
    ) -> Result<IntValue<'ctx>, CompilerError> {
        // Tentar descobrir se o ponteiro vem de um GlobalValue
        // Verificar se é um GlobalValue consultando nosso mapa
        for (global, &tamanho) in &self.tamanhos_literais_globais {
            if global.as_pointer_value() == ponteiro {
                // Otimização: tamanho conhecido em tempo de compilação!
                return Ok(self.i64_tipo.const_int(tamanho as u64, false));
            }
        }

        // Fallback: chamar strlen dinamicamente
        let strlen_fn = self.pega_funcoes_nativas("strlen")?;
        let len_call = self.safe_build(
            self.construtor.build_call(strlen_fn, &[ponteiro.into()], nome),
            "strlen"
        )?;

        Ok(len_call.try_as_basic_value()
            .left()
            .ok_or_else(|| CompilerError::CodeGen("Falha ao obter tamanho da string".to_string()))?
            .into_int_value())
    }

    fn liberar_buffers_temporarios(&mut self) -> Result<(), CompilerError> {
        if self.buffers_concatenacao_temporarios.is_empty() {
            return Ok(());
        }

        let free_fn = self.pega_funcoes_nativas("free")?;
        for buffer in &self.buffers_concatenacao_temporarios {
            self.construtor.build_call(free_fn, &[(*buffer).into()], "liberar_temp")
                .map_err(|e| CompilerError::CodeGen(format!("Erro ao liberar buffer temporário: {:?}", e)))?;
        }

        self.buffers_concatenacao_temporarios.clear();
        Ok(())
    }

    pub fn gerar(&mut self, programa: &Programa) -> Result<(), CompilerError> {
        // main() retorna i32 por convenção (exit code)
        let i32_tipo = self.contexto.i32_type();
        let main_type = i32_tipo.fn_type(&[], false);
        let main_fn = self.modulo.add_function("main", main_type, None);
        let basic_block = self.contexto.append_basic_block(main_fn, "entry");
        self.construtor.position_at_end(basic_block);

        for statement in &programa.declaracoes {
            self.gera_declacao(statement)?;
        }

        // Liberar buffers alocados
        if !self.buffers_para_liberar.is_empty() {
            let free_fn = self.pega_funcoes_nativas("free")?;
            for buffer in &self.buffers_para_liberar {
                self.construtor.build_call(free_fn, &[(*buffer).into()], "liberar_buffer")
                    .map_err(|e| CompilerError::CodeGen(format!("Erro ao liberar buffer: {:?}", e)))?;
            }
        }

        let zero = i32_tipo.const_int(0, false);
        self.construtor.build_return(Some(&zero))
            .map_err(|e| CompilerError::CodeGen(format!("Error compiling return: {:?}", e)))?;

        Ok(())
    }

    fn gera_declacao(&mut self, statement: &Declaracao) -> Result<(), CompilerError> {
        match statement {
            Declaracao::Variavel { nome: name, valor: value } => {
                self.gera_declaracao_variavel(name, value)
            }
            Declaracao::Atribuicao { nome: name, valor: value } => {
                self.gera_atribuicao(name, value)
            }
            Declaracao::Importacao { modulo, itens } => {
                todo!("Implementar importação de módulos")
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
                self.gera_espressao(expr)?;
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

    fn gera_declaracao_variavel(&mut self, name: &str, value: &Espressao) -> Result<(), CompilerError> {
        let val = self.gera_espressao(value)?;

        let (alloca, var_type) = match val {
            BasicValueEnum::IntValue(int_val) => {
                let alloca = self.safe_build(
                    self.construtor.build_alloca(self.i64_tipo, name),
                    &format!("alloca para variável '{}'", name)
                )?;
                self.safe_build(
                    self.construtor.build_store(alloca, int_val),
                    &format!("armazenar na variável '{}'", name)
                )?;
                (alloca, VariavelTipo::Numero(self.i64_tipo))
            }
            BasicValueEnum::PointerValue(ptr_val) => {
                match value {
                    Espressao::Funcao { .. } => {
                        let func_ptr_type = ptr_val.get_type();
                        let alloca = self.safe_build(
                            self.construtor.build_alloca(func_ptr_type, name),
                            &format!("alloca para função '{}'", name)
                        )?;
                        self.safe_build(
                            self.construtor.build_store(alloca, ptr_val),
                            &format!("armazenar função em '{}'", name)
                        )?;
                        (alloca, VariavelTipo::Funcao(func_ptr_type))
                    }
                    _ => {
                        let alloca = self.safe_build(
                            self.construtor.build_alloca(self.i8_tipo, name),
                            &format!("alloca para texto '{}'", name)
                        )?;
                        self.safe_build(
                            self.construtor.build_store(alloca, ptr_val),
                            &format!("armazenar texto em '{}'", name)
                        )?;
                        (alloca, VariavelTipo::Texto(self.i8_tipo))
                    }
                }
            }
            _ => return Err(CompilerError::CodeGen(
                format!("Tipo de valor não suportado para variável '{}': {:?}", name, val.get_type())
            )),
        };
        
        self.variaveis.insert(name.to_string(), (alloca, var_type));
        Ok(())
    }

    fn gera_atribuicao(&mut self, nome: &str, valor: &Espressao) -> Result<(), CompilerError> {
        let val = self.gera_espressao(valor)?;

        if let Some(&(var_ptr, _)) = self.variaveis.get(nome) {
            match val {
                BasicValueEnum::IntValue(int_val) => {
                    self.construtor.build_store(var_ptr, int_val)
                        .map_err(|e| CompilerError::CodeGen(format!("Error ao armazenar valor inteiro: {:?}", e)))?;
                }
                BasicValueEnum::PointerValue(ptr_val) => {
                    self.construtor.build_store(var_ptr, ptr_val)
                        .map_err(|e| CompilerError::CodeGen(format!("Error ao armazenar ponteiro: {:?}", e)))?;
                }
                _ => return Err(CompilerError::CodeGen(format!("Tipo de valor não suportado para atribuição: {:?}", val.get_type()))),
            }
        } else {
            return Err(CompilerError::CodeGen(format!("Variável '{}' não declarada", nome)));
        }
        Ok(())
    }

    fn gera_espressao(&mut self, expr: &Espressao) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        match expr {
            Espressao::Numero(n) => {
                let val = self.i64_tipo.const_int(*n as u64, false);
                Ok(val.into())
            }

            Espressao::Texto(s) => {
                let texto_valor = self.contexto.const_string(s.as_bytes(), true);

                let global = self.modulo.add_global(texto_valor.get_type(), None, "texto_literal");
                global.set_initializer(&texto_valor);
                global.set_constant(true);
                global.set_unnamed_addr(true);

                // Registrar tamanho do literal para otimização de strlen
                let tamanho = s.len();
                self.tamanhos_literais_globais.insert(global, tamanho);

                // Com ponteiros opacos (LLVM 15+), não precisamos de GEP explícito
                // O LLVM entende automaticamente a conversão de ptr para ptr
                let ponteiro = global.as_pointer_value();
                Ok(ponteiro.into())
            }

            Espressao::Logico(b) => {
                let val = self.i64_tipo.const_int(if *b { 1 } else { 0 }, false);
                Ok(val.into())
            }

            Espressao::Identificador(name) => {
                if let Some(&(var_ptr, var_type)) = self.variaveis.get(name) {
                    let loaded = self.construtor.build_load(var_type.convert_para_basic_type_enum(), var_ptr, name)
                        .map_err(|e| CompilerError::CodeGen(format!("Error ao carregar variável: {:?}", e)))?;
                    Ok(loaded)
                } else {
                    Err(CompilerError::CodeGen(format!("Variável '{}' não encontrada", name)))
                }
            }

            Espressao::Binario { esquerda: left, operador: operator, direita: right } => {
                let left_val = self.gera_espressao(left)?;
                let right_val = self.gera_espressao(right)?;

                match (left_val, right_val) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        let (result, _op_name) = match operator {
                            OperacaoBinaria::Adicao => (self.safe_build(self.construtor.build_int_add(l, r, "add"), "integer addition")?, "add"),
                            OperacaoBinaria::Subtracao => (self.safe_build(self.construtor.build_int_sub(l, r, "sub"), "integer subtraction")?, "sub"),
                            OperacaoBinaria::Multiplicacao => (self.safe_build(self.construtor.build_int_mul(l, r, "mul"), "integer multiplication")?, "mul"),
                            OperacaoBinaria::Divisao => (self.safe_build(self.construtor.build_int_signed_div(l, r, "div"), "integer division")?, "div"),
                            OperacaoBinaria::Modulo => (self.safe_build(self.construtor.build_int_signed_rem(l, r, "mod"), "integer modulo")?, "mod"),
                            OperacaoBinaria::Potencia => (self.generate_integer_power(l, r)?, "pow"),
                            OperacaoBinaria::Igual => (self.safe_build(self.construtor.build_int_compare(inkwell::IntPredicate::EQ, l, r, "eq"), "equality comparison")?, "eq"),
                            OperacaoBinaria::NaoIgual => (self.safe_build(self.construtor.build_int_compare(inkwell::IntPredicate::NE, l, r, "ne"), "inequality comparison")?, "ne"),
                            OperacaoBinaria::Menor => (self.safe_build(self.construtor.build_int_compare(inkwell::IntPredicate::SLT, l, r, "lt"), "less than comparison")?, "lt"),
                            OperacaoBinaria::Maior => (self.safe_build(self.construtor.build_int_compare(inkwell::IntPredicate::SGT, l, r, "gt"), "greater than comparison")?, "gt"),
                            OperacaoBinaria::MenorIgual => (self.safe_build(self.construtor.build_int_compare(inkwell::IntPredicate::SLE, l, r, "le"), "less equal comparison")?, "le"),
                            OperacaoBinaria::MaiorIgual => (self.safe_build(self.construtor.build_int_compare(inkwell::IntPredicate::SGE, l, r, "ge"), "greater equal comparison")?, "ge"),
                            OperacaoBinaria::E => (self.safe_build(self.construtor.build_and(l, r, "and"), "logical and")?, "and"),
                            OperacaoBinaria::Ou => (self.safe_build(self.construtor.build_or(l, r, "or"), "logical or")?, "or"),
                            _ => return Err(CompilerError::CodeGen(format!("Operador binário não suportado para inteiros: {:?}", operator))),
                        };
                        Ok(result.into())
                    }

                    (BasicValueEnum::PointerValue(l), BasicValueEnum::PointerValue(r)) => {
                        if matches!(operator, OperacaoBinaria::Adicao) {
                            self.generate_string_concat(l, r)
                        } else {
                            Err(CompilerError::CodeGen("Apenas concatenação (+) é suportada para strings".to_string()))
                        }
                    }

                    (BasicValueEnum::PointerValue(l), BasicValueEnum::IntValue(r)) => {
                        if matches!(operator, OperacaoBinaria::Adicao) {
                            let r_str = self.int_to_string(r)?;
                            self.generate_string_concat(l, r_str)
                        } else {
                            Err(CompilerError::CodeGen("Apenas concatenação (+) é suportada entre string e número".to_string()))
                        }
                    }

                    _ => Err(CompilerError::CodeGen("Tipos de operandos não suportados para operação binária".to_string())),
                }
            }

            Espressao::Unario { operador: operator, operando: operand } => {
                let operand_val = self.gera_espressao(operand)?;
                match operand_val {
                    BasicValueEnum::IntValue(val) => {
                        let result = match operator {
                            OperacaoBinaria::Subtracao => self.construtor.build_int_neg(val, "neg")
                                .map_err(|e| CompilerError::CodeGen(format!("Erro ao construir operação unária: {:?}", e)))?,
                            OperacaoBinaria::Nao => self.construtor.build_not(val, "not")
                                .map_err(|e| CompilerError::CodeGen(format!("Erro ao construir operação lógica not: {:?}", e)))?,
                            _ => return Err(CompilerError::CodeGen("Operador unário não suportado".to_string())),
                        };
                        Ok(result.into())
                    }
                    _ => Err(CompilerError::CodeGen("Operações unárias são suportadas apenas para inteiros".to_string())),
                }
            }

            Espressao::ChamadaFuncao { chamado: callee, argumentos: args } => {
                match callee.as_ref() {
                    Espressao::Identificador(name) => {
                        match name.as_str() {
                            "escreva" => self.gera_escreva_chamada(args),
                            "texto" => self.gera_texto_chamada(args),
                            "leia" => self.generate_leia_call(args),
                            "comprimento" => self.generate_comprimento_call(args),
                            "maiuscula" => self.generate_maiuscula_call(args),
                            "minuscula" => self.generate_minuscula_call(args),
                            "absoluto" => self.generate_absoluto_call(args),
                            "raiz_quadrada" => self.generate_raiz_quadrada_call(args),
                            _ => {
                                if let Some(func) = self.funcoes.get(name) {
                                    let func = *func;
                                    self.gera_usuario_funcao_chamada(&func, args)
                                } else if name.contains('.') {
                                    self.gera_chamada_de_funcao_de_modulo(name, args)
                                } else if self.variaveis.contains_key(name) {
                                    if let Some(&(var_ptr, var_type)) = self.variaveis.get(name) {
                                        match var_type {
                                            VariavelTipo::Funcao(func_ptr_type) => {
                                                let param_types = vec![self.i64_tipo.into(); args.len()];
                                                let fn_type = self.i64_tipo.fn_type(&param_types, false);

                                                let func_ptr = self.construtor.build_load(func_ptr_type, var_ptr, &format!("load_func_{}", name))
                                                    .map_err(|e| CompilerError::CodeGen(format!("Erro ao carregar ponteiro da função: {:?}", e)))?;

                                                let mut arg_values = Vec::new();
                                                for arg in args {
                                                    arg_values.push(self.gera_espressao(arg)?.into());
                                                }

                                                let call = self.construtor.build_indirect_call(
                                                    fn_type,
                                                    func_ptr.into_pointer_value(),
                                                    &arg_values,
                                                    &format!("call_{}", name)
                                                ).map_err(|e| CompilerError::CodeGen(format!("Erro ao construir chamada indireta: {:?}", e)))?;

                                                Ok(call.try_as_basic_value().left().unwrap())
                                            }
                                            _ => Err(CompilerError::CodeGen(format!("Variável '{}' não é uma função", name))),
                                        }
                                    } else {
                                        Err(CompilerError::CodeGen(format!("Variável '{}' não encontrada", name)))
                                    }
                                } else {
                                    Err(CompilerError::CodeGen(format!("Função desconhecida: {}", name)))
                                }
                            }
                        }
                    }
                    _ => Err(CompilerError::CodeGen("Chamadas de função através de expressões ainda não suportadas".to_string())),
                }
            }
            Espressao::Funcao { paramentros: params, corpo: body } => {
                self.generate_anonymous_function(&params, &body)
            }
            
            Espressao::Incremento { operando: operand, prefixo: prefix } => {
                self.generate_increment_decrement(operand, true, *prefix)
            }
            
            Espressao::Decremento { operando: operand, prefixo: prefix } => {
                self.generate_increment_decrement(operand, false, *prefix)
            }

            Espressao::Lista { elementos: elements } => {
                self.generate_array_literal(elements)
            }

            Espressao::Indice { lista: array, indice: index } => {
                self.generate_array_index(array, index)
            }

            Espressao::Objeto { propriedades: properties } => {
                todo!("Implementar criação de objetos")
            }

            Espressao::PropriedadeAcesso { objeto: object, propriedade: property } => {
                todo!("Implementar acesso a propriedades de objetos")
            }
        }
    }

    fn gera_chamada_de_funcao_de_modulo(&mut self, nome: &str, argumentos: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let parts: Vec<&str> = nome.split('.').collect();
        if parts.len() != 2 {
            return Err(CompilerError::CodeGen(format!("Chamada de função de módulo inválida: {}", nome)));
        }

        let module_name = parts[0];
        let function_name = parts[1];

        let func = if let Some(module_functions) = self.modulos.get(module_name) {
            if let Some(func) = module_functions.get(function_name) {
                *func
            } else {
                return Err(CompilerError::CodeGen(format!("Função '{}' não encontrada no módulo '{}'", function_name, module_name)));
            }
        } else {
            return Err(CompilerError::CodeGen(format!("Módulo '{}' não encontrado", module_name)));
        };

        let mut arg_values = Vec::new();
        for arg in argumentos {
            let arg_val = self.gera_espressao(arg)?;
            arg_values.push(arg_val.into());
        }

        let call = self.construtor.build_call(func, &arg_values, "module_call")
            .map_err(|e| CompilerError::CodeGen(format!("Erro ao chamar função de módulo: {:?}", e)))?;

        if let Some(return_val) = call.try_as_basic_value().left() {
            Ok(return_val)
        } else {
            let zero = self.i64_tipo.const_int(0, false);
            Ok(zero.into())
        }
    }

    fn gera_usuario_funcao_chamada(&mut self, func: &FunctionValue<'ctx>, args: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.gera_espressao(arg)?.into());
        }

        let call = self.construtor.build_call(*func, &arg_values, "user_call")
            .map_err(|e| CompilerError::CodeGen(format!("Erro ao chamar função de usuário: {:?}", e)))?;

        if let Some(return_val) = call.try_as_basic_value().left() {
            Ok(return_val)
        } else {
            let zero = self.i64_tipo.const_int(0, false);
            Ok(zero.into())
        }
    }

    fn call_printf(
        &mut self,
        printf_fn: FunctionValue<'ctx>,
        formato: &str,
        arg: BasicValueEnum<'ctx>,
    ) -> Result<(), CompilerError> {
        let formato_global = self.get_or_create_format_string(formato);
        
        // https://llvm.org/docs/OpaquePointers.html
        // Com ponteiros opacos (LLVM 15+), podemos passar diretamente o ponteiro
        // Não precisamos de GEP explícito para conversão de tipo
        self.safe_build(
            self.construtor.build_call(
                printf_fn,
                &[formato_global.as_pointer_value().into(), arg.into()],
                "printf_call",
            ),
            "printf call",
        )?;

        Ok(())
    }

    fn gera_escreva_chamada(
        &mut self,
        argumentos: &[Espressao],
    ) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if argumentos.len() != 1 {
            return Err(CompilerError::CodeGen(
                "escreva() espera exatamente um argumento".to_string(),
            ));
        }

        let arg = self.gera_espressao(&argumentos[0])?;
        let printf_fn = self.pega_funcoes_nativas("printf")?;

        match arg {
            BasicValueEnum::PointerValue(_) => {
                self.call_printf(printf_fn, "%s\n\0", arg)?;
            }
            BasicValueEnum::IntValue(_) => {
                self.call_printf(printf_fn, "%lld\n\0", arg)?;
            }
            _ => {
                return Err(CompilerError::CodeGen(
                    "escreva() suporta apenas argumentos de string e inteiro".to_string(),
                ))
            }
        }

        // Liberar buffers temporários de concatenação após escrever
        self.liberar_buffers_temporarios()?;

        Ok(self.i64_tipo.const_int(0, false).into())
    }

    fn gera_texto_chamada(&mut self, argumentos: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if argumentos.len() != 1 {
            return Err(CompilerError::CodeGen("texto() espera exatamente um argumento".to_string()));
        }

        let arg = self.gera_espressao(&argumentos[0])?;

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

    fn generate_leia_call(&mut self, args: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let scanf_fn = self.pega_funcoes_nativas("scanf")?;
        let malloc_fn = self.pega_funcoes_nativas("malloc")?;

        if args.is_empty() {
            let buffer_size = self.i64_tipo.const_int(256, false);
            let buffer_call = self.safe_build(
                self.construtor.build_call(malloc_fn, &[buffer_size.into()], "input_buffer"),
                "malloc for input buffer"
            )?;
            let buffer = buffer_call.try_as_basic_value().left()
                .ok_or_else(|| CompilerError::CodeGen("Falha ao obter buffer do malloc".to_string()))?
                .into_pointer_value();

            // Verificar se a alocação foi bem-sucedida
            self.verificar_alocacao(buffer)?;

            let format_ptr = self.get_or_create_format_string("%255s\0");
            self.safe_build(
                self.construtor.build_call(
                    scanf_fn,
                    &[format_ptr.as_pointer_value().into(), buffer.into()],
                    "scanf_call"
                ),
                "scanf call"
            )?;

            // Registrar buffer para liberação automática
            self.buffers_para_liberar.push(buffer);

            Ok(buffer.into())
        } else if args.len() == 1 {
            let prompt = self.gera_espressao(&args[0])?;
            if let BasicValueEnum::PointerValue(prompt_ptr) = prompt {
                let puts_fn = self.pega_funcoes_nativas("puts")?;
                self.safe_build(
                    self.construtor.build_call(
                        puts_fn,
                        &[prompt_ptr.into()],
                        "puts_prompt"
                    ),
                    "puts prompt call"
                )?;
            }

            let buffer_size = self.i64_tipo.const_int(256, false);
            let buffer_call = self.safe_build(
                self.construtor.build_call(malloc_fn, &[buffer_size.into()], "input_buffer"),
                "malloc for input buffer"
            )?;
            let buffer = buffer_call.try_as_basic_value().left()
                .ok_or_else(|| CompilerError::CodeGen("Falha ao obter buffer do malloc".to_string()))?
                .into_pointer_value();

            // Verificar se a alocação foi bem-sucedida
            self.verificar_alocacao(buffer)?;

            let format_ptr = self.get_or_create_format_string("%255s\0");
            self.safe_build(
                self.construtor.build_call(
                    scanf_fn,
                    &[format_ptr.as_pointer_value().into(), buffer.into()],
                    "scanf_call"
                ),
                "scanf call"
            )?;

            // Registrar buffer para liberação automática
            self.buffers_para_liberar.push(buffer);

            Ok(buffer.into())
        } else {
            Err(CompilerError::CodeGen(
                "leia() expects 0 or 1 argument".to_string()
            ))
        }
    }

    fn generate_comprimento_call(&mut self, args: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "comprimento() expects exactly 1 argument".to_string()
            ));
        }

        let string_arg = self.gera_espressao(&args[0])?;

        let strlen_fn = self.pega_funcoes_nativas("strlen")?;

        let length_call = self.safe_build(
            self.construtor.build_call(strlen_fn, &[string_arg.into()], "strlen_call"),
            "call strlen function"
        )?;

        let length_value = length_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get length from strlen".to_string()))?;

        Ok(length_value)
    }

    fn generate_maiuscula_call(&mut self, args: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "maiuscula() expects exactly 1 argument".to_string()
            ));
        }

        let string_arg = self.gera_espressao(&args[0])?;

        let malloc_fn = self.pega_funcoes_nativas("malloc")?;
        let strlen_fn = self.pega_funcoes_nativas("strlen")?;
        let strcpy_fn = self.pega_funcoes_nativas("strcpy")?;

        let length_call = self.safe_build(
            self.construtor.build_call(strlen_fn, &[string_arg.into()], "strlen_call"),
            "call strlen for maiuscula"
        )?;
        let length = length_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get length".to_string()))?
            .into_int_value();

        let one = self.i64_tipo.const_int(1, false);
        let buffer_size = self.safe_build(
            self.construtor.build_int_add(length, one, "buffer_size"),
            "calculate buffer size"
        )?;

        let buffer_call = self.safe_build(
            self.construtor.build_call(malloc_fn, &[buffer_size.into()], "upper_buffer"),
            "malloc for uppercase buffer"
        )?;
        let buffer = buffer_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to allocate buffer".to_string()))?
            .into_pointer_value();

        self.safe_build(
            self.construtor.build_call(strcpy_fn, &[buffer.into(), string_arg.into()], "strcpy_call"),
            "copy string to buffer"
        )?;

        let current_function = self.construtor.get_insert_block().unwrap().get_parent().unwrap();
        let loop_bb = self.contexto.append_basic_block(current_function, "upper_loop");
        let loop_body_bb = self.contexto.append_basic_block(current_function, "upper_body");
        let after_loop_bb = self.contexto.append_basic_block(current_function, "upper_done");

        let counter = self.safe_build(
            self.construtor.build_alloca(self.i64_tipo, "upper_counter"),
            "allocate counter"
        )?;
        let zero = self.i64_tipo.const_int(0, false);
        self.safe_build(
            self.construtor.build_store(counter, zero),
            "initialize counter"
        )?;

        self.safe_build(
            self.construtor.build_unconditional_branch(loop_bb),
            "branch to loop"
        )?;

        self.construtor.position_at_end(loop_bb);
        let current_counter = self.safe_build(
            self.construtor.build_load(self.i64_tipo, counter, "load_counter"),
            "load counter"
        )?.into_int_value();

        let is_end = self.safe_build(
            self.construtor.build_int_compare(inkwell::IntPredicate::EQ, current_counter, length, "is_end"),
            "check if end reached"
        )?;

        self.safe_build(
            self.construtor.build_conditional_branch(is_end, after_loop_bb, loop_body_bb),
            "conditional branch"
        )?;

        self.construtor.position_at_end(loop_body_bb);

        let char_ptr = unsafe {
            self.safe_build(
                self.construtor.build_gep(self.contexto.i8_type(), buffer, &[current_counter], "char_ptr"),
                "get character pointer"
            )?
        };

        let char_val = self.safe_build(
            self.construtor.build_load(self.contexto.i8_type(), char_ptr, "char_val"),
            "load character"
        )?.into_int_value();

        let char_as_i64 = self.safe_build(
            self.construtor.build_int_z_extend(char_val, self.i64_tipo, "char_ext"),
            "extend character to i64"
        )?;

        let a_val = self.i64_tipo.const_int(97, false);  // 'a'
        let z_val = self.i64_tipo.const_int(122, false); // 'z'

        let is_ge_a = self.safe_build(
            self.construtor.build_int_compare(inkwell::IntPredicate::SGE, char_as_i64, a_val, "is_ge_a"),
            "check if >= 'a'"
        )?;

        let is_le_z = self.safe_build(
            self.construtor.build_int_compare(inkwell::IntPredicate::SLE, char_as_i64, z_val, "is_le_z"),
            "check if <= 'z'"
        )?;

        let is_lowercase = self.safe_build(
            self.construtor.build_and(is_ge_a, is_le_z, "is_lowercase"),
            "check if lowercase"
        )?;

        let diff = self.i64_tipo.const_int(32, false);
        let upper_char = self.safe_build(
            self.construtor.build_int_sub(char_as_i64, diff, "upper_char"),
            "convert to uppercase"
        )?;

        let final_char = self.safe_build(
            self.construtor.build_select(is_lowercase, upper_char, char_as_i64, "final_char"),
            "select final character"
        )?;

        let final_char_i8 = self.safe_build(
            self.construtor.build_int_truncate(final_char.into_int_value(), self.contexto.i8_type(), "final_char_i8"),
            "truncate to i8"
        )?;

        self.safe_build(
            self.construtor.build_store(char_ptr, final_char_i8),
            "store uppercase character"
        )?;

        let incremented = self.safe_build(
            self.construtor.build_int_add(current_counter, one, "increment"),
            "increment counter"
        )?;

        self.safe_build(
            self.construtor.build_store(counter, incremented),
            "store incremented counter"
        )?;

        self.safe_build(
            self.construtor.build_unconditional_branch(loop_bb),
            "branch back to loop"
        )?;

        self.construtor.position_at_end(after_loop_bb);
        Ok(buffer.into())
    }

    fn generate_minuscula_call(&mut self, args: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "minuscula() expects exactly 1 argument".to_string()
            ));
        }

        let string_arg = self.gera_espressao(&args[0])?;

        let malloc_fn = self.pega_funcoes_nativas("malloc")?;
        let strlen_fn = self.pega_funcoes_nativas("strlen")?;
        let strcpy_fn = self.pega_funcoes_nativas("strcpy")?;

        let length_call = self.safe_build(
            self.construtor.build_call(strlen_fn, &[string_arg.into()], "strlen_call"),
            "call strlen for minuscula"
        )?;
        let length = length_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get length".to_string()))?
            .into_int_value();

        let one = self.i64_tipo.const_int(1, false);
        let buffer_size = self.safe_build(
            self.construtor.build_int_add(length, one, "buffer_size"),
            "calculate buffer size"
        )?;

        let buffer_call = self.safe_build(
            self.construtor.build_call(malloc_fn, &[buffer_size.into()], "lower_buffer"),
            "malloc for lowercase buffer"
        )?;
        let buffer = buffer_call.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to allocate buffer".to_string()))?
            .into_pointer_value();

        self.safe_build(
            self.construtor.build_call(strcpy_fn, &[buffer.into(), string_arg.into()], "strcpy_call"),
            "copy string to buffer"
        )?;

        let current_function = self.construtor.get_insert_block().unwrap().get_parent().unwrap();
        let loop_bb = self.contexto.append_basic_block(current_function, "lower_loop");
        let loop_body_bb = self.contexto.append_basic_block(current_function, "lower_body");
        let after_loop_bb = self.contexto.append_basic_block(current_function, "lower_done");

        let counter = self.safe_build(
            self.construtor.build_alloca(self.i64_tipo, "lower_counter"),
            "allocate counter"
        )?;
        let zero = self.i64_tipo.const_int(0, false);
        self.safe_build(
            self.construtor.build_store(counter, zero),
            "initialize counter"
        )?;

        self.safe_build(
            self.construtor.build_unconditional_branch(loop_bb),
            "branch to loop"
        )?;

        self.construtor.position_at_end(loop_bb);
        let current_counter = self.safe_build(
            self.construtor.build_load(self.i64_tipo, counter, "load_counter"),
            "load counter"
        )?.into_int_value();

        let is_end = self.safe_build(
            self.construtor.build_int_compare(inkwell::IntPredicate::EQ, current_counter, length, "is_end"),
            "check if end reached"
        )?;

        self.safe_build(
            self.construtor.build_conditional_branch(is_end, after_loop_bb, loop_body_bb),
            "conditional branch"
        )?;

        self.construtor.position_at_end(loop_body_bb);

        let char_ptr = unsafe {
            self.safe_build(
                self.construtor.build_gep(self.contexto.i8_type(), buffer, &[current_counter], "char_ptr"),
                "get character pointer"
            )?
        };

        let char_val = self.safe_build(
            self.construtor.build_load(self.contexto.i8_type(), char_ptr, "char_val"),
            "load character"
        )?.into_int_value();

        let char_as_i64 = self.safe_build(
            self.construtor.build_int_z_extend(char_val, self.i64_tipo, "char_ext"),
            "extend character to i64"
        )?;

        let a_val = self.i64_tipo.const_int(65, false);  // 'A'
        let z_val = self.i64_tipo.const_int(90, false);  // 'Z'

        let is_ge_a = self.safe_build(
            self.construtor.build_int_compare(inkwell::IntPredicate::SGE, char_as_i64, a_val, "is_ge_A"),
            "check if >= 'A'"
        )?;

        let is_le_z = self.safe_build(
            self.construtor.build_int_compare(inkwell::IntPredicate::SLE, char_as_i64, z_val, "is_le_Z"),
            "check if <= 'Z'"
        )?;

        let is_uppercase = self.safe_build(
            self.construtor.build_and(is_ge_a, is_le_z, "is_uppercase"),
            "check if uppercase"
        )?;

        let diff = self.i64_tipo.const_int(32, false);
        let lower_char = self.safe_build(
            self.construtor.build_int_add(char_as_i64, diff, "lower_char"),
            "convert to lowercase"
        )?;

        let final_char = self.safe_build(
            self.construtor.build_select(is_uppercase, lower_char, char_as_i64, "final_char"),
            "select final character"
        )?;

        let final_char_i8 = self.safe_build(
            self.construtor.build_int_truncate(final_char.into_int_value(), self.contexto.i8_type(), "final_char_i8"),
            "truncate to i8"
        )?;

        self.safe_build(
            self.construtor.build_store(char_ptr, final_char_i8),
            "store lowercase character"
        )?;

        let incremented = self.safe_build(
            self.construtor.build_int_add(current_counter, one, "increment"),
            "increment counter"
        )?;

        self.safe_build(
            self.construtor.build_store(counter, incremented),
            "store incremented counter"
        )?;

        self.safe_build(
            self.construtor.build_unconditional_branch(loop_bb),
            "branch back to loop"
        )?;

        self.construtor.position_at_end(after_loop_bb);
        Ok(buffer.into())
    }

    fn generate_absoluto_call(&mut self, args: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "absoluto() expects exactly 1 argument".to_string()
            ));
        }

        let arg_value = self.gera_espressao(&args[0])?;

        let absoluto_fn = self.modulo.get_function("matematica_absoluto")
            .ok_or_else(|| CompilerError::CodeGen(
                "Função 'absoluto' do módulo 'matematica' não foi importada. Use: importar { absoluto } de \"matematica\";".to_string()
            ))?;

        let call_result = self.safe_build(
            self.construtor.build_call(absoluto_fn, &[arg_value.into()], "absoluto_call"),
            "call absoluto function"
        )?;

        call_result.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get result from absoluto".to_string()))
    }

    fn generate_raiz_quadrada_call(&mut self, args: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if args.len() != 1 {
            return Err(CompilerError::CodeGen(
                "raiz_quadrada() expects exactly 1 argument".to_string()
            ));
        }

        let arg_value = self.gera_espressao(&args[0])?;

        let raiz_fn = self.modulo.get_function("matematica_raiz_quadrada")
            .ok_or_else(|| CompilerError::CodeGen("matematica_raiz_quadrada function not found".to_string()))?;

        let call_result = self.safe_build(
            self.construtor.build_call(raiz_fn, &[arg_value.into()], "raiz_quadrada_call"),
            "call raiz_quadrada function"
        )?;

        call_result.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to get result from raiz_quadrada".to_string()))
    }

    fn int_to_string(&mut self, int_val: IntValue<'ctx>) -> Result<PointerValue<'ctx>, CompilerError> {
        let malloc_fn = self.pega_funcoes_nativas("malloc")?;
        let sprintf_fn = self.pega_funcoes_nativas("sprintf")?;
        
        let buffer_size = self.i64_tipo.const_int(20, false);
        let buffer_call = self.safe_build(
            self.construtor.build_call(malloc_fn, &[buffer_size.into()], "int_str_buffer"),
            "malloc integer to string buffer"
        )?;
        let buffer = buffer_call.try_as_basic_value()
            .left()
            .ok_or_else(|| CompilerError::CodeGen("Failed to allocate integer string buffer".to_string()))?
            .into_pointer_value();

        let format_ptr = self.get_or_create_format_string("%lld\0");

        self.safe_build(
            self.construtor.build_call(
                sprintf_fn,
                &[buffer.into(), format_ptr.as_pointer_value().into(), int_val.into()],
                "sprintf_int"
            ),
            "sprintf integer conversion"
        )?;

        Ok(buffer)
    }

    fn generate_string_concat(&mut self, left: PointerValue<'ctx>, right: PointerValue<'ctx>) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let malloc_fn = self.pega_funcoes_nativas("malloc")?;
        let strcpy_fn = self.pega_funcoes_nativas("strcpy")?;
        let strcat_fn = self.pega_funcoes_nativas("strcat")?;

        // Usar método otimizado para obter tamanhos (usa constante para literais)
        let left_len = self.obter_tamanho_string(left, "left_len")?;
        let right_len = self.obter_tamanho_string(right, "right_len")?;

        let total_len = self.safe_build(
            self.construtor.build_int_add(left_len, right_len, "total_len"),
            "somar tamanhos das strings"
        )?;
        let total_len_plus_one = self.safe_build(
            self.construtor.build_int_add(
                total_len,
                self.i64_tipo.const_int(1, false),
                "total_len_plus_one"
            ),
            "adicionar espaço para terminador nulo"
        )?;

        let result_buffer_call = self.safe_build(
            self.construtor.build_call(malloc_fn, &[total_len_plus_one.into()], "concat_buffer"),
            "malloc buffer de concatenação"
        )?;
        let result_buffer = result_buffer_call.try_as_basic_value()
            .left()
            .ok_or_else(|| CompilerError::CodeGen("Falha ao alocar buffer de concatenação".to_string()))?
            .into_pointer_value();

        // Verificar se a alocação foi bem-sucedida
        self.verificar_alocacao(result_buffer)?;

        // Registrar buffer temporário para liberação posterior
        self.buffers_concatenacao_temporarios.push(result_buffer);

        self.safe_build(
            self.construtor.build_call(strcpy_fn, &[result_buffer.into(), left.into()], "strcpy_first"),
            "copiar primeira string"
        )?;
        self.safe_build(
            self.construtor.build_call(strcat_fn, &[result_buffer.into(), right.into()], "strcat_second"),
            "concatenar segunda string"
        )?;

        Ok(result_buffer.into())
    }

    pub fn has_variable(&self, name: &str) -> bool {
        self.variaveis.contains_key(name)
    }

    pub fn print_ir(&self) {
        println!("{}", self.modulo.print_to_string().to_string());
    }

    pub fn get_ir(&self) -> String {
        self.modulo.print_to_string().to_string()
    }
    
    
    fn get_or_create_format_string(&mut self, format: &str) -> GlobalValue<'ctx> {
        if let Some(&ptr) = self.textos_literais.get(format) {
            return ptr;
        }
        
        let format_str = self.contexto.const_string(format.as_bytes(), false);
        let global = self.modulo.add_global(format_str.get_type(), None, "formato_texto");
        global.set_initializer(&format_str);
        global.set_constant(true);
        global.set_unnamed_addr(true);
        
        self.textos_literais.insert(format.to_string(), global);
        global
    }

    #[inline]
    fn value_to_bool(&mut self, val: BasicValueEnum<'ctx>, name: &str) -> Result<inkwell::values::IntValue<'ctx>, CompilerError> {
        match val {
            BasicValueEnum::IntValue(int_val) => {
                if int_val.get_type().get_bit_width() == 1 {
                    Ok(int_val)
                } else {
                    let zero = self.i64_tipo.const_int(0, false);
                    self.construtor.build_int_compare(
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
        
        let current_function = self.construtor.get_insert_block()
            .ok_or_else(|| CompilerError::CodeGen("No current basic block for power operation".to_string()))?
            .get_parent()
            .ok_or_else(|| CompilerError::CodeGen("No current function for power operation".to_string()))?;
            
        let _entry_bb = self.construtor.get_insert_block().unwrap();
        let loop_bb = self.contexto.append_basic_block(current_function, "pow_loop");
        let exit_bb = self.contexto.append_basic_block(current_function, "pow_exit");
        
        let result_ptr = self.safe_build(
            self.construtor.build_alloca(self.i64_tipo, "pow_result"),
            "alloca pow result"
        )?;
        let counter_ptr = self.safe_build(
            self.construtor.build_alloca(self.i64_tipo, "pow_counter"), 
            "alloca pow counter"
        )?;
        
        let one = self.i64_tipo.const_int(1, false);
        let zero = self.i64_tipo.const_int(0, false);
        
        self.safe_build(
            self.construtor.build_store(result_ptr, one),
            "store initial pow result"
        )?;
        self.safe_build(
            self.construtor.build_store(counter_ptr, exp),
            "store initial pow counter"
        )?;
        
        self.safe_build(
            self.construtor.build_unconditional_branch(loop_bb),
            "jump to pow loop"
        )?;
        
        self.construtor.position_at_end(loop_bb);
        let counter_val = self.safe_build(
            self.construtor.build_load(self.i64_tipo, counter_ptr, "load_counter"),
            "load pow counter"
        )?.into_int_value();
        
        let is_positive = self.safe_build(
            self.construtor.build_int_compare(inkwell::IntPredicate::SGT, counter_val, zero, "counter_positive"),
            "compare counter with zero"
        )?;
        
        let multiply_bb = self.contexto.append_basic_block(current_function, "pow_multiply");
        self.safe_build(
            self.construtor.build_conditional_branch(is_positive, multiply_bb, exit_bb),
            "pow loop condition"
        )?;
        
        self.construtor.position_at_end(multiply_bb);
        let current_result = self.safe_build(
            self.construtor.build_load(self.i64_tipo, result_ptr, "load_result"),
            "load pow result"
        )?.into_int_value();
        
        let new_result = self.safe_build(
            self.construtor.build_int_mul(current_result, base, "pow_multiply"),
            "multiply in pow loop"
        )?;
        let decremented_counter = self.safe_build(
            self.construtor.build_int_sub(counter_val, one, "decrement_counter"),
            "decrement pow counter"
        )?;
        
        self.safe_build(
            self.construtor.build_store(result_ptr, new_result),
            "store updated pow result"
        )?;
        self.safe_build(
            self.construtor.build_store(counter_ptr, decremented_counter),
            "store updated pow counter"
        )?;
        
        self.safe_build(
            self.construtor.build_unconditional_branch(loop_bb),
            "jump back to pow loop"
        )?;
        
        self.construtor.position_at_end(exit_bb);
        let final_result = self.safe_build(
            self.construtor.build_load(self.i64_tipo, result_ptr, "final_pow_result"),
            "load final pow result"
        )?.into_int_value();
        
        Ok(final_result)
    }
    
    fn generate_increment_decrement(&mut self, operand: &Espressao, is_increment: bool, prefix: bool) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        if let Espressao::Identificador(name) = operand {
            if let Some(&(var_ptr, var_type)) = self.variaveis.get(name) {
                match var_type {
                    VariavelTipo::Numero(_) => {
                        let current_val = self.safe_build(
                            self.construtor.build_load(self.i64_tipo, var_ptr, &format!("load_{}", name)),
                            &format!("load variable {} for increment/decrement", name)
                        )?.into_int_value();
                        
                        let one = self.i64_tipo.const_int(1, false);
                        let new_val = if is_increment {
                            self.safe_build(
                                self.construtor.build_int_add(current_val, one, &format!("inc_{}", name)),
                                &format!("increment variable {}", name)
                            )?
                        } else {
                            self.safe_build(
                                self.construtor.build_int_sub(current_val, one, &format!("dec_{}", name)),
                                &format!("decrement variable {}", name)
                            )?
                        };
                        
                        self.safe_build(
                            self.construtor.build_store(var_ptr, new_val),
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

    fn generate_array_literal(&mut self, elements: &[Espressao]) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let array_size = elements.len();
        let size_value = self.i64_tipo.const_int(array_size as u64, false);

        let total_size = self.i64_tipo.const_int(((array_size + 1) * 8) as u64, false);

        let malloc_fn = self.pega_funcoes_nativas("malloc")?;

        let array_ptr = self.safe_build(
            self.construtor.build_call(malloc_fn, &[total_size.into()], "array_alloc"),
            "allocate array memory"
        )?;

        let array_ptr = array_ptr.try_as_basic_value().left()
            .ok_or_else(|| CompilerError::CodeGen("malloc call failed".to_string()))?
            .into_pointer_value();

        let array_ptr = self.safe_build(
            self.construtor.build_pointer_cast(array_ptr, self.contexto.ptr_type(AddressSpace::default()), "array_cast"),
            "cast array pointer"
        )?;

        self.safe_build(
            self.construtor.build_store(array_ptr, size_value),
            "store array size"
        )?;

        for (i, element) in elements.iter().enumerate() {
            let element_val = self.gera_espressao(element)?;
            let element_int = self.value_to_int(element_val, &format!("array_element_{}", i))?;

            let index = self.i64_tipo.const_int((i + 1) as u64, false);
            let element_ptr = unsafe {
                self.safe_build(
                    self.construtor.build_gep(self.i64_tipo, array_ptr, &[index], &format!("array_elem_ptr_{}", i)),
                    &format!("get pointer to array element {}", i)
                )?
            };

            self.safe_build(
                self.construtor.build_store(element_ptr, element_int),
                &format!("store array element {}", i)
            )?;
        }

        Ok(array_ptr.into())
    }

    fn generate_array_index(&mut self, array: &Espressao, index: &Espressao) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let array_val = self.gera_espressao(array)?;
        let index_val = self.gera_espressao(index)?;

        let array_ptr = array_val.into_pointer_value();
        let index_int = self.value_to_int(index_val, "array_index")?;

        let one = self.i64_tipo.const_int(1, false);
        let adjusted_index = self.safe_build(
            self.construtor.build_int_add(index_int, one, "adjusted_index"),
            "adjust array index"
        )?;

        let element_ptr = unsafe {
            self.safe_build(
                self.construtor.build_gep(self.i64_tipo, array_ptr, &[adjusted_index], "array_element_ptr"),
                "get array element pointer"
            )?
        };

        let element_val = self.safe_build(
            self.construtor.build_load(self.i64_tipo, element_ptr, "array_element"),
            "load array element"
        )?;

        Ok(element_val)
    }

    fn generate_if_statement(&mut self, condition: &Espressao, then_branch: &[Declaracao], else_branch: Option<&Vec<Declaracao>>) -> Result<(), CompilerError> {
        let condition_val = self.gera_espressao(condition)?;
        let condition_bool = self.value_to_bool(condition_val, "if_condition")?;

        let current_function = self.construtor.get_insert_block().unwrap().get_parent().unwrap();
        
        let then_bb = self.contexto.append_basic_block(current_function, "then");
        let merge_bb = self.contexto.append_basic_block(current_function, "merge");

        let else_bb = if else_branch.is_some() {
            Some(self.contexto.append_basic_block(current_function, "else"))
        } else {
            None
        };

        self.safe_build(
            self.construtor.build_conditional_branch(condition_bool, then_bb, else_bb.unwrap_or(merge_bb)),
            "if conditional branch"
        )?;

        self.construtor.position_at_end(then_bb);
        for stmt in then_branch {
            self.gera_declacao(stmt)?;
        }
        self.safe_build(
            self.construtor.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;

        if let Some(else_stmts) = else_branch {
            if let Some(else_block) = else_bb {
                self.construtor.position_at_end(else_block);
                for stmt in else_stmts {
                    self.gera_declacao(stmt)?;
                }
                self.safe_build(
            self.construtor.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;
            }
        }

        self.construtor.position_at_end(merge_bb);
        Ok(())
    }

    fn generate_if_else_if_statement(&mut self, condition: &Espressao, then_branch: &[Declaracao], else_if_branches: &[(Espressao, Vec<Declaracao>)], else_branch: Option<&Vec<Declaracao>>) -> Result<(), CompilerError> {
        let current_function = self.construtor.get_insert_block().unwrap().get_parent().unwrap();

        let condition_val = self.gera_espressao(condition)?;
        let condition_bool = match condition_val {
            BasicValueEnum::IntValue(val) => {
                if val.get_type().get_bit_width() == 1 {
                    val
                } else {
                    let zero = self.i64_tipo.const_int(0, false);
                    self.construtor.build_int_compare(
                        inkwell::IntPredicate::NE,
                        val,
                        zero,
                        "condition_bool"
                    ).unwrap()
                }
            }
            _ => return Err(CompilerError::CodeGen("If condition must be integer".to_string())),
        };

        let then_bb = self.contexto.append_basic_block(current_function, "then");
        let current_else_bb = self.contexto.append_basic_block(current_function, "else_if_start");
        let merge_bb = self.contexto.append_basic_block(current_function, "merge");

        self.construtor.build_conditional_branch(condition_bool, then_bb, current_else_bb).unwrap();

        self.construtor.position_at_end(then_bb);
        for stmt in then_branch {
            self.gera_declacao(stmt)?;
        }
        self.safe_build(
            self.construtor.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;

        let mut previous_else_bb = current_else_bb;
        for (i, (else_if_condition, else_if_statements)) in else_if_branches.iter().enumerate() {
            self.construtor.position_at_end(previous_else_bb);

            let else_if_condition_val = self.gera_espressao(else_if_condition)?;
            let else_if_condition_bool = match else_if_condition_val {
                BasicValueEnum::IntValue(val) => {
                    if val.get_type().get_bit_width() == 1 {
                        val
                    } else {
                        let zero = self.i64_tipo.const_int(0, false);
                        self.construtor.build_int_compare(
                            inkwell::IntPredicate::NE,
                            val,
                            zero,
                            &format!("else_if_condition_bool_{}", i)
                        ).unwrap()
                    }
                }
                _ => return Err(CompilerError::CodeGen("Else-if condition must be integer".to_string())),
            };

            let else_if_then_bb = self.contexto.append_basic_block(current_function, &format!("else_if_then_{}", i));
            let next_else_bb = if i < else_if_branches.len() - 1 {
                self.contexto.append_basic_block(current_function, &format!("else_if_{}", i + 1))
            } else if else_branch.is_some() {
                self.contexto.append_basic_block(current_function, "final_else")
            } else {
                merge_bb
            };

            self.construtor.build_conditional_branch(else_if_condition_bool, else_if_then_bb, next_else_bb).unwrap();

            self.construtor.position_at_end(else_if_then_bb);
            for stmt in else_if_statements {
                self.gera_declacao(stmt)?;
            }
            self.safe_build(
            self.construtor.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;

            previous_else_bb = next_else_bb;
        }

        if let Some(else_stmts) = else_branch {
            self.construtor.position_at_end(previous_else_bb);
            for stmt in else_stmts {
                self.gera_declacao(stmt)?;
            }
            self.safe_build(
            self.construtor.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;
        } else if !else_if_branches.is_empty() {
            self.construtor.position_at_end(previous_else_bb);
            self.safe_build(
            self.construtor.build_unconditional_branch(merge_bb),
            "unconditional branch"
        )?;
        }

        self.construtor.position_at_end(merge_bb);
        Ok(())
    }

    fn generate_switch_statement(&mut self, _value: &Espressao, cases: &[(Espressao, Vec<Declaracao>)], default: Option<&Vec<Declaracao>>) -> Result<(), CompilerError> {
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

    fn generate_while_statement(&mut self, condition: &Espressao, body: &[Declaracao]) -> Result<(), CompilerError> {
        let current_function = self.construtor.get_insert_block().unwrap().get_parent().unwrap();
        
        let loop_bb = self.contexto.append_basic_block(current_function, "loop");
        let body_bb = self.contexto.append_basic_block(current_function, "loop_body");
        let after_bb = self.contexto.append_basic_block(current_function, "after_loop");
        
        self.construtor.build_unconditional_branch(loop_bb).unwrap();
        
        self.construtor.position_at_end(loop_bb);
        let condition_val = self.gera_espressao(condition)?;
        let condition_bool = match condition_val {
            BasicValueEnum::IntValue(val) => {
                if val.get_type().get_bit_width() == 1 {
                    val
                } else {
                    let zero = self.i64_tipo.const_int(0, false);
                    self.construtor.build_int_compare(
                        inkwell::IntPredicate::NE,
                        val,
                        zero,
                        "loop_condition"
                    ).unwrap()
                }
            }
            _ => return Err(CompilerError::CodeGen("While condition must be integer".to_string())),
        };
        
        self.construtor.build_conditional_branch(condition_bool, body_bb, after_bb).unwrap();
        
        self.construtor.position_at_end(body_bb);
        for stmt in body {
            self.gera_declacao(stmt)?;
        }
        self.construtor.build_unconditional_branch(loop_bb).unwrap();
        
        self.construtor.position_at_end(after_bb);
        Ok(())
    }

    fn generate_do_while_statement(&mut self, body: &[Declaracao], condition: &Espressao) -> Result<(), CompilerError> {
        let current_function = self.construtor.get_insert_block().unwrap().get_parent().unwrap();
        
        let body_bb = self.contexto.append_basic_block(current_function, "do_body");
        let condition_bb = self.contexto.append_basic_block(current_function, "do_condition");
        let after_bb = self.contexto.append_basic_block(current_function, "after_do");
        
        self.construtor.build_unconditional_branch(body_bb).unwrap();
        
        self.construtor.position_at_end(body_bb);
        for stmt in body {
            self.gera_declacao(stmt)?;
        }
        self.construtor.build_unconditional_branch(condition_bb).unwrap();
        
        self.construtor.position_at_end(condition_bb);
        let condition_val = self.gera_espressao(condition)?;
        let condition_bool = match condition_val {
            BasicValueEnum::IntValue(val) => {
                if val.get_type().get_bit_width() == 1 {
                    val
                } else {
                    let zero = self.i64_tipo.const_int(0, false);
                    self.construtor.build_int_compare(
                        inkwell::IntPredicate::NE,
                        val,
                        zero,
                        "do_condition"
                    ).unwrap()
                }
            }
            _ => return Err(CompilerError::CodeGen("Do-while condition must be integer".to_string())),
        };
        
        self.construtor.build_conditional_branch(condition_bool, body_bb, after_bb).unwrap();
        
        self.construtor.position_at_end(after_bb);
        Ok(())
    }

    fn generate_for_statement(&mut self, initializer: Option<&Declaracao>, condition: Option<&Espressao>, increment: Option<&Espressao>, body: &[Declaracao]) -> Result<(), CompilerError> {
        let current_function = self.construtor.get_insert_block().unwrap().get_parent().unwrap();
        
        let init_bb = self.contexto.append_basic_block(current_function, "for_init");
        let condition_bb = self.contexto.append_basic_block(current_function, "for_condition");
        let body_bb = self.contexto.append_basic_block(current_function, "for_body");
        let increment_bb = self.contexto.append_basic_block(current_function, "for_increment");
        let after_bb = self.contexto.append_basic_block(current_function, "after_for");
        
        self.construtor.build_unconditional_branch(init_bb).unwrap();
        
        self.construtor.position_at_end(init_bb);
        if let Some(init_stmt) = initializer {
            self.gera_declacao(init_stmt)?;
        }
        self.construtor.build_unconditional_branch(condition_bb).unwrap();
        
        self.construtor.position_at_end(condition_bb);
        let condition_bool = if let Some(cond_expr) = condition {
            let condition_val = self.gera_espressao(cond_expr)?;
            match condition_val {
                BasicValueEnum::IntValue(val) => {
                    if val.get_type().get_bit_width() == 1 {
                        val
                    } else {
                        let zero = self.i64_tipo.const_int(0, false);
                        self.construtor.build_int_compare(
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
            self.contexto.bool_type().const_int(1, false)
        };
        
        self.construtor.build_conditional_branch(condition_bool, body_bb, after_bb).unwrap();
        
        self.construtor.position_at_end(body_bb);
        for stmt in body {
            self.gera_declacao(stmt)?;
        }
        self.construtor.build_unconditional_branch(increment_bb).unwrap();
        
        self.construtor.position_at_end(increment_bb);
        if let Some(inc_expr) = increment {
            self.gera_espressao(inc_expr)?;
        }
        self.construtor.build_unconditional_branch(condition_bb).unwrap();
        
        self.construtor.position_at_end(after_bb);
        Ok(())
    }

    fn generate_for_each_statement(&mut self, _variable: &str, _iterable: &Espressao, _body: &[Declaracao]) -> Result<(), CompilerError> {
        Ok(())
    }

    fn generate_break_statement(&mut self) -> Result<(), CompilerError> {
        if let Some(loop_ctx) = self.loop_pilha.last() {
            self.safe_build(
                self.construtor.build_unconditional_branch(loop_ctx.sustar_bloco),
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
        if let Some(loop_ctx) = self.loop_pilha.last() {
            self.safe_build(
                self.construtor.build_unconditional_branch(loop_ctx.continua_bloco),
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
            let param_types = vec![self.i64_tipo.into(); params.len()];
            let fn_type = self.i64_tipo.fn_type(&param_types, false);

            let func = self.modulo.add_function(name, fn_type, None);

            self.funcoes.insert(name.clone(), func);

            let current_block = self.construtor.get_insert_block();
            let saved_variables = self.variaveis.clone();

            let entry_block = self.contexto.append_basic_block(func, "entry");
            self.construtor.position_at_end(entry_block);

            self.variaveis.clear();

            for (i, param) in params.iter().enumerate() {
                let param_value = func.get_nth_param(i as u32).unwrap().into_int_value();
                let param_ptr = self.construtor.build_alloca(self.i64_tipo, param)
                    .map_err(|e| CompilerError::CodeGen(format!("Error allocating parameter: {:?}", e)))?;
                self.construtor.build_store(param_ptr, param_value)
                    .map_err(|e| CompilerError::CodeGen(format!("Error storing parameter: {:?}", e)))?;
                self.variaveis.insert(param.clone(), (param_ptr, VariavelTipo::Numero(self.i64_tipo)));
            }

            for stmt in body {
                self.gera_declacao(stmt)?;
            }

            if !body.iter().any(|stmt| matches!(stmt, Declaracao::Retorna(_))) {
                self.construtor.build_return(Some(&self.i64_tipo.const_int(0, false)))
                    .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;
            }

            self.variaveis = saved_variables;
            if let Some(block) = current_block {
                self.construtor.position_at_end(block);
            }
        }

        Ok(())
    }

    fn generate_return_statement(&mut self, value: Option<&Espressao>) -> Result<(), CompilerError> {
        if let Some(expr) = value {
            let val = self.gera_espressao(expr)?;
            match val {
                BasicValueEnum::IntValue(int_val) => {
                    self.construtor.build_return(Some(&int_val))
                        .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;
                }
                _ => return Err(CompilerError::CodeGen("Return value must be an integer".to_string())),
            }
        } else {
            self.construtor.build_return(Some(&self.i64_tipo.const_int(0, false)))
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

        let param_types = vec![self.i64_tipo.into(); params.len()];
        let fn_type = self.i64_tipo.fn_type(&param_types, false);

        let func = self.modulo.add_function(&func_name, fn_type, None);

        let current_block = self.construtor.get_insert_block();
        let saved_variables = self.variaveis.clone();

        let entry_block = self.contexto.append_basic_block(func, "entry");
        self.construtor.position_at_end(entry_block);

        self.variaveis.clear();

        for (i, param) in params.iter().enumerate() {
            let param_value = func.get_nth_param(i as u32).unwrap().into_int_value();
            let param_ptr = self.construtor.build_alloca(self.i64_tipo, param)
                .map_err(|e| CompilerError::CodeGen(format!("Error allocating parameter: {:?}", e)))?;
            self.construtor.build_store(param_ptr, param_value)
                .map_err(|e| CompilerError::CodeGen(format!("Error storing parameter: {:?}", e)))?;
            self.variaveis.insert(param.clone(), (param_ptr, VariavelTipo::Numero(self.i64_tipo)));
        }

        for stmt in body {
            self.gera_declacao(stmt)?;
        }

        if !body.iter().any(|stmt| matches!(stmt, Declaracao::Retorna(_))) {
            self.construtor.build_return(Some(&self.i64_tipo.const_int(0, false)))
                .map_err(|e| CompilerError::CodeGen(format!("Error building return: {:?}", e)))?;
        }

        self.variaveis = saved_variables;
        if let Some(block) = current_block {
            self.construtor.position_at_end(block);
        }

        Ok(func.as_global_value().as_pointer_value().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexador::Lexador;
    use crate::analise_sintatica::AvaliadorSintatico;

    #[test]
    fn test_simple_variable() {
        let context = Context::create();
        let mut codegen = GeradorDeCodigo::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var a = 42;");
        let mut parser = AvaliadorSintatico::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.gerar(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("alloca"));
        assert!(ir.contains("store"));
    }

    #[test]
    fn test_arithmetic() {
        let context = Context::create();
        let mut codegen = GeradorDeCodigo::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var a = 10; var b = 5; var c = 2; var result = a + b * c;");
        let mut parser = AvaliadorSintatico::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.gerar(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("mul"));
        assert!(ir.contains("add"));
    }

    #[test]
    fn test_string_literal() {
        let context = Context::create();
        let mut codegen = GeradorDeCodigo::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var msg = \"Hello World\";");
        let mut parser = AvaliadorSintatico::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.gerar(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("Hello World"));
    }

    #[test]
    fn test_complete_program() {
        let context = Context::create();
        let mut codegen = GeradorDeCodigo::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let code = r#"
            var a = 10;
            var b = 5;
            var soma = a + b;
            escreva("Resultado: " + texto(soma));
        "#;
        let tokens = lexer.analisar(code);
        let mut parser = AvaliadorSintatico::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.gerar(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("main"));
        assert!(ir.contains("printf"));
        assert!(ir.contains("add"));
        assert!(ir.contains("Resultado:"));
    }

    #[test]
    fn test_array_literal() {
        let context = Context::create();
        let mut codegen = GeradorDeCodigo::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var arr = [1, 2, 3];");
        let mut parser = AvaliadorSintatico::new(tokens);
        let program = parser.analisar().unwrap();

        let result = codegen.gerar(&program);
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
        let mut codegen = GeradorDeCodigo::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var arr = [10, 20, 30]; var x = arr[1];");
        let mut parser = AvaliadorSintatico::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.gerar(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("malloc"));
        assert!(ir.contains("array_element"));
    }

    #[test]
    fn test_empty_array() {
        let context = Context::create();
        let mut codegen = GeradorDeCodigo::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var arr = [];");
        let mut parser = AvaliadorSintatico::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.gerar(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("malloc"));
    }

    #[test]
    fn test_array_with_expressions() {
        let context = Context::create();
        let mut codegen = GeradorDeCodigo::new(&context, "test").unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var a = 5; var b = 10; var arr = [a + b, a * b];");
        let mut parser = AvaliadorSintatico::new(tokens);
        let program = parser.analisar().unwrap();

        assert!(codegen.gerar(&program).is_ok());

        let ir = codegen.get_ir();
        assert!(ir.contains("malloc"));
        assert!(ir.contains("add"));
        assert!(ir.contains("mul"));
    }
}
