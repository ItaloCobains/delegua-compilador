use crate::core::token::{Simbolo, Posicao};

#[derive(Default)]
struct TokenPool {
    positions: Vec<Posicao>,
    pos_index: usize,
}

impl TokenPool {
    fn new(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            pos_index: 0,
        }
    }

    fn get_position(&mut self, line: u32, column: u32, offset: usize) -> Posicao {
        if self.pos_index < self.positions.len() {
            let pos = &mut self.positions[self.pos_index];
            pos.linha = line;
            pos.coluna = column;
            pos.deslocamento = offset;
            self.pos_index += 1;
            *pos
        } else {
            let pos = Posicao { linha: line, coluna: column, deslocamento: offset };
            self.positions.push(pos);
            self.pos_index += 1;
            pos
        }
    }

    fn reset(&mut self) {
        self.pos_index = 0;
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    position: Posicao,
    token_pool: TokenPool,
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Self {
            input: "",
            chars: "".chars().peekable(),
            position: Posicao { linha: 1, coluna: 1, deslocamento: 0 },
            token_pool: TokenPool::new(1024), // Pre-allocate for 1024 tokens
        }
    }

    pub fn tokenize(&mut self, input: &'a str) -> Vec<Simbolo<'a>> {
        self.input = input;
        self.chars = input.chars().peekable();
        self.position = Posicao { linha: 1, coluna: 1, deslocamento: 0 };
        self.token_pool.reset();

        let mut tokens = Vec::with_capacity(256); // Pre-allocate reasonable capacity

        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.position.linha += 1;
                    self.position.coluna = 1;
                    self.advance();
                }

                '/' => {
                    self.advance();
                    if let Some('/') = self.chars.peek() {
                        self.advance();
                        self.skip_comment();
                    } else {
                        tokens.push(Simbolo::Divisao(self.get_position()));
                    }
                }

                '+' => {
                    self.advance();
                    if let Some('+') = self.chars.peek() {
                        self.advance();
                        tokens.push(Simbolo::Incremento(self.get_position()));
                    } else {
                        tokens.push(Simbolo::Adicao(self.get_position()));
                    }
                }
                '-' => {
                    self.advance();
                    if let Some('-') = self.chars.peek() {
                        self.advance();
                        tokens.push(Simbolo::Decremento(self.get_position()));
                    } else {
                        tokens.push(Simbolo::Subtracao(self.get_position()));
                    }
                }
                '*' => {
                    self.advance();
                    if let Some('*') = self.chars.peek() {
                        self.advance();
                        tokens.push(Simbolo::Potencia(self.get_position()));
                    } else {
                        tokens.push(Simbolo::Multiplicacao(self.get_position()));
                    }
                }
                '%' => { self.advance(); tokens.push(Simbolo::Modulo(self.get_position())); }
                '=' => {
                    self.advance();
                    if let Some('=') = self.chars.peek() {
                        self.advance();
                        tokens.push(Simbolo::Igual(self.get_position()));
                    } else {
                        tokens.push(Simbolo::Atribuir(self.get_position()));
                    }
                }
                '!' => {
                    self.advance();
                    if let Some('=') = self.chars.peek() {
                        self.advance();
                        tokens.push(Simbolo::NaoIgual(self.get_position()));
                    } else {
                        tokens.push(Simbolo::Error('!', self.get_position()));
                    }
                }
                '<' => {
                    self.advance();
                    if let Some('=') = self.chars.peek() {
                        self.advance();
                        tokens.push(Simbolo::MenorIgual(self.get_position()));
                    } else {
                        tokens.push(Simbolo::Menor(self.get_position()));
                    }
                }
                '>' => {
                    self.advance();
                    if let Some('=') = self.chars.peek() {
                        self.advance();
                        tokens.push(Simbolo::MaiorIgual(self.get_position()));
                    } else {
                        tokens.push(Simbolo::Maior(self.get_position()));
                    }
                }
                ';' => { self.advance(); tokens.push(Simbolo::PontoEVirgula(self.get_position())); }
                '(' => { self.advance(); tokens.push(Simbolo::ParenteseEsquerdo(self.get_position())); }
                ')' => { self.advance(); tokens.push(Simbolo::ParenteseDireito(self.get_position())); }
                '{' => { self.advance(); tokens.push(Simbolo::ChaveEsquerda(self.get_position())); }
                '}' => { self.advance(); tokens.push(Simbolo::ChaveDireita(self.get_position())); }
                '[' => { self.advance(); tokens.push(Simbolo::ColcheteEsquerdo(self.get_position())); }
                ']' => { self.advance(); tokens.push(Simbolo::ColcheteDireito(self.get_position())); }
                ',' => { self.advance(); tokens.push(Simbolo::Virgula(self.get_position())); }
                ':' => { self.advance(); tokens.push(Simbolo::DoisPontos(self.get_position())); }
                '.' => {
                    if let Some(next_ch) = self.chars.clone().nth(1) {
                        if next_ch.is_ascii_digit() {
                            let ch = self.chars.next().unwrap();
                            self.position.coluna += 1;
                            self.position.deslocamento += ch.len_utf8();
                            tokens.push(Simbolo::Error(ch, self.get_position()));
                        } else {
                            self.advance();
                            tokens.push(Simbolo::Ponto(self.get_position()));
                        }
                    } else {
                        self.advance();
                        tokens.push(Simbolo::Ponto(self.get_position()));
                    }
                }

                '"' => {
                    if let Some(token) = self.lex_string() {
                        tokens.push(token);
                    }
                }

                '0'..='9' => tokens.push(self.lex_number()),

                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.lex_identifier_or_keyword()),

                _ => {
                    let ch = self.chars.next().unwrap();
                    self.position.coluna += 1;
                    self.position.deslocamento += ch.len_utf8();
                    tokens.push(Simbolo::Error(ch, self.get_position()));
                }
            }
        }

        tokens.push(Simbolo::EOF(self.get_position()));
        tokens
    }

    fn advance(&mut self) {
        if let Some(ch) = self.chars.next() {
            self.position.coluna += 1;
            self.position.deslocamento += ch.len_utf8();
        }
    }

    fn get_position(&mut self) -> Posicao {
        self.token_pool.get_position(self.position.linha, self.position.coluna, self.position.deslocamento)
    }

    fn lex_number(&mut self) -> Simbolo<'a> {
        let start_pos = self.get_position();
        let start_offset = self.position.deslocamento;

        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let num_str = &self.input[start_offset..self.position.deslocamento];
        let value = num_str.parse().unwrap_or(0);
        Simbolo::Number(value, start_pos)
    }

    fn lex_string(&mut self) -> Option<Simbolo<'a>> {
        let start_pos = self.get_position();
        self.advance(); // Skip opening quote
        let start_offset = self.position.deslocamento;

        while let Some(&ch) = self.chars.peek() {
            if ch == '"' {
                let end_offset = self.position.deslocamento;
                self.advance(); // Skip closing quote
                let string_slice = &self.input[start_offset..end_offset];
                return Some(Simbolo::Texto(string_slice, start_pos));
            }
            if ch == '\n' {
                return Some(Simbolo::Error('"', start_pos));
            }
            self.advance();
        }

        Some(Simbolo::Error('"', start_pos))
    }

    fn lex_identifier_or_keyword(&mut self) -> Simbolo<'a> {
        let start_pos = self.get_position();
        let start_offset = self.position.deslocamento;

        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident_slice = &self.input[start_offset..self.position.deslocamento];

        match ident_slice {
            "var" => Simbolo::Variavel(start_pos),
            "escreva" => Simbolo::Escreva(start_pos),
            "texto" => Simbolo::TextoFuncao(start_pos),
            "leia" => Simbolo::Leia(start_pos),
            "comprimento" => Simbolo::Comprimento(start_pos),
            "maiuscula" => Simbolo::Maiuscula(start_pos),
            "minuscula" => Simbolo::Minuscula(start_pos),
            "absoluto" => Simbolo::Absoluto(start_pos),
            "potencia" => Simbolo::PotenciaFuncao(start_pos),
            "raiz_quadrada" => Simbolo::RaizQuadrada(start_pos),
            "importar" => Simbolo::Importacao(start_pos),
            "se" => Simbolo::Se(start_pos),
            "senao" => self.handle_senao_se(start_pos),
            "escolha" => Simbolo::Escolha(start_pos),
            "caso" => Simbolo::Caso(start_pos),
            "padrao" => Simbolo::Padrao(start_pos),
            "enquanto" => Simbolo::Enquanto(start_pos),
            "fazer" => Simbolo::Fazer(start_pos),
            "para" => self.handle_para_cada(start_pos),
            "sustar" => Simbolo::Sustar(start_pos),
            "continua" => Simbolo::Continua(start_pos),
            "verdadeiro" => Simbolo::Verdadeiro(start_pos),
            "falso" => Simbolo::Falso(start_pos),
            "funcao" => Simbolo::Funcao(start_pos),
            "função" => Simbolo::Funcao(start_pos), 
            "retorna" => Simbolo::Retorna(start_pos),
            "e" => Simbolo::E(start_pos),
            "ou" => Simbolo::Ou(start_pos),
            "não" => Simbolo::Nao(start_pos),
            "nao" => Simbolo::Nao(start_pos),
            _ => Simbolo::Identificador(ident_slice, start_pos),
        }
    }

    fn handle_senao_se(&mut self, start_pos: Posicao) -> Simbolo<'a> {
        while let Some(&ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        let mut temp_chars = self.chars.clone();
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
                self.advance();
            }
            Simbolo::SenaoSe(start_pos)
        } else {
            Simbolo::Senao(start_pos)
        }
    }

    fn handle_para_cada(&mut self, start_pos: Posicao) -> Simbolo<'a> {
        while let Some(&ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        let mut temp_chars = self.chars.clone();
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
                self.advance();
            }
            Simbolo::ParaCada(start_pos)
        } else {
            Simbolo::Para(start_pos)
        }
    }

    fn skip_comment(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_variable_declaration() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var x = 42;");
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
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("\"Hello World\"");
        let expected = vec![
            Simbolo::Texto("Hello World", Posicao { linha: 1, coluna: 1, deslocamento: 0 }),
            Simbolo::EOF(Posicao { linha: 1, coluna: 14, deslocamento: 13 })
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_arithmetic_expression() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("a + b * c");
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
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("escreva(\"test\")");
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Simbolo::Escreva(_)));
        assert!(matches!(tokens[1], Simbolo::ParenteseEsquerdo(_)));
        assert!(matches!(tokens[2], Simbolo::Texto("test", _)));
        assert!(matches!(tokens[3], Simbolo::ParenteseDireito(_)));
        assert!(matches!(tokens[4], Simbolo::EOF(_)));
    }

    #[test]
    fn test_tokenize_with_comments() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("var x = 1; // comment\nvar y = 2;");
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
}
