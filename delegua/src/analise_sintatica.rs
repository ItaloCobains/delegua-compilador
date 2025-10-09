use crate::simbolo::Simbolo;
use crate::ast::{Expressoes, OperacaoBinaria, Declaracao, Programa};
use crate::error::CompilerError;
use crate::simbolo::Posicao;

/// Analisador sintático para a linguagem de programação Delegua.
/// Recebe uma lista de tokens e produz uma árvore de sintaxe abstrata (AST).
/// Implementa um analisador recursivo descendente com tratamento de erros.
pub struct AnaliseSintatica<'a> {
    /// Lista de tokens a serem analisados.
    simbolos: Vec<Simbolo<'a>>,
    /// Índice do token atual na lista.
    atual: usize,
    /// Token EOF para facilitar a verificação de fim de arquivo.
    eof_simbolo: Simbolo<'a>,
}

impl<'a> AnaliseSintatica<'a> {
    /// Gera uma nova instância do analisador sintático com a lista de tokens fornecida.
    /// # Argumentos
    /// * `simbolos` - Vetor de tokens a serem analisados.
    /// # Retorna
    /// Nova instância do analisador sintático.
    pub fn new(simbolos: Vec<Simbolo<'a>>) -> Self {
        AnaliseSintatica { 
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

        Ok(Programa { declaracoes })
    }

    fn resolve_declaracao(&mut self) -> Result<Declaracao, CompilerError> {
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

    fn parse_variable_declaration(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Variavel), "Expected 'var' keyword")?;

        let name = self.consume_identifier("Expected variable name")?;
        self.consume(Self::simbolo_com_posicao(Simbolo::Atribuir), "Expected '=' after variable name")?;
        let initializer = self.parse_expression()?;
        self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after variable declaration")?;

        Ok(Declaracao::Variavel { nome: name, valor: initializer })
    }

