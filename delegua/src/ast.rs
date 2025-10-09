#[derive(Debug, PartialEq, Clone)]
pub enum Espressao {
    Numero(i64),

    Texto(String),

    Logico(bool),

    Identificador(String),

    Unario {
        operador: OperacaoBinaria,
        operando: Box<Espressao>,
    },

    Binario {
        esquerda: Box<Espressao>,
        operador: OperacaoBinaria,
        direita: Box<Espressao>,
    },

    ChamadaFuncao {
        chamado: Box<Espressao>,
        argumentos: Vec<Espressao>,
    },
    
    Incremento {
        operando: Box<Espressao>,
        prefixo: bool, // true for ++x, false for x++
    },
    
    Decremento {
        operando: Box<Espressao>,
        prefixo: bool, // true for --x, false for x--
    },

    Funcao {
        paramentros: Vec<String>,
        corpo: Vec<Declaracao>,
    },

    Lista {
        elementos: Vec<Espressao>,
    },

    Indice {
        lista: Box<Espressao>,
        indice: Box<Espressao>,
    },

    Objeto {
        propriedades: Vec<(String, Espressao)>,
    },

    PropriedadeAcesso {
        objeto: Box<Espressao>,
        propriedade: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperacaoBinaria {
    Adicao, Subtracao, Multiplicacao, Divisao, Modulo, Potencia,
    Igual, NaoIgual, Menor, Maior, MenorIgual, MaiorIgual,
    E, Ou, Nao,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaracao {
    Variavel {
        nome: String,
        valor: Espressao,
    },

    Atribuicao {
        nome: String,
        valor: Espressao,
    },

    Importacao {
        modulo: String,
        itens: Option<Vec<String>>,
    },

    Se {
        condicao: Espressao,
        ramificacao_entao: Vec<Declaracao>,
        ramificacao_outro: Option<Vec<Declaracao>>,
    },

    SeSenao {
        condicao: Espressao,
        ramificacao_entao: Vec<Declaracao>,
        ramificacao_se_outro: Vec<(Espressao, Vec<Declaracao>)>,
        ramificacao_outro: Option<Vec<Declaracao>>,
    },

    Selecao { // Switch
        valor: Espressao,
        casos: Vec<(Espressao, Vec<Declaracao>)>,
        padrao: Option<Vec<Declaracao>>,
    },

    Enquanto {
        condicao: Espressao,
        corpo: Vec<Declaracao>,
    },

    FacaEnquanto {
        corpo: Vec<Declaracao>,
        condicao: Espressao,
    },

    Para {
        inicializador: Option<Box<Declaracao>>,
        condicao: Option<Espressao>,
        incremento: Option<Espressao>,
        corpo: Vec<Declaracao>,
    },

    ParaCada {
        variavel: String,
        iteravel: Espressao,
        corpo: Vec<Declaracao>,
    },

    Sustar,

    Continua,

    ChamadaDeFuncao(Espressao),

    DeclaracaoDeFuncao {
        nome: Option<String>,
        parametros: Vec<String>,
        corpo: Vec<Declaracao>,
    },

    Retorna(Option<Espressao>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Programa {
    pub declaracoes: Vec<Declaracao>,
}
