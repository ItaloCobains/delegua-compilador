#[derive(Debug, PartialEq, Clone)]
pub enum Expressoes {
    Numero(i64),

    Texto(String),

    Logico(bool),

    Identificador(String),

    Unario {
        operador: OperacaoBinaria,
        operando: Box<Expressoes>,
    },

    Binario {
        esquerda: Box<Expressoes>,
        operador: OperacaoBinaria,
        direita: Box<Expressoes>,
    },

    ChamadaFuncao {
        chamado: Box<Expressoes>,
        argumentos: Vec<Expressoes>,
    },
    
    Incremento {
        operando: Box<Expressoes>,
        prefixo: bool, // true for ++x, false for x++
    },
    
    Decremento {
        operando: Box<Expressoes>,
        prefixo: bool, // true for --x, false for x--
    },

    Funcao {
        paramentros: Vec<String>,
        corpo: Vec<Declaracao>,
    },

    Lista {
        elementos: Vec<Expressoes>,
    },

    Indice {
        lista: Box<Expressoes>,
        indice: Box<Expressoes>,
    },

    Objeto {
        propriedades: Vec<(String, Expressoes)>,
    },

    PropriedadeAcesso {
        objeto: Box<Expressoes>,
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
        valor: Expressoes,
    },

    Atribuicao {
        nome: String,
        valor: Expressoes,
    },

    Importacao {
        modulo: String,
        itens: Option<Vec<String>>,
    },

    Se {
        condicao: Expressoes,
        ramificacao_entao: Vec<Declaracao>,
        ramificacao_outro: Option<Vec<Declaracao>>,
    },

    SeSenao {
        condicao: Expressoes,
        ramificacao_entao: Vec<Declaracao>,
        ramificacao_se_outro: Vec<(Expressoes, Vec<Declaracao>)>,
        ramificacao_outro: Option<Vec<Declaracao>>,
    },

    Selecao { // Switch
        valor: Expressoes,
        casos: Vec<(Expressoes, Vec<Declaracao>)>,
        padrao: Option<Vec<Declaracao>>,
    },

    Enquanto {
        condicao: Expressoes,
        corpo: Vec<Declaracao>,
    },

    FacaEnquanto {
        corpo: Vec<Declaracao>,
        condicao: Expressoes,
    },

    Para {
        inicializador: Option<Box<Declaracao>>,
        condicao: Option<Expressoes>,
        incremento: Option<Expressoes>,
        corpo: Vec<Declaracao>,
    },

    ParaCada {
        variavel: String,
        iteravel: Expressoes,
        corpo: Vec<Declaracao>,
    },

    Sustar,

    Continua,

    ChamadaDeFuncao(Expressoes),

    DeclaracaoDeFuncao {
        nome: Option<String>,
        parametros: Vec<String>,
        corpo: Vec<Declaracao>,
    },

    Retorna(Option<Expressoes>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Programa {
    pub declaracoes: Vec<Declaracao>,
}
