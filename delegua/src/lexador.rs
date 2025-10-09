use crate::simbolo::{Simbolo, Posicao};

/// Reservatório de objetos `Posicao` reutilizáveis para otimizar a alocação de memória.
/// 
/// Durante a análise léxica, muitas posições são criadas e descartadas. Este pool
/// mantém posições já alocadas que podem ser reutilizadas, reduzindo a pressão no
/// alocador de memória e melhorando a performance.
#[derive(Default)]
struct ReservatorioSimbolos {
    /// Vetor de posições pré-alocadas.
    posicoes: Vec<Posicao>,
    /// Índice da próxima posição disponível no vetor.
    indice_posicao: usize,
}

impl ReservatorioSimbolos {
    /// Cria um novo reservatório com a capacidade inicial pre-alocada.
    /// ### Argumentos
    /// * `capacidade` - Capacidade inicial do vetor de posições.
    fn new(capacidade: usize) -> Self {
        Self {
            posicoes: Vec::with_capacity(capacidade),
            indice_posicao: 0,
        }
    }

    /// Obtém uma posição do reservatório, reutilizando uma existente ou criando uma nova.
    /// ### Argumentos
    /// * `linha` - Linha da posição.
    /// * `coluna` - Coluna da posição.
    /// * `deslocamento` - Deslocamento da posição.
    /// ### Retorna
    /// A posição solicitada.
    fn pega_posicao(&mut self, linha: u32, coluna: u32, deslocamento: usize) -> Posicao {
        if self.indice_posicao < self.posicoes.len() {
            let pos: &mut Posicao = &mut self.posicoes[self.indice_posicao];
            pos.linha = linha;
            pos.coluna = coluna;
            pos.deslocamento = deslocamento;
            self.indice_posicao += 1;
            *pos
        } else {
            let pos: Posicao = Posicao { linha, coluna, deslocamento };
            self.posicoes.push(pos);
            self.indice_posicao += 1;
            pos
        }
    }

    /// Reseta o índice do reservatório para reutilizar as posições desde o início.
    /// Deve ser chamado no início de cada nova análise léxica.
    /// ### Notas
    /// Esta função não limpa o vetor de posições, apenas reseta o índice.
    /// As posições antigas permanecem alocadas para reutilização futura.
    fn reseta(&mut self) {
        self.indice_posicao = 0;
    }
}

/// Analisador léxico para a linguagem Delegua.
/// Converte uma string de entrada em uma sequência de tokens.
pub struct Lexador<'a> {
    /// String de entrada a ser analisada.
    entrada: &'a str,
    /// Iterador sobre os caracteres da string de entrada.
    caracteres: std::iter::Peekable<std::str::Chars<'a>>,
    /// Posição atual na string de entrada.
    posicao: Posicao,
    /// Reservatório de posições para otimização de alocação.
    reservatorio_simbolos: ReservatorioSimbolos,
}

impl<'a> Lexador<'a> {
    /// Cria um novo analisador léxico.
    /// ### Retorna
    /// Uma instância do analisador léxico.
    pub fn new() -> Self {
        Self {
            entrada: "",
            caracteres: "".chars().peekable(),
            posicao: Posicao { linha: 1, coluna: 1, deslocamento: 0 },
            reservatorio_simbolos: ReservatorioSimbolos::new(1024), 
        }
    }

