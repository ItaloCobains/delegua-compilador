use crate::simbolo::Simbolo;
use crate::ast::{Espressao, OperacaoBinaria, Declaracao, Programa};
use crate::error::CompilerError;
use crate::simbolo::Posicao;

/// Avaliador Sintático para a linguagem de programação Delegua.
/// Recebe uma lista de tokens e produz uma árvore de sintaxe abstrata (AST).
/// Implementa um analisador recursivo descendente com tratamento de erros.
pub struct AvaliadorSintatico<'a> {
    /// Lista de tokens a serem analisados.
    simbolos: Vec<Simbolo<'a>>,
    /// Índice do token atual na lista.
    atual: usize,
    /// Token EOF para facilitar a verificação de fim de arquivo.
    eof_simbolo: Simbolo<'a>,
}

impl<'a> AvaliadorSintatico<'a> {
    /// Gera uma nova instância do Avaliador Sintático com a lista de tokens fornecida.
    /// # Argumentos
    /// * `simbolos` - Vetor de tokens a serem analisados.
    /// # Retorna
    /// Nova instância do Avaliador Sintático.
    pub fn new(simbolos: Vec<Simbolo<'a>>) -> Self {
        AvaliadorSintatico { 
            simbolos, 
            atual: 0,
            eof_simbolo: Simbolo::EOF(Posicao::default()),
        }
    }