    fn parse_assignment_or_call(&mut self) -> Result<Declaracao, CompilerError> {
        let name = self.consume_identifier("Expected identifier")?;

        if self.match_token(Self::simbolo_com_posicao(Simbolo::Atribuir)) {
            let value = self.parse_expression()?;
            self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after assignment")?;
            Ok(Declaracao::Atribuicao { nome: name, valor: value })
        } else if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
            let args = self.parse_arguments()?;
            self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
            self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after function call")?;
            Ok(Declaracao::ChamadaDeFuncao(Expressoes::ChamadaFuncao {
                chamado: Box::new(Expressoes::Identificador(name)),
                argumentos: args,
            }))
        } else if matches!(self.current_token(), Simbolo::Incremento(_)) {
            self.advance(); // consume ++
            self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after increment")?;
            Ok(Declaracao::ChamadaDeFuncao(Expressoes::Incremento {
                operando: Box::new(Expressoes::Identificador(name)),
                prefixo: false, // postfix: x++
            }))
        } else if matches!(self.current_token(), Simbolo::Decremento(_)) {
            self.advance(); // consume --
            self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after decrement")?;
            Ok(Declaracao::ChamadaDeFuncao(Expressoes::Decremento {
                operando: Box::new(Expressoes::Identificador(name)),
                prefixo: false, // postfix: x--
            }))
        } else {
            Err(CompilerError::Parser(
                "Expected '=', '(', '++', or '--' after identifier".to_string()
            ))
        }
    }
    
    fn parse_prefix_increment_decrement(&mut self, is_increment: bool) -> Result<Declaracao, CompilerError> {
        self.advance(); // consume ++ or --
        
        let name = match self.current_token() {
            Simbolo::Identificador(name, _) => name.to_string(),
            _ => return Err(CompilerError::Parser("Expected identifier after ++ or --".to_string())),
        };
        self.advance();
        
        self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after prefix increment/decrement")?;
        
        if is_increment {
            Ok(Declaracao::ChamadaDeFuncao(Expressoes::Incremento {
                operando: Box::new(Expressoes::Identificador(name)),
                prefixo: true, // prefix: ++x
            }))
        } else {
            Ok(Declaracao::ChamadaDeFuncao(Expressoes::Decremento {
                operando: Box::new(Expressoes::Identificador(name)),
                prefixo: true, // prefix: --x
            }))
        }
    }

    fn parse_function_call_statement(&mut self) -> Result<Declaracao, CompilerError> {
        let expr = self.parse_function_call()?;
        self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after function call")?;
        Ok(Declaracao::ChamadaDeFuncao(expr))
    }

    fn parse_import_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Importacao), "Expected 'importar' keyword")?;

        if let Simbolo::Texto(module, _) = self.current_token() {
            let module = module.to_string();
            self.advance();
            self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after import")?;
            Ok(Declaracao::Importacao { modulo: module, itens: None })
        } else if self.match_token(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda)) {
            let mut items = Vec::new();
            while !self.check(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
                if let Simbolo::Identificador(name, _) = self.current_token() {
                    let name = name.to_string();
                    items.push(name);
                    self.advance();
                } else {
                    return Err(CompilerError::Parser("Expected identifier in import list".to_string()));
                }
                if self.match_token(Self::simbolo_com_posicao(Simbolo::Virgula)) {
                    // continue
                } else if !self.check(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) {
                    return Err(CompilerError::Parser("Expected ',' or '}' in import list".to_string()));
                }
            }
            self.consume(Self::simbolo_com_posicao(Simbolo::ChaveDireita), "Expected '}' after import list")?;
            self.consume(Self::identificador_simbolo("from"), "Expected 'from' after import list")?;
            if let Simbolo::Texto(module, _) = self.current_token() {
                let module = module.to_string();
                self.advance();
                self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after import")?;
                Ok(Declaracao::Importacao { modulo: module, itens: Some(items) })
            } else {
                Err(CompilerError::Parser("Expected module name after 'from'".to_string()))
            }
        } else {
            Err(CompilerError::Parser("Expected string or '{' after 'import'".to_string()))
        }
    }

    fn parse_function_call(&mut self) -> Result<Expressoes, CompilerError> {
        let callee = self.parse_primary()?;

        if let Expressoes::ChamadaFuncao { .. } = callee {
            return Ok(callee);
        }

        self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo), "Expected '(' after function name")?;

        let args = self.parse_arguments()?;

        self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;

        Ok(Expressoes::ChamadaFuncao { chamado: Box::new(callee), argumentos: args })
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expressoes>, CompilerError> {
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

    fn parse_expression(&mut self) -> Result<Expressoes, CompilerError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expressoes, CompilerError> {
        let mut left = self.parse_logical_and()?;

        while matches!(self.current_token(), Simbolo::Ou(_)) {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expressoes::Binario {
                esquerda: Box::new(left),
                operador: OperacaoBinaria::Ou,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expressoes, CompilerError> {
        let mut left = self.parse_comparison()?;

        while matches!(self.current_token(), Simbolo::E(_)) {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expressoes::Binario {
                esquerda: Box::new(left),
                operador: OperacaoBinaria::E,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expressoes, CompilerError> {
        let mut left = self.parse_additive()?;

        while matches!(self.current_token(), 
            Simbolo::Menor(_) | Simbolo::Maior(_) | Simbolo::MenorIgual(_) | 
            Simbolo::MaiorIgual(_) | Simbolo::Igual(_) | Simbolo::NaoIgual(_)
        ) {
            let operator = match self.current_token() {
                Simbolo::Menor(_) => OperacaoBinaria::Menor,
                Simbolo::Maior(_) => OperacaoBinaria::Maior,
                Simbolo::MenorIgual(_) => OperacaoBinaria::MenorIgual,
                Simbolo::MaiorIgual(_) => OperacaoBinaria::MaiorIgual,
                Simbolo::Igual(_) => OperacaoBinaria::Igual,
                Simbolo::NaoIgual(_) => OperacaoBinaria::NaoIgual,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_additive()?;
            left = Expressoes::Binario {
                esquerda: Box::new(left),
                operador: operator,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expressoes, CompilerError> {
        let mut left = self.parse_multiplicative()?;

        while matches!(self.current_token(), Simbolo::Adicao(_) | Simbolo::Subtracao(_)) {
            let operator = match self.current_token() {
                Simbolo::Adicao(_) => OperacaoBinaria::Adicao,
                Simbolo::Subtracao(_) => OperacaoBinaria::Subtracao,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_multiplicative()?;
            left = Expressoes::Binario {
                esquerda: Box::new(left),
                operador: operator,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expressoes, CompilerError> {
        let mut left = self.parse_power()?;

        while matches!(self.current_token(), Simbolo::Multiplicacao(_) | Simbolo::Divisao(_) | Simbolo::Modulo(_)) {
            let operator = match self.current_token() {
                Simbolo::Multiplicacao(_) => OperacaoBinaria::Multiplicacao,
                Simbolo::Divisao(_) => OperacaoBinaria::Divisao,
                Simbolo::Modulo(_) => OperacaoBinaria::Modulo,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_power()?;
            left = Expressoes::Binario {
                esquerda: Box::new(left),
                operador: operator,
                direita: Box::new(right),
            };
        }

        Ok(left)
    }
    
    fn parse_power(&mut self) -> Result<Expressoes, CompilerError> {
        let mut left = self.parse_unary()?;
        
        // Right associative: 2**3**2 = 2**(3**2) = 512
        if matches!(self.current_token(), Simbolo::Potencia(_)) {
            self.advance();
            let right = self.parse_power()?; // Right associative recursion
            left = Expressoes::Binario {
                esquerda: Box::new(left),
                operador: OperacaoBinaria::Potencia,
                direita: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expressoes, CompilerError> {
        match self.current_token() {
            Simbolo::Adicao(_) => {
                self.advance();
                self.parse_unary()
            }
            Simbolo::Subtracao(_) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expressoes::Unario {
                    operador: OperacaoBinaria::Subtracao,
                    operando: Box::new(operand),
                })
            }
            Simbolo::Nao(_) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expressoes::Unario {
                    operador: OperacaoBinaria::Nao,
                    operando: Box::new(operand),
                })
            }
            Simbolo::Incremento(_) => {
                self.advance();
                let operand = self.parse_postfix()?;
                Ok(Expressoes::Incremento {
                    operando: Box::new(operand),
                    prefixo: true,
                })
            }
            Simbolo::Decremento(_) => {
                self.advance();
                let operand = self.parse_postfix()?;
                Ok(Expressoes::Decremento {
                    operando: Box::new(operand),
                    prefixo: true,
                })
            }
            _ => self.parse_postfix()
        }
    }
    
    fn parse_postfix(&mut self) -> Result<Expressoes, CompilerError> {
        let mut expr = self.parse_primary()?;
        
        // Handle postfix increment/decrement
        loop {
            match self.current_token() {
                Simbolo::Incremento(_) => {
                    self.advance();
                    expr = Expressoes::Incremento {
                        operando: Box::new(expr),
                        prefixo: false,
                    };
                }
                Simbolo::Decremento(_) => {
                    self.advance();
                    expr = Expressoes::Decremento {
                        operando: Box::new(expr),
                        prefixo: false,
                    };
                }
                Simbolo::ColcheteEsquerdo(_) => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ColcheteDireito), "Expected ']' after array index")?;
                    expr = Expressoes::Indice {
                        lista: Box::new(expr),
                        indice: Box::new(index),
                    };
                }
                Simbolo::Ponto(_) => {
                    self.advance();
                    if let Simbolo::Identificador(name, _) = self.current_token() {
                        let property = name.to_string();
                        self.advance();
                        expr = Expressoes::PropriedadeAcesso {
                            objeto: Box::new(expr),
                            propriedade: property,
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

    fn parse_primary(&mut self) -> Result<Expressoes, CompilerError> {
        match *self.current_token() {
            Simbolo::Number(n, _) => {
                self.advance();
                Ok(Expressoes::Numero(n))
            }
            Simbolo::Texto(ref s, _) => {
                let s = s.to_string();
                self.advance();
                Ok(Expressoes::Texto(s))
            }
            Simbolo::Verdadeiro(_) => {
                self.advance();
                Ok(Expressoes::Logico(true))
            }
            Simbolo::Falso(_) => {
                self.advance();
                Ok(Expressoes::Logico(false))
            }
            Simbolo::Escreva(_) => {
                self.advance();
                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador("escreva".to_string())),
                        argumentos: args,
                    })
                } else {
                    Ok(Expressoes::Identificador("escreva".to_string()))
                }
            }
            Simbolo::Identificador(name, _) => {
                let name = name.to_string();
                self.advance();

                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador(name)),
                        argumentos: args,
                    })
                } else {
                    Ok(Expressoes::Identificador(name))
                }
            }
            Simbolo::Funcao(_) => {
                self.advance();
                self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo), "Expected '(' after 'funcao'")?;
                let params = self.parse_parameters()?;
                self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after parameters")?;
                self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after function signature")?;
                let body = self.parse_block()?;
                Ok(Expressoes::Funcao { paramentros: params, corpo: body })
            }
            Simbolo::TextoFuncao(_) => {
                self.advance();
                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador("texto".to_string())),
                        argumentos: args,
                    })
                } else {
                    // Treat as variable identifier
                    Ok(Expressoes::Identificador("texto".to_string()))
                }
            }
            Simbolo::Leia(_) => {
                self.advance();
                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador("leia".to_string())),
                        argumentos: args,
                    })
                } else {
                    Ok(Expressoes::Identificador("leia".to_string()))
                }
            }
            Simbolo::Comprimento(_) => {
                self.advance();
                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador("comprimento".to_string())),
                        argumentos: args,
                    })
                } else {
                    Ok(Expressoes::Identificador("comprimento".to_string()))
                }
            }
            Simbolo::Maiuscula(_) => {
                self.advance();
                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador("maiuscula".to_string())),
                        argumentos: args,
                    })
                } else {
                    Ok(Expressoes::Identificador("maiuscula".to_string()))
                }
            }
            Simbolo::Minuscula(_) => {
                self.advance();
                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador("minuscula".to_string())),
                        argumentos: args,
                    })
                } else {
                    Ok(Expressoes::Identificador("minuscula".to_string()))
                }
            }
            Simbolo::Absoluto(_) => {
                self.advance();
                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador("absoluto".to_string())),
                        argumentos: args,
                    })
                } else {
                    Ok(Expressoes::Identificador("absoluto".to_string()))
                }
            }
            Simbolo::PotenciaFuncao(_) => {
                self.advance();
                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador("potencia".to_string())),
                        argumentos: args,
                    })
                } else {
                    Ok(Expressoes::Identificador("potencia".to_string()))
                }
            }
            Simbolo::RaizQuadrada(_) => {
                self.advance();
                if self.match_token(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo)) {
                    let args = self.parse_arguments()?;
                    self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after function arguments")?;
                    Ok(Expressoes::ChamadaFuncao {
                        chamado: Box::new(Expressoes::Identificador("raiz_quadrada".to_string())),
                        argumentos: args,
                    })
                } else {
                    Ok(Expressoes::Identificador("raiz_quadrada".to_string()))
                }
            }
            Simbolo::ParenteseEsquerdo(_) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after expression")?;
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

                self.consume(Self::simbolo_com_posicao(Simbolo::ColcheteDireito), "Expected ']' after array elements")?;
                Ok(Expressoes::Lista { elementos: elements })
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

                        self.consume(Self::simbolo_com_posicao(Simbolo::DoisPontos), "Expected ':' after property name")?;
                        let value = self.parse_expression()?;
                        properties.push((key, value));

                        if matches!(self.current_token(), Simbolo::Virgula(_)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.consume(Self::simbolo_com_posicao(Simbolo::ChaveDireita), "Expected '}' after object properties")?;
                Ok(Expressoes::Objeto { propriedades: properties })
            }
            _ => Err(CompilerError::Parser(format!(
                "Unexpected token in expression: {:?}", self.current_token()
            ))),
        }
    }


    #[inline]
    fn current_token(&self) -> &Simbolo<'a> {
        self.simbolos.get(self.atual).unwrap_or(&self.eof_simbolo)
    }

    #[inline]
    fn previous_token(&self) -> &Simbolo<'a> {
        if self.atual > 0 {
            self.simbolos.get(self.atual - 1).unwrap_or(&self.eof_simbolo)
        } else {
            &self.eof_simbolo
        }
    }

    #[inline]
    fn advance(&mut self) -> &Simbolo<'a> {
        if !self.esta_no_fim() {
            self.atual += 1;
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
        !self.esta_no_fim() && self.token_matches(&expected)
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

    fn esta_no_fim(&self) -> bool {
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
        
        while !self.esta_no_fim() {
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

    fn parse_if_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Se), "Expected 'se' keyword")?;
        let condition = self.parse_expression()?;
        self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after condition")?;
        let then_branch = self.parse_block()?;

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;

        while self.match_token(Self::simbolo_com_posicao(Simbolo::SenaoSe)) {
            let else_if_condition = self.parse_expression()?;
            self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after else-if condition")?;
            let else_if_statements = self.parse_block()?;
            else_if_branches.push((else_if_condition, else_if_statements));
        }

        if self.match_token(Self::simbolo_com_posicao(Simbolo::Senao)) {
            self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after 'senao'")?;
            else_branch = Some(self.parse_block()?);
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

    fn parse_switch_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Escolha), "Expected 'escolha' keyword")?;
        let value = self.parse_expression()?;
        self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after switch value")?;

        let mut cases = Vec::new();
        let mut default = None;

        while !self.check(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
            if self.match_token(Self::simbolo_com_posicao(Simbolo::Caso)) {
                let case_value = self.parse_expression()?;
                self.consume(Self::simbolo_com_posicao(Simbolo::DoisPontos), "Expected ':' after case value")?;
                let mut case_statements = Vec::new();

                while !self.check(Self::simbolo_com_posicao(Simbolo::Caso)) && !self.check(Self::simbolo_com_posicao(Simbolo::Padrao)) && !self.check(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
                    case_statements.push(self.resolve_declaracao()?);
                }

                cases.push((case_value, case_statements));
            } else if self.match_token(Self::simbolo_com_posicao(Simbolo::Padrao)) {
                self.consume(Self::simbolo_com_posicao(Simbolo::DoisPontos), "Expected ':' after 'padrao'")?;
                let mut default_statements = Vec::new();

                while !self.check(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
                    default_statements.push(self.resolve_declaracao()?);
                }

                default = Some(default_statements);
            } else {
                return Err(CompilerError::Parser("Expected 'caso' or 'padrao' in switch statement".to_string()));
            }
        }

        self.consume(Self::simbolo_com_posicao(Simbolo::ChaveDireita), "Expected '}' after switch body")?;

        Ok(Declaracao::Selecao {
            valor: value,
            casos: cases,
            padrao: default,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Enquanto), "Expected 'enquanto' keyword")?;
        let condition = self.parse_expression()?;
        self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after condition")?;
        let body = self.parse_block()?;

        Ok(Declaracao::Enquanto { condicao: condition, corpo: body })
    }

    fn parse_do_while_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Fazer), "Expected 'fazer' keyword")?;
        self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after 'fazer'")?;
        let body = self.parse_block()?;
        self.consume(Self::simbolo_com_posicao(Simbolo::Enquanto), "Expected 'enquanto' after do block")?;
        let condition = self.parse_expression()?;

        Ok(Declaracao::FacaEnquanto { corpo: body, condicao: condition })
    }

    fn parse_for_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Para), "Expected 'para' keyword")?;
        self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo), "Expected '(' after 'para'")?;

        let initializer = if self.match_token(Self::simbolo_com_posicao(Simbolo::Variavel)) {
            let name = self.consume_identifier("Expected variable name")?;
            self.consume(Self::simbolo_com_posicao(Simbolo::Atribuir), "Expected '=' after variable name")?;
            let value = self.parse_expression()?;
            Some(Box::new(Declaracao::Variavel { nome: name, valor: value }))
        } else if self.match_token(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) {
            None
        } else {
            return Err(CompilerError::Parser("Expected variable declaration or ';' in for loop".to_string()));
        };
        self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after initializer")?;

        let condition = if !self.check(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) && !self.check(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda)) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if self.check(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) {
            self.advance();
        }

        let increment = if !self.check(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda)) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if self.check(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) {
            self.advance();
        }

        self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after for header")?;
        self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after for header")?;
        let body = self.parse_block()?;

        Ok(Declaracao::Para { inicializador: initializer, condicao: condition, incremento: increment, corpo: body })
    }

    fn parse_for_each_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::ParaCada), "Expected 'para cada' keyword")?;
        let variable = self.consume_identifier("Expected variable name")?;
        self.consume(Self::identificador_simbolo("de"), "Expected 'de' after variable")?;
        let iterable = self.parse_expression()?;
        self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after iterable")?;
        let body = self.parse_block()?;

        Ok(Declaracao::ParaCada { variavel: variable, iteravel: iterable, corpo: body })
    }

    fn parse_break_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Sustar), "Expected 'sustar' keyword")?;
        Ok(Declaracao::Interromper)
    }

    fn parse_continue_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Continua), "Expected 'continua' keyword")?;
        Ok(Declaracao::Continue)
    }

    fn parse_block(&mut self) -> Result<Vec<Declaracao>, CompilerError> {
        let mut statements = Vec::new();

        while !self.check(Self::simbolo_com_posicao(Simbolo::ChaveDireita)) && !self.esta_no_fim() {
            statements.push(self.resolve_declaracao()?);
        }

        self.consume(Self::simbolo_com_posicao(Simbolo::ChaveDireita), "Expected '}' after block")?;
        Ok(statements)
    }

    fn parse_named_function_declaration(&mut self) -> Result<Declaracao, CompilerError> {
        let name = self.consume_identifier("Expected function name")?;
        self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseEsquerdo), "Expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(Self::simbolo_com_posicao(Simbolo::ParenteseDireito)) {
            loop {
                params.push(self.consume_identifier("Expected parameter name")?);

                if self.match_token(Self::simbolo_com_posicao(Simbolo::Virgula)) {
                    continue;
                } else {
                    break;
                }
            }
        }

        self.consume(Self::simbolo_com_posicao(Simbolo::ParenteseDireito), "Expected ')' after parameters")?;
        self.consume(Self::simbolo_com_posicao(Simbolo::ChaveEsquerda), "Expected '{' after function signature")?;

        let body = self.parse_block()?;

        Ok(Declaracao::DeclaracaoDeFuncao { nome: Some(name), parametros: params, corpo: body })
    }

    fn parse_return_statement(&mut self) -> Result<Declaracao, CompilerError> {
        self.consume(Self::simbolo_com_posicao(Simbolo::Retorna), "Expected 'retorna' keyword")?;

        let value = if self.check(Self::simbolo_com_posicao(Simbolo::PontoEVirgula)) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume(Self::simbolo_com_posicao(Simbolo::PontoEVirgula), "Expected ';' after return statement")?;

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
        let mut parser = AnaliseSintatica::new(tokens);

        let program = parser.analisar().unwrap();
        assert_eq!(program.declaracoes.len(), 1);

        match &program.declaracoes[0] {
            Declaracao::Variavel { nome: name, valor: value } => {
                assert_eq!(name, "x");
                assert_eq!(*value, Expressoes::Numero(42));
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
        let mut parser = AnaliseSintatica::new(tokens);

        let program = parser.analisar().unwrap();
        assert_eq!(program.declaracoes.len(), 3);
    }

}