    /// Analisa a string de entrada e retorna uma lista de tokens.
    /// ### Argumentos
    /// * `entrada` - String de entrada a ser analisada.
    /// ### Retorna
    /// Vetor de simbolos resultantes da análise.
    pub fn analisar(&mut self, entrada: &'a str) -> Vec<Simbolo<'a>> {
        self.entrada = entrada;
        self.caracteres = entrada.chars().peekable();
        self.posicao = Posicao { linha: 1, coluna: 1, deslocamento: 0 };
        self.reservatorio_simbolos.reseta();

        let mut simbolos = Vec::with_capacity(256);

        while let Some(&ch) = self.caracteres.peek() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.avancar();
                }
                '\n' => {
                    self.posicao.linha += 1;
                    self.posicao.coluna = 1;
                    self.avancar();
                }

                '/' => {
                    self.avancar();
                    if let Some('/') = self.caracteres.peek() {
                        self.avancar();
                        self.pula_comentario();
                    } else {
                        simbolos.push(Simbolo::Divisao(self.pega_posicao()));
                    }
                }

                '+' => {
                    self.avancar();
                    if let Some('+') = self.caracteres.peek() {
                        self.avancar();
                        simbolos.push(Simbolo::Incremento(self.pega_posicao()));
                    } else {
                        simbolos.push(Simbolo::Adicao(self.pega_posicao()));
                    }
                }
                '-' => {
                    self.avancar();
                    if let Some('-') = self.caracteres.peek() {
                        self.avancar();
                        simbolos.push(Simbolo::Decremento(self.pega_posicao()));
                    } else {
                        simbolos.push(Simbolo::Subtracao(self.pega_posicao()));
                    }
                }
                '*' => {
                    self.avancar();
                    if let Some('*') = self.caracteres.peek() {
                        self.avancar();
                        simbolos.push(Simbolo::Potencia(self.pega_posicao()));
                    } else {
                        simbolos.push(Simbolo::Multiplicacao(self.pega_posicao()));
                    }
                }
                '%' => { self.avancar(); simbolos.push(Simbolo::Modulo(self.pega_posicao())); }
                '=' => {
                    self.avancar();
                    if let Some('=') = self.caracteres.peek() {
                        self.avancar();
                        simbolos.push(Simbolo::Igual(self.pega_posicao()));
                    } else {
                        simbolos.push(Simbolo::Atribuir(self.pega_posicao()));
                    }
                }
                '!' => {
                    self.avancar();
                    if let Some('=') = self.caracteres.peek() {
                        self.avancar();
                        simbolos.push(Simbolo::NaoIgual(self.pega_posicao()));
                    } else {
                        simbolos.push(Simbolo::Error('!', self.pega_posicao()));
                    }
                }
                '<' => {
                    self.avancar();
                    if let Some('=') = self.caracteres.peek() {
                        self.avancar();
                        simbolos.push(Simbolo::MenorIgual(self.pega_posicao()));
                    } else {
                        simbolos.push(Simbolo::Menor(self.pega_posicao()));
                    }
                }
                '>' => {
                    self.avancar();
                    if let Some('=') = self.caracteres.peek() {
                        self.avancar();
                        simbolos.push(Simbolo::MaiorIgual(self.pega_posicao()));
                    } else {
                        simbolos.push(Simbolo::Maior(self.pega_posicao()));
                    }
                }
                ';' => { self.avancar(); simbolos.push(Simbolo::PontoEVirgula(self.pega_posicao())); }
                '(' => { self.avancar(); simbolos.push(Simbolo::ParenteseEsquerdo(self.pega_posicao())); }
                ')' => { self.avancar(); simbolos.push(Simbolo::ParenteseDireito(self.pega_posicao())); }
                '{' => { self.avancar(); simbolos.push(Simbolo::ChaveEsquerda(self.pega_posicao())); }
                '}' => { self.avancar(); simbolos.push(Simbolo::ChaveDireita(self.pega_posicao())); }
                '[' => { self.avancar(); simbolos.push(Simbolo::ColcheteEsquerdo(self.pega_posicao())); }
                ']' => { self.avancar(); simbolos.push(Simbolo::ColcheteDireito(self.pega_posicao())); }
                ',' => { self.avancar(); simbolos.push(Simbolo::Virgula(self.pega_posicao())); }
                ':' => { self.avancar(); simbolos.push(Simbolo::DoisPontos(self.pega_posicao())); }
                '.' => {
                    if let Some(next_ch) = self.caracteres.clone().nth(1) {
                        if next_ch.is_ascii_digit() {
                            let ch = self.caracteres.next().unwrap();
                            self.posicao.coluna += 1;
                            self.posicao.deslocamento += ch.len_utf8();
                            simbolos.push(Simbolo::Error(ch, self.pega_posicao()));
                            // Hoje da erro, mas no futuro pode ser o inicio de um numero float
                        } else {
                            self.avancar();
                            simbolos.push(Simbolo::Ponto(self.pega_posicao()));
                        }
                    } else {
                        self.avancar();
                        simbolos.push(Simbolo::Ponto(self.pega_posicao()));
                    }
                }

                '"' => {
                    if let Some(token) = self.texto_string() {
                        simbolos.push(token);
                    }
                }

                '0'..='9' => simbolos.push(self.resolve_numero()),

                'a'..='z' | 'A'..='Z' | '_' => simbolos.push(self.resolve_identificador_ou_palavra_chave()),

                _ => {
                    let ch = self.caracteres.next().unwrap();
                    self.posicao.coluna += 1;
                    self.posicao.deslocamento += ch.len_utf8();
                    simbolos.push(Simbolo::Error(ch, self.pega_posicao()));
                }
            }
        }

        simbolos.push(Simbolo::EOF(self.pega_posicao()));
        simbolos
    }

    fn avancar(&mut self) {
        if let Some(ch) = self.caracteres.next() {
            self.posicao.coluna += 1;
            self.posicao.deslocamento += ch.len_utf8();
        }
    }

    fn pega_posicao(&mut self) -> Posicao {
        self.reservatorio_simbolos.pega_posicao(self.posicao.linha, self.posicao.coluna, self.posicao.deslocamento)
    }

    fn resolve_numero(&mut self) -> Simbolo<'a> {
        let posicao_inicio = self.pega_posicao();
        let inicio_deslocamento = self.posicao.deslocamento;

        while let Some(&ch) = self.caracteres.peek() {
            if ch.is_ascii_digit() {
                self.avancar();
            } else {
                break;
            }
        }

        let num_str = &self.entrada[inicio_deslocamento..self.posicao.deslocamento];
        let valor = num_str.parse().unwrap_or(0);
        Simbolo::Number(valor, posicao_inicio)
    }

    fn texto_string(&mut self) -> Option<Simbolo<'a>> {
        let posicao_inicio = self.pega_posicao();
        self.avancar(); // pula a aspas iniciais
        let inicio_deslocamento = self.posicao.deslocamento;

        while let Some(&ch) = self.caracteres.peek() {
            if ch == '"' {
                let final_deslocamento = self.posicao.deslocamento;
                self.avancar(); // pula a aspas finais
                let string_slice = &self.entrada[inicio_deslocamento..final_deslocamento];
                return Some(Simbolo::Texto(string_slice, posicao_inicio));
            }
            if ch == '\n' {
                return Some(Simbolo::Error('"', posicao_inicio));
            }
            self.avancar();
        }

        Some(Simbolo::Error('"', posicao_inicio))
    }

    fn resolve_identificador_ou_palavra_chave(&mut self) -> Simbolo<'a> {
        let posicao_inicio = self.pega_posicao();
        let inicio_deslocamento = self.posicao.deslocamento;

        while let Some(&ch) = self.caracteres.peek() {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' {
                self.avancar();
            } else {
                break;
            }
        }

        let ident_slice = &self.entrada[inicio_deslocamento..self.posicao.deslocamento];

        match ident_slice {
            "var" => Simbolo::Variavel(posicao_inicio),
            "escreva" => Simbolo::Escreva(posicao_inicio),
            "texto" => Simbolo::TextoFuncao(posicao_inicio),
            "leia" => Simbolo::Leia(posicao_inicio),
            "comprimento" => Simbolo::Comprimento(posicao_inicio),
            "maiuscula" => Simbolo::Maiuscula(posicao_inicio),
            "minuscula" => Simbolo::Minuscula(posicao_inicio),
            "absoluto" => Simbolo::Absoluto(posicao_inicio),
            "potencia" => Simbolo::PotenciaFuncao(posicao_inicio),
            "raiz_quadrada" => Simbolo::RaizQuadrada(posicao_inicio),
            "importar" => Simbolo::Importacao(posicao_inicio),
            "se" => Simbolo::Se(posicao_inicio),
            "senao" => self.resolve_senao_se(posicao_inicio),
            "escolha" => Simbolo::Escolha(posicao_inicio),
            "caso" => Simbolo::Caso(posicao_inicio),
            "padrao" => Simbolo::Padrao(posicao_inicio),
            "enquanto" => Simbolo::Enquanto(posicao_inicio),
            "fazer" => Simbolo::Fazer(posicao_inicio),
            "para" => self.resolve_para_cada(posicao_inicio),
            "sustar" => Simbolo::Sustar(posicao_inicio),
            "continua" => Simbolo::Continua(posicao_inicio),
            "verdadeiro" => Simbolo::Verdadeiro(posicao_inicio),
            "falso" => Simbolo::Falso(posicao_inicio),
            "funcao" => Simbolo::Funcao(posicao_inicio),
            "função" => Simbolo::Funcao(posicao_inicio), 
            "retorna" => Simbolo::Retorna(posicao_inicio),
            "e" => Simbolo::E(posicao_inicio),
            "ou" => Simbolo::Ou(posicao_inicio),
            "não" => Simbolo::Nao(posicao_inicio),
            "nao" => Simbolo::Nao(posicao_inicio),
            _ => Simbolo::Identificador(ident_slice, posicao_inicio),
        }
    }

    fn resolve_senao_se(&mut self, inicio_posicao: Posicao) -> Simbolo<'a> {
        while let Some(&ch) = self.caracteres.peek() {
            if ch.is_whitespace() {
                self.avancar();
            } else {
                break;
            }
        }

        let mut temp_chars = self.caracteres.clone();
        let mut next_word = String::new();
        while let Some(&ch) = temp_chars.peek() {
            if ch.is_alphabetic() {
                next_word.push(ch);
                temp_chars.next();
            } else {
                break;
            }
        }

        if next_word == "se" {
            for _ in 0.."se".len() {
                self.avancar();
            }
            Simbolo::SenaoSe(inicio_posicao)
        } else {
            Simbolo::Senao(inicio_posicao)
        }
    }

    fn resolve_para_cada(&mut self, inicio_posicao: Posicao) -> Simbolo<'a> {
        while let Some(&ch) = self.caracteres.peek() {
            if ch.is_whitespace() {
                self.avancar();
            } else {
                break;
            }
        }

        let mut temp_chars = self.caracteres.clone();
        let mut next_word = String::new();
        while let Some(&ch) = temp_chars.peek() {
            if ch.is_alphabetic() {
                next_word.push(ch);
                temp_chars.next();
            } else {
                break;
            }
        }

        if next_word == "cada" {
            for _ in 0.."cada".len() {
                self.avancar();
            }
            Simbolo::ParaCada(inicio_posicao)
        } else {
            Simbolo::Para(inicio_posicao)
        }
    }

    fn pula_comentario(&mut self) {
        while let Some(&ch) = self.caracteres.peek() {
            if ch == '\n' {
                break;
            }
            self.avancar();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_variable_declaration() {
        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var x = 42;");
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0], Simbolo::Variavel(_)));
        assert!(matches!(tokens[1], Simbolo::Identificador("x", _)));
        assert!(matches!(tokens[2], Simbolo::Atribuir(_)));
        assert!(matches!(tokens[3], Simbolo::Number(42, _)));
        assert!(matches!(tokens[4], Simbolo::PontoEVirgula(_)));
        assert!(matches!(tokens[5], Simbolo::EOF(_)));
    }

    #[test]
    fn test_tokenize_string_literal() {
        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("\"Hello World\"");
        let expected = vec![
            Simbolo::Texto("Hello World", Posicao { linha: 1, coluna: 1, deslocamento: 0 }),
            Simbolo::EOF(Posicao { linha: 1, coluna: 14, deslocamento: 13 })
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_arithmetic_expression() {
        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("a + b * c");
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0], Simbolo::Identificador("a", _)));
        assert!(matches!(tokens[1], Simbolo::Adicao(_)));
        assert!(matches!(tokens[2], Simbolo::Identificador("b", _)));
        assert!(matches!(tokens[3], Simbolo::Multiplicacao(_)));
        assert!(matches!(tokens[4], Simbolo::Identificador("c", _)));
        assert!(matches!(tokens[5], Simbolo::EOF(_)));
    }

    #[test]
    fn test_tokenize_function_call() {
        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("escreva(\"test\")");
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Simbolo::Escreva(_)));
        assert!(matches!(tokens[1], Simbolo::ParenteseEsquerdo(_)));
        assert!(matches!(tokens[2], Simbolo::Texto("test", _)));
        assert!(matches!(tokens[3], Simbolo::ParenteseDireito(_)));
        assert!(matches!(tokens[4], Simbolo::EOF(_)));
    }

    #[test]
    fn test_tokenize_with_comments() {
        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("var x = 1; // comment\nvar y = 2;");
        assert_eq!(tokens.len(), 11);
        assert!(matches!(tokens[0], Simbolo::Variavel(_)));
        assert!(matches!(tokens[1], Simbolo::Identificador("x", _)));
        assert!(matches!(tokens[2], Simbolo::Atribuir(_)));
        assert!(matches!(tokens[3], Simbolo::Number(1, _)));
        assert!(matches!(tokens[4], Simbolo::PontoEVirgula(_)));
        assert!(matches!(tokens[5], Simbolo::Variavel(_)));
        assert!(matches!(tokens[6], Simbolo::Identificador("y", _)));
        assert!(matches!(tokens[7], Simbolo::Atribuir(_)));
        assert!(matches!(tokens[8], Simbolo::Number(2, _)));
        assert!(matches!(tokens[9], Simbolo::PontoEVirgula(_)));
        assert!(matches!(tokens[10], Simbolo::EOF(_)));
    }

    #[test]
    fn test_se_senao() {
        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("se (x == 1) { escreva(\"Um\") } senao { escreva(\"Dois\") }");
        assert_eq!(tokens.len(),  20);
        assert!(matches!(tokens[0], Simbolo::Se(_)));
        assert!(matches!(tokens[1], Simbolo::ParenteseEsquerdo(_)));
        assert!(matches!(tokens[2], Simbolo::Identificador("x", _)));
        assert!(matches!(tokens[3], Simbolo::Igual(_)));
        assert!(matches!(tokens[4], Simbolo::Number(1, _)));
        assert!(matches!(tokens[5], Simbolo::ParenteseDireito(_)));
        assert!(matches!(tokens[6], Simbolo::ChaveEsquerda(_)));
        assert!(matches!(tokens[7], Simbolo::Escreva(_)));
        assert!(matches!(tokens[8], Simbolo::ParenteseEsquerdo(_)));
        assert!(matches!(tokens[9], Simbolo::Texto("Um", _)));
        assert!(matches!(tokens[10], Simbolo::ParenteseDireito(_)));
        assert!(matches!(tokens[11], Simbolo::ChaveDireita(_)));
        assert!(matches!(tokens[12], Simbolo::Senao(_)));
        assert!(matches!(tokens[13], Simbolo::ChaveEsquerda(_)));
        assert!(matches!(tokens[14], Simbolo::Escreva(_)));
        assert!(matches!(tokens[15], Simbolo::ParenteseEsquerdo(_)));
        assert!(matches!(tokens[16], Simbolo::Texto("Dois", _)));
        assert!(matches!(tokens[17], Simbolo::ParenteseDireito(_)));
        assert!(matches!(tokens[18], Simbolo::ChaveDireita(_)));
        assert!(matches!(tokens[19], Simbolo::EOF(_)));
    }

    #[test]
    fn test_escolha() {
        let mut lexer = Lexador::new();
        let tokens = lexer.analisar("escolha (x) { caso 1: escreva(\"Um\"); caso 2: escreva(\"Dois\"); padrao: escreva(\"Outro\"); }");
        assert_eq!(tokens.len(),  30);
        assert!(matches!(tokens[0], Simbolo::Escolha(_)));
        assert!(matches!(tokens[1], Simbolo::ParenteseEsquerdo(_)));
        assert!(matches!(tokens[2], Simbolo::Identificador("x", _)));
        assert!(matches!(tokens[3], Simbolo::ParenteseDireito(_)));
        assert!(matches!(tokens[4], Simbolo::ChaveEsquerda(_)));
        assert!(matches!(tokens[5], Simbolo::Caso(_)));
        assert!(matches!(tokens[6], Simbolo::Number(1, _)));
        assert!(matches!(tokens[7], Simbolo::DoisPontos(_)));
        assert!(matches!(tokens[8], Simbolo::Escreva(_)));
        assert!(matches!(tokens[9], Simbolo::ParenteseEsquerdo(_)));
        assert!(matches!(tokens[10], Simbolo::Texto("Um", _)));
        assert!(matches!(tokens[11], Simbolo::ParenteseDireito(_)));
        assert!(matches!(tokens[12], Simbolo::PontoEVirgula(_)));
        assert!(matches!(tokens[13], Simbolo::Caso(_)));
        assert!(matches!(tokens[14], Simbolo::Number(2, _)));
        assert!(matches!(tokens[15], Simbolo::DoisPontos(_)));
        assert!(matches!(tokens[16], Simbolo::Escreva(_)));
        assert!(matches!(tokens[17], Simbolo::ParenteseEsquerdo(_)));
        assert!(matches!(tokens[18], Simbolo::Texto("Dois", _)));
        assert!(matches!(tokens[19], Simbolo::ParenteseDireito(_)));
        assert!(matches!(tokens[20], Simbolo::PontoEVirgula(_)));
        assert!(matches!(tokens[21], Simbolo::Padrao(_)));
        assert!(matches!(tokens[22], Simbolo::DoisPontos(_)));
        assert!(matches!(tokens[23], Simbolo::Escreva(_)));
        assert!(matches!(tokens[24], Simbolo::ParenteseEsquerdo(_)));
        assert!(matches!(tokens[25], Simbolo::Texto("Outro", _)));
        assert!(matches!(tokens[26], Simbolo::ParenteseDireito(_)));
        assert!(matches!(tokens[27], Simbolo::PontoEVirgula(_)));
        assert!(matches!(tokens[28], Simbolo::ChaveDireita(_)));
        assert!(matches!(tokens[29], Simbolo::EOF(_)));
    }
}
