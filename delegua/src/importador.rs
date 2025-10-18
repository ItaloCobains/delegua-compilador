use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use std::collections::HashMap;
use crate::error::CompilerError;

/// Trait que define a interface para módulos da stdlib
pub trait Modulo<'ctx> {
    /// Declara todas as funções do módulo no LLVM module
    fn declarar_funcoes(&self, module: &Module<'ctx>) -> HashMap<String, FunctionValue<'ctx>>;

    /// Gera as implementações de todas as funções do módulo
    fn gerar_implementacoes(&self, module: &Module<'ctx>);

    /// Declara uma função específica do módulo
    fn declarar_funcao_especifica(&self, nome: &str, module: &Module<'ctx>) -> Option<FunctionValue<'ctx>>;

    /// Gera a implementação de uma função específica
    fn gerar_implementacao_especifica(&self, nome: &str, module: &Module<'ctx>) -> bool;

    /// Lista todas as funções disponíveis no módulo
    fn listar_funcoes(&self) -> Vec<String>;
}

/// Registro para módulos da stdlib
pub struct RegistroDeModulos<'ctx> {
    context: &'ctx Context,
    registrados: HashMap<String, Box<dyn Fn(&'ctx Context) -> Box<dyn Modulo<'ctx> + 'ctx>>>,
}

impl<'ctx> RegistroDeModulos<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let mut registro = RegistroDeModulos {
            context,
            registrados: HashMap::new(),
        };

        registro.registrar_modulos_padrao();
        registro
    }

    /// Registra os módulos padrão da stdlib
    fn registrar_modulos_padrao(&mut self) {
        use crate::modules::matematica::Matematica;

        // Registra o módulo "matematica" -> struct Matematica
        self.registrar("matematica", |ctx| Box::new(Matematica::new(ctx)));
    }

    /// Registra um módulo customizado
    pub fn registrar<F>(&mut self, nome: &str, construtor: F)
    where
        F: Fn(&'ctx Context) -> Box<dyn Modulo<'ctx> + 'ctx> + 'static,
    {
        self.registrados.insert(nome.to_string(), Box::new(construtor));
    }

    /// Tenta resolver um módulo pelo nome
    pub fn resolver(&self, nome_modulo: &str) -> Result<Box<dyn Modulo<'ctx> + 'ctx>, CompilerError> {
        match self.registrados.get(nome_modulo) {
            Some(construtor) => Ok(construtor(self.context)),
            None => Err(CompilerError::CodeGen(
                format!("Módulo desconhecido: {}. Disponíveis: {}",
                    nome_modulo,
                    self.registrados.keys().map(|k| k.as_str()).collect::<Vec<_>>().join(", ")
                )
            )),
        }
    }
}

pub struct Importador<'ctx> {
    registro: RegistroDeModulos<'ctx>,
    cache_funcoes_importadas: HashMap<String, bool>,
}

impl<'ctx> Importador<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Importador {
            registro: RegistroDeModulos::new(context),
            cache_funcoes_importadas: HashMap::new(),
        }
    }

    /// Importa um módulo e retorna suas funções
    /// Suporta importação seletiva (apenas funções específicas) ou completa (todas)
    pub fn importar(
        &mut self,
        module: &Module<'ctx>,
        nome_modulo: &str,
        items: Option<&Vec<String>>,
    ) -> Result<Option<HashMap<String, FunctionValue<'ctx>>>, CompilerError> {
        // Tenta resolver o módulo da stdlib
        let modulo = self.registro.resolver(nome_modulo)?;

        let mut funcoes_importadas = HashMap::new();

        match items {
            // Importação seletiva: importar { absoluto, potencia } de "matematica"
            Some(lista_funcoes) => {
                for nome_funcao in lista_funcoes {
                    let chave_cache = format!("{}::{}", nome_modulo, nome_funcao);

                    // Verifica se a função já foi importada anteriormente
                    if self.cache_funcoes_importadas.contains_key(&chave_cache) {
                        continue;
                    }

                    // Declara a função específica
                    if let Some(func) = modulo.declarar_funcao_especifica(nome_funcao, module) {
                        // Gera a implementação específica
                        if modulo.gerar_implementacao_especifica(nome_funcao, module) {
                            funcoes_importadas.insert(nome_funcao.clone(), func);
                            self.cache_funcoes_importadas.insert(chave_cache, true);
                        }
                    } else {
                        return Err(CompilerError::CodeGen(
                            format!("Função '{}' não encontrada no módulo '{}'. Disponíveis: {}",
                                nome_funcao,
                                nome_modulo,
                                modulo.listar_funcoes().join(", ")
                            )
                        ));
                    }
                }
            }
            // Importação completa: importar * de "matematica"
            None => {
                let todas_funcoes = modulo.listar_funcoes();

                for nome_funcao in &todas_funcoes {
                    let chave_cache = format!("{}::{}", nome_modulo, nome_funcao);

                    // Pula se já foi importada
                    if self.cache_funcoes_importadas.contains_key(&chave_cache) {
                        continue;
                    }

                    if let Some(func) = modulo.declarar_funcao_especifica(nome_funcao, module) {
                        if modulo.gerar_implementacao_especifica(nome_funcao, module) {
                            funcoes_importadas.insert(nome_funcao.clone(), func);
                            self.cache_funcoes_importadas.insert(chave_cache, true);
                        }
                    }
                }
            }
        }

        if funcoes_importadas.is_empty() {
            Ok(None)
        } else {
            Ok(Some(funcoes_importadas))
        }
    }
}