    fn simbolo_com_posicao(simbolo_tipo: fn(Posicao) -> Simbolo<'a>) -> Simbolo<'a> {
        simbolo_tipo(Posicao { linha: 0, coluna: 0, deslocamento: 0 })
    }

    fn identificador_simbolo(s: &'a str) -> Simbolo<'a> {
        Simbolo::Identificador(s, Posicao { linha: 0, coluna: 0, deslocamento: 0 })
    }

    pub fn analisar(&mut self) -> Result<Programa, CompilerError> {
        let mut declaracoes = Vec::new();
        let mut errors = Vec::new();

        while !self.esta_no_fim() {
            match self.resolve_declaracao() {
                Ok(declaracao) => declaracoes.push(declaracao),
                Err(err) => {
                    errors.push(err.clone());
                    if errors.len() > 10 {
                        return Err(CompilerError::AvaliadorSintatico(
                            format!("Muitos erros de análise ({}). Primeiro erro: {}", 
                                   errors.len(), errors[0])
                        ));
                    }
                    self.sincronizar();
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors.into_iter().next().unwrap());
        }

        Ok(Programa { declaracoes })
    }

    fn resolve_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        match self.simbolo_atual() {
            Simbolo::Variavel(_) => self.resolve_declaracao_variavel(),
            Simbolo::Importacao(_) => self.resolve_declaracao_importacao(),
            Simbolo::Se(_) => self.resolve_se_declaracao(),
            Simbolo::Escolha(_) => self.resolve_escolha_declaracao(),
            Simbolo::Enquanto(_) => self.resolve_enquanto_declaracao(),
            Simbolo::Fazer(_) => self.resolve_faca_enquanto_declaracao(),
            Simbolo::Para(_) => self.resolve_para_declaracao(),
            Simbolo::ParaCada(_) => self.resolve_para_cada_declaracao(),
            Simbolo::Sustar(_) => self.resolve_sustar_declaracao(),
            Simbolo::Continua(_) => self.resolve_continua_declaracao(),
            Simbolo::Funcao(_) => {
                self.avancar();
                if let Simbolo::Identificador(_, _) = self.simbolo_atual() {
                    self.resolve_declaracao_funcao()
                } else {
                    Err(CompilerError::AvaliadorSintatico("Inesperado 'funcao' no contexto da declaração".to_string()))
                }
            }
            Simbolo::Retorna(_) => self.resolve_retorna_declaracao(),
            Simbolo::Identificador(_, _) => self.resolve_atribuicao_ou_chamada(),
            Simbolo::Incremento(_) => self.resolve_prefixo_incremento_decremento(true),
            Simbolo::Decremento(_) => self.resolve_prefixo_incremento_decremento(false),
            _ => Err(CompilerError::AvaliadorSintatico(format!(
                "Token inesperado: {:?}", self.simbolo_atual()
            ))),
        }
    }

    fn resolve_declaracao_variavel(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Variavel), "Esperado 'var'")?;

        let nome = self.consumir_identificador("Esperado nome da variável")?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::Atribuir), "Esperado '=' após o nome da variável")?;
        let inicializador = self.resolve_espressao()?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após a declaração da variável")?;

        Ok(Declaracao::Variavel { nome, valor: inicializador })
    }

    fn resolve_atribuicao_ou_chamada(&mut self) -> Result<Declaracao, CompilerError> {
        let nome = self.consumir_identificador("Esperado identificador")?;

        if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::Atribuir)) {
            let valor = self.resolve_espressao()?;
            self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após a atribuição")?;
            Ok(Declaracao::Atribuicao { nome, valor })
        } else if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
            let args = self.resolve_argumentos()?;
            self.consumir(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Esperado ')' após os argumentos da função")?;
            let _ = self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após a chamada da função");
            Ok(Declaracao::ChamadaDeFuncao(Espressao::ChamadaFuncao {
                chamado: Box::new(Espressao::Identificador(nome)),
                argumentos: args,
            }))
        } else if matches!(self.simbolo_atual(), Simbolo::Incremento(_)) {
            self.avancar(); // consume ++
            self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após o incremento")?;
            Ok(Declaracao::ChamadaDeFuncao(Espressao::Incremento {
                operando: Box::new(Espressao::Identificador(nome)),
                prefixo: false, // postfix: x++
            }))
        } else if matches!(self.simbolo_atual(), Simbolo::Decremento(_)) {
            self.avancar(); // consume --
            self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após o decremento")?;
            Ok(Declaracao::ChamadaDeFuncao(Espressao::Decremento {
                operando: Box::new(Espressao::Identificador(nome)),
                prefixo: false, // postfix: x--
            }))
        } else {
            Err(CompilerError::AvaliadorSintatico(
                "Esperado '=', '(', '++', ou '--' após o identificador".to_string()
            ))
        }
    }
    
    fn resolve_prefixo_incremento_decremento(&mut self, is_increment: bool) -> Result<Declaracao, CompilerError> {
        self.avancar(); // consume ++ or --
        
        let nome = match self.simbolo_atual() {
            Simbolo::Identificador(nome, _) => nome.to_string(),
            _ => return Err(CompilerError::AvaliadorSintatico("Esperado identificador após ++ ou --".to_string())),
        };
        self.avancar();

        self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após o incremento/decremento")?;

        if is_increment {
            Ok(Declaracao::ChamadaDeFuncao(Espressao::Incremento {
                operando: Box::new(Espressao::Identificador(nome)),
                prefixo: true, // prefix: ++x
            }))
        } else {
            Ok(Declaracao::ChamadaDeFuncao(Espressao::Decremento {
                operando: Box::new(Espressao::Identificador(nome)),
                prefixo: true, // prefix: --x
            }))
        }
    }

    fn resolve_declaracao_importacao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Importacao), "Esperado palavra-chave 'importar'")?;

        if let Simbolo::Texto(module, _) = self.simbolo_atual() {
            let module = module.to_string();
            self.avancar();
            self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após a importação")?;
            Ok(Declaracao::Importacao { modulo: module, itens: None })
        } else if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda)) {
            let mut items = Vec::new();
            while !self.compara(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
                if let Simbolo::Identificador(name, _) = self.simbolo_atual() {
                    let name = name.to_string();
                    items.push(name);
                    self.avancar();
                } else {
                    return Err(CompilerError::AvaliadorSintatico("Esperado identificador na lista de importação".to_string()));
                }
                if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::Virgula)) {
                    // continue
                } else if !self.compara(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) {
                    return Err(CompilerError::AvaliadorSintatico("Esperado ',' ou '}' na lista de importação".to_string()));
                }
            }
            self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveDireita), "Esperado '}' após a lista de importação")?;
            self.consumir(Self::identificador_simbolo("from"), "Esperado 'from' após a lista de importação")?;
            if let Simbolo::Texto(module, _) = self.simbolo_atual() {
                let module = module.to_string();
                self.avancar();
                self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após a importação")?;
                Ok(Declaracao::Importacao { modulo: module, itens: Some(items) })
            } else {
                Err(CompilerError::AvaliadorSintatico("Esperado nome do módulo após 'from'".to_string()))
            }
        } else {
            Err(CompilerError::AvaliadorSintatico("Esperado string ou '{' após 'importar'".to_string()))
        }
    }

    fn resolve_argumentos(&mut self) -> Result<Vec<Espressao>, CompilerError> {
        if matches!(self.simbolo_atual(), Simbolo::ParenteseDireito(_)) {
            return Ok(Vec::new());
        }

        let mut args = Vec::with_capacity(4);
        
        loop {
            args.push(self.resolve_espressao()?);
            
            if matches!(self.simbolo_atual(), Simbolo::Virgula(_)) {
                self.avancar();
            } else {
                break;
            }
        }

        Ok(args)
    }

    fn resolve_parametros(&mut self) -> Result<Vec<String>, CompilerError> {
        if matches!(self.simbolo_atual(), Simbolo::ParenteseDireito(_)) {
            return Ok(Vec::new());
        }

        let mut params = Vec::with_capacity(4);
        
        loop {
            match self.simbolo_atual() {
                Simbolo::Identificador(param, _) => {
                    params.push(param.to_string());
                    self.avancar();
                }
                _ => {
                    let pos = self.posicao_atual();
                    return Err(CompilerError::AvaliadorSintatico(
                        format!("Esperado nome do parâmetro na linha {}, coluna {}", 
                               pos.linha, pos.coluna)
                    ));
                }
            }

            if matches!(self.simbolo_atual(), Simbolo::Virgula(_)) {
                self.avancar();
            } else {
                break;
            }
        }
        
        Ok(params)
    }

    fn resolve_espressao(&mut self) -> Result<Espressao, CompilerError> {
        self.resolve_logico_ou()
    }

    fn resolve_logico_ou(&mut self) -> Result<Espressao, CompilerError> {
        let mut left = self.resolve_logico_e()?;

        while matches!(self.simbolo_atual(), Simbolo::Ou(_)) {
            self.avancar();
            let right = self.resolve_logico_e()?;
            left = Espressao::Binario {
                esquerda: Box::new(left),
                operador: OperacaoBinaria::Ou,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }

    fn resolve_logico_e(&mut self) -> Result<Espressao, CompilerError> {
        let mut left = self.resolve_comparacao()?;

        while matches!(self.simbolo_atual(), Simbolo::E(_)) {
            self.avancar();
            let right = self.resolve_comparacao()?;
            left = Espressao::Binario {
                esquerda: Box::new(left),
                operador: OperacaoBinaria::E,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }

    fn resolve_comparacao(&mut self) -> Result<Espressao, CompilerError> {
        let mut left = self.resolve_aditiva()?;

        while matches!(self.simbolo_atual(), 
            Simbolo::Menor(_) | Simbolo::Maior(_) | Simbolo::MenorIgual(_) | 
            Simbolo::MaiorIgual(_) | Simbolo::Igual(_) | Simbolo::NaoIgual(_)
        ) {
            let operator = match self.simbolo_atual() {
                Simbolo::Menor(_) => OperacaoBinaria::Menor,
                Simbolo::Maior(_) => OperacaoBinaria::Maior,
                Simbolo::MenorIgual(_) => OperacaoBinaria::MenorIgual,
                Simbolo::MaiorIgual(_) => OperacaoBinaria::MaiorIgual,
                Simbolo::Igual(_) => OperacaoBinaria::Igual,
                Simbolo::NaoIgual(_) => OperacaoBinaria::NaoIgual,
                _ => unreachable!(),
            };
            self.avancar();

            let right = self.resolve_aditiva()?;
            left = Espressao::Binario {
                esquerda: Box::new(left),
                operador: operator,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }

    fn resolve_aditiva(&mut self) -> Result<Espressao, CompilerError> {
        let mut left = self.resolve_multiplacativa()?;

        while matches!(self.simbolo_atual(), Simbolo::Adicao(_) | Simbolo::Subtracao(_)) {
            let operator = match self.simbolo_atual() {
                Simbolo::Adicao(_) => OperacaoBinaria::Adicao,
                Simbolo::Subtracao(_) => OperacaoBinaria::Subtracao,
                _ => unreachable!(),
            };
            self.avancar();

            let right = self.resolve_multiplacativa()?;
            left = Espressao::Binario {
                esquerda: Box::new(left),
                operador: operator,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }

    fn resolve_multiplacativa(&mut self) -> Result<Espressao, CompilerError> {
        let mut left = self.resolve_potencia()?;

        while matches!(self.simbolo_atual(), Simbolo::Multiplicacao(_) | Simbolo::Divisao(_) | Simbolo::Modulo(_)) {
            let operator = match self.simbolo_atual() {
                Simbolo::Multiplicacao(_) => OperacaoBinaria::Multiplicacao,
                Simbolo::Divisao(_) => OperacaoBinaria::Divisao,
                Simbolo::Modulo(_) => OperacaoBinaria::Modulo,
                _ => unreachable!(),
            };
            self.avancar();

            let right = self.resolve_potencia()?;
            left = Espressao::Binario {
                esquerda: Box::new(left),
                operador: operator,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }
    
    fn resolve_potencia(&mut self) -> Result<Espressao, CompilerError> {
        let mut left = self.resolve_unario()?;
        
        // Right associative: 2**3**2 = 2**(3**2) = 512
        if matches!(self.simbolo_atual(), Simbolo::Potencia(_)) {
            self.avancar();
            let right = self.resolve_potencia()?; // Right associative recursion
            left = Espressao::Binario {
                esquerda: Box::new(left),
                operador: OperacaoBinaria::Potencia,
                direita: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn resolve_unario(&mut self) -> Result<Espressao, CompilerError> {
        match self.simbolo_atual() {
            Simbolo::Adicao(_) => {
                self.avancar();
                self.resolve_unario()
            }
            Simbolo::Subtracao(_) => {
                self.avancar();
                let operand = self.resolve_unario()?;
                Ok(Espressao::Unario {
                    operador: OperacaoBinaria::Subtracao,
                    operando: Box::new(operand),
                })
            }
            Simbolo::Nao(_) => {
                self.avancar();
                let operand = self.resolve_unario()?;
                Ok(Espressao::Unario {
                    operador: OperacaoBinaria::Nao,
                    operando: Box::new(operand),
                })
            }
            Simbolo::Incremento(_) => {
                self.avancar();
                let operand = self.resolve_pos_fixado()?;
                Ok(Espressao::Incremento {
                    operando: Box::new(operand),
                    prefixo: true,
                })
            }
            Simbolo::Decremento(_) => {
                self.avancar();
                let operand = self.resolve_pos_fixado()?;
                Ok(Espressao::Decremento {
                    operando: Box::new(operand),
                    prefixo: true,
                })
            }
            _ => self.resolve_pos_fixado()
        }
    }
    
    fn resolve_pos_fixado(&mut self) -> Result<Espressao, CompilerError> {
        let mut expr = self.resolve_primario()?;
        
        loop {
            match self.simbolo_atual() {
                Simbolo::Incremento(_) => {
                    self.avancar();
                    expr = Espressao::Incremento {
                        operando: Box::new(expr),
                        prefixo: false,
                    };
                }
                Simbolo::Decremento(_) => {
                    self.avancar();
                    expr = Espressao::Decremento {
                        operando: Box::new(expr),
                        prefixo: false,
                    };
                }
                Simbolo::ColcheteEsquerdo(_) => {
                    self.avancar();
                    let index = self.resolve_espressao()?;
                    self.consumir(Self::simbolo_com_posicao(Simbolo::ColcheteDireito), "Expected ']' after array index")?;
                    expr = Espressao::Indice {
                        lista: Box::new(expr),
                        indice: Box::new(index),
                    };
                }
                Simbolo::Ponto(_) => {
                    self.avancar();
                    if let Simbolo::Identificador(name, _) = self.simbolo_atual() {
                        let property = name.to_string();
                        self.avancar();
                        expr = Espressao::PropriedadeAcesso {
                            objeto: Box::new(expr),
                            propriedade: property,
                        };
                    } else {
                        return Err(CompilerError::AvaliadorSintatico("Expected property name after '.'".to_string()));
                    }
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }

    fn resolve_primario(&mut self) -> Result<Espressao, CompilerError> {
        match *self.simbolo_atual() {
            Simbolo::Number(n, _) => {
                self.avancar();
                Ok(Espressao::Numero(n))
            }
            Simbolo::Texto(ref s, _) => {
                let s = s.to_string();
                self.avancar();
                Ok(Espressao::Texto(s))
            }
            Simbolo::Verdadeiro(_) => {
                self.avancar();
                Ok(Espressao::Logico(true))
            }
            Simbolo::Falso(_) => {
                self.avancar();
                Ok(Espressao::Logico(false))
            }
            Simbolo::Identificador(name, _) => {
                let name = name.to_string();
                self.avancar();

                if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.resolve_argumentos()?;
                    self.consumir(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Esperado ')' após argumentos da função")?;
                    Ok(Espressao::ChamadaFuncao {
                        chamado: Box::new(Espressao::Identificador(name)),
                        argumentos: args,
                    })
                } else {
                    Ok(Espressao::Identificador(name))
                }
            }
            Simbolo::Funcao(_) => {
                self.avancar();
                self.consumir(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo), "Esperado '(' após 'funcao'")?;
                let params = self.resolve_parametros()?;
                self.consumir(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Esperado ')' após parâmetros")?;
                self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após assinatura da função")?;
                let body = self.resolve_bloco()?;
                Ok(Espressao::Funcao { paramentros: params, corpo: body })
            }
            Simbolo::ParenteseEsquerdo(_) => {
                self.avancar();
                let expr = self.resolve_espressao()?;
                self.consumir(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Esperado ')' após expressão")?;
                Ok(expr)
            }
            Simbolo::ColcheteEsquerdo(_) => {
                self.avancar();
                let mut elements = Vec::new();

                if !matches!(self.simbolo_atual(), Simbolo::ColcheteDireito(_)) {
                    loop {
                        elements.push(self.resolve_espressao()?);

                        if matches!(self.simbolo_atual(), Simbolo::Virgula(_)) {
                            self.avancar();
                        } else {
                            break;
                        }
                    }
                }

                self.consumir(Self::simbolo_com_posicao(Simbolo::ColcheteDireito), "Esperado ']' após elementos da lista")?;
                Ok(Espressao::Lista { elementos: elements })
            }
            Simbolo::ChaveEsquerda(_) => {
                self.avancar();
                let mut properties = Vec::new();

                if !matches!(self.simbolo_atual(), Simbolo::ChaveDireita(_)) {
                    loop {
                        let key = match self.simbolo_atual() {
                            Simbolo::Identificador(name, _) => {
                                let key = name.to_string();
                                self.avancar();
                                key
                            }
                            Simbolo::Texto(name, _) => {
                                let key = name.to_string();
                                self.avancar();
                                key
                            }
                            _ => return Err(CompilerError::AvaliadorSintatico("Esperado nome da propriedade no literal de objeto".to_string())),
                        };

                        self.consumir(Self::simbolo_com_posicao(Simbolo::DoisPontos), "Esperado ':' após o nome da propriedade")?;
                        let value = self.resolve_espressao()?;
                        properties.push((key, value));

                        if matches!(self.simbolo_atual(), Simbolo::Virgula(_)) {
                            self.avancar();
                        } else {
                            break;
                        }
                    }
                }

                self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveDireita), "Esperado '}' após propriedades do objeto")?;
                Ok(Espressao::Objeto { propriedades: properties })
            }
            _ => Err(CompilerError::AvaliadorSintatico(format!(
                "Token inesperado na expressão: {:?}", self.simbolo_atual()
            ))),
        }
    }


    #[inline]
    fn simbolo_atual(&self) -> &Simbolo<'a> {
        self.simbolos.get(self.atual).unwrap_or(&self.eof_simbolo)
    }

    #[inline]
    fn simbolo_anterior(&self) -> &Simbolo<'a> {
        if self.atual > 0 {
            self.simbolos.get(self.atual - 1).unwrap_or(&self.eof_simbolo)
        } else {
            &self.eof_simbolo
        }
    }

    #[inline]
    fn avancar(&mut self) -> &Simbolo<'a> {
        if !self.esta_no_fim() {
            self.atual += 1;
        }
        self.simbolo_anterior()
    }

    fn consumir(&mut self, expected: Simbolo, message: &str) -> Result<String, CompilerError> {
        if self.compara(expected.clone()) {
            let result = match self.simbolo_atual() {
                Simbolo::Identificador(name, _) => name.to_string(),
                Simbolo::Texto(s, _) => s.to_string(),
                _ => String::new(),
            };
            self.avancar();
            Ok(result)
        } else {
            let pos = self.posicao_atual();
            Err(CompilerError::AvaliadorSintatico(
                format!("{} at line {}, column {}. Found: {:?}", 
                       message, pos.linha, pos.coluna, self.simbolo_atual())
            ))
        }
    }

    fn token_to_identifier_name(&self, token: &Simbolo) -> Option<String> {
        match token {
            Simbolo::Identificador(name, _) => Some(name.to_string()),
            _ => None,
        }
    }

    fn consumir_identificador(&mut self, message: &str) -> Result<String, CompilerError> {
        if let Some(name) = self.token_to_identifier_name(&self.simbolo_atual()) {
            self.avancar();
            Ok(name)
        } else {
            Err(CompilerError::AvaliadorSintatico(message.to_string()))
        }
    }



    fn compara_simbolos(&mut self, expected: Simbolo) -> bool {
        if self.compara(expected) {
            self.avancar();
            true
        } else {
            false
        }
    }


    #[inline]
    fn compara(&self, expected: Simbolo) -> bool {
        !self.esta_no_fim() && self.compara_simbolo_atual(&expected)
    }
    
    #[inline]
    fn compara_simbolo_atual(&self, expected: &Simbolo) -> bool {
        match (self.simbolo_atual(), expected) {
            (Simbolo::Texto(_, _), Simbolo::Texto(_, _)) => true,
            (Simbolo::Identificador(_, _), Simbolo::Identificador(_, _)) => true,
            (Simbolo::Number(_, _), Simbolo::Number(_, _)) => true,
            _ => std::mem::discriminant(self.simbolo_atual()) == std::mem::discriminant(expected),
        }
    }

    fn esta_no_fim(&self) -> bool {
        matches!(self.simbolo_atual(), Simbolo::EOF(_))
    }

    fn posicao_atual(&self) -> Posicao {
        match self.simbolo_atual() {
            Simbolo::Number(_, pos) | Simbolo::Texto(_, pos) | Simbolo::Identificador(_, pos) |
            Simbolo::Variavel(pos) | Simbolo::Importacao(pos) |
            Simbolo::Se(pos) | Simbolo::Senao(pos) | Simbolo::EOF(pos) => *pos,
            _ => Posicao::default(),
        }
    }

    fn sincronizar(&mut self) {
        self.avancar();
        
        while !self.esta_no_fim() {
            if matches!(self.simbolo_anterior(), Simbolo::PontoEVirgula(_)) {
                return;
            }
            
            match self.simbolo_atual() {
                Simbolo::Variavel(_) | Simbolo::Funcao(_) | Simbolo::Se(_) |
                Simbolo::Para(_) | Simbolo::Enquanto(_) | Simbolo::Retorna(_) => return,
                _ => { self.avancar(); }
            }
        }
    }

    fn resolve_se_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Se), "Esperado 'se'")?;
        let condition = self.resolve_espressao()?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após a condição")?;
        let then_branch = self.resolve_bloco()?;

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;

        while self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::SenaoSe)) {
            let else_if_condition = self.resolve_espressao()?;
            self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após a condição do else-if")?;
            let else_if_statements = self.resolve_bloco()?;
            else_if_branches.push((else_if_condition, else_if_statements));
        }

        if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::Senao)) {
            self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após 'senao'")?;
            else_branch = Some(self.resolve_bloco()?);
        }

        if else_if_branches.is_empty() {
            Ok(Declaracao::Se {
                condicao: condition,
                ramificacao_entao: then_branch,
                ramificacao_outro: else_branch,
            })
        } else {
            Ok(Declaracao::SeSenao {
                condicao: condition,
                ramificacao_entao: then_branch,
                ramificacao_se_outro: else_if_branches,
                ramificacao_outro: else_branch,
            })
        }
    }

    // Switch-case
    fn resolve_escolha_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Escolha), "Esperado 'escolha'")?;
        let value = self.resolve_espressao()?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após o valor da escolha")?;

        let mut cases = Vec::new();
        let mut default = None;

        while !self.compara(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
            if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::Caso)) {
                let case_value = self.resolve_espressao()?;
                self.consumir(Self::simbolo_com_posicao(Simbolo::DoisPontos), "Esperado ':' após o valor do caso")?;
                let mut case_statements = Vec::new();

                while !self.compara(Self::simbolo_com_posicao(Simbolo::Caso)) && !self.compara(Self::simbolo_com_posicao(Simbolo::Padrao)) && !self.compara(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
                    case_statements.push(self.resolve_declaracao()?);
                }

                cases.push((case_value, case_statements));
            } else if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::Padrao)) {
                self.consumir(Self::simbolo_com_posicao(Simbolo::DoisPontos), "Esperado ':' após 'padrao'")?;
                let mut default_statements = Vec::new();

                while !self.compara(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
                    default_statements.push(self.resolve_declaracao()?);
                }

                default = Some(default_statements);
            } else {
                return Err(CompilerError::AvaliadorSintatico("Esperado 'caso' ou 'padrao' na declaração switch".to_string()));
            }
        }

        self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveDireita), "Esperado '}' após o corpo da escolha")?;

        Ok(Declaracao::Selecao {
            valor: value,
            casos: cases,
            padrao: default,
        })
    }

    fn resolve_enquanto_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Enquanto), "Esperado 'enquanto'")?;
        let condition = self.resolve_espressao()?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após a condição")?;
        let body = self.resolve_bloco()?;

        Ok(Declaracao::Enquanto { condicao: condition, corpo: body })
    }

    fn resolve_faca_enquanto_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Fazer), "Esperado 'fazer'")?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após 'fazer'")?;
        let body = self.resolve_bloco()?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::Enquanto), "Esperado 'enquanto' após o bloco 'fazer'")?;
        let condition = self.resolve_espressao()?;

        Ok(Declaracao::FacaEnquanto { corpo: body, condicao: condition })
    }

    fn resolve_para_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Para), "Esperado 'para'")?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo), "Esperado '(' após 'para'")?;

        let initializer = if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::Variavel)) {
            let name = self.consumir_identificador("Esperado nome da variável")?;
            self.consumir(Self::simbolo_com_posicao(Simbolo::Atribuir), "Esperado '=' após o nome da variável")?;
            let value = self.resolve_espressao()?;
            Some(Box::new(Declaracao::Variavel { nome: name, valor: value }))
        } else if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) {
            None
        } else {
            return Err(CompilerError::AvaliadorSintatico("Esperado declaração de variável ou ';' no loop for".to_string()));
        };
        self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após o inicializador")?;

        let condition = if !self.compara(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) && !self.compara(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda)) {
            Some(self.resolve_espressao()?)
        } else {
            None
        };
        
        if self.compara(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) {
            self.avancar();
        }

        let increment = if !self.compara(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda)) {
            Some(self.resolve_espressao()?)
        } else {
            None
        };
        
        if self.compara(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) {
            self.avancar();
        }

        self.consumir(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Esperado ')' após o cabeçalho do for")?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após o cabeçalho do for")?;
        let body = self.resolve_bloco()?;

        Ok(Declaracao::Para { inicializador: initializer, condicao: condition, incremento: increment, corpo: body })
    }

    fn resolve_para_cada_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::ParaCada), "Esperado palavra-chave 'para cada'")?;
        let variable = self.consumir_identificador("Esperado nome da variável")?;
        self.consumir(Self::identificador_simbolo("de"), "Esperado 'de' após a variável")?;
        let iterable = self.resolve_espressao()?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após o iterável")?;
        let body = self.resolve_bloco()?;

        Ok(Declaracao::ParaCada { variavel: variable, iteravel: iterable, corpo: body })
    }

    fn resolve_sustar_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Sustar), "Esperado palavra-chave 'sustar'")?;
        Ok(Declaracao::Sustar)
    }

    fn resolve_continua_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Continua), "Esperado palavra-chave 'continua'")?;
        Ok(Declaracao::Continua)
    }

    fn resolve_bloco(&mut self) -> Result<Vec<Declaracao>, CompilerError> {
        let mut statements = Vec::new();

        while !self.compara(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
            statements.push(self.resolve_declaracao()?);
        }

        self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveDireita), "Esperado '}' após o bloco")?;
        Ok(statements)
    }

    fn resolve_declaracao_funcao(&mut self) -> Result<Declaracao, CompilerError> {
        let name = self.consumir_identificador("Esperado nome da função")?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo), "Esperado '(' após o nome da função")?;

        let mut params = Vec::new();
        if !self.compara(Self::simbolo_com_posicao(Simbolo::ParenteseDireito)) {
            loop {
                params.push(self.consumir_identificador("Esperado nome do parâmetro")?);

                if self.compara_simbolos(Self::simbolo_com_posicao(Simbolo::Virgula)) {
                    continue;
                } else {
                    break;
                }
            }
        }

        self.consumir(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Esperado ')' após os parâmetros")?;
        self.consumir(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Esperado '{' após a assinatura da função")?;

        let body = self.resolve_bloco()?;

        Ok(Declaracao::DeclaracaoDeFuncao { nome: Some(name), parametros: params, corpo: body })
    }

    fn resolve_retorna_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
        self.consumir(Self::simbolo_com_posicao(Simbolo::Retorna), "Esperado palavra-chave 'retorna'")?;

        let value = if self.compara(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) {
            None
        } else {
            Some(self.resolve_espressao()?)
        };

        self.consumir(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Esperado ';' após a declaração de retorno")?;

        Ok(Declaracao::Retorna(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexador::Lexador;

    #[test]
    fn test_parse_variable_declaration() {
        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var x = 42;");
        let mut parser = AvaliadorSintatico::new(tokens);

        let program = parser.analisar().unwrap();
        assert_eq!(program.declaracoes.len(), 1);

        match &program.declaracoes[0] {
            Declaracao::Variavel { nome: name, valor: value } => {
                assert_eq!(name, "x");
                assert_eq!(*value, Espressao::Numero(42));
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
        let mut parser = AvaliadorSintatico::new(tokens);

        let program = parser.analisar().unwrap();
        assert_eq!(program.declaracoes.len(), 3);
    }

}
