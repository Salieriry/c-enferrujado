#[derive(Clone, PartialEq)]
pub enum Token {
    // enumeração dos tipos de tokens
    Mais,
    Menos,
    Igual,

    PontoVirgula,
    AbreParentesis,
    FechaParentesis,
    AbreChave,
    FechaChave,
    AbreColchete,
    FechaColchete,
    Virgula,

    Incremento,
    Decremento,
    SomaIgual,
    SubtracaoIgual,
    MultiplicacaoIgual,
    DivisaoIgual,
    ModuloIgual,
    Asterisco,
    Divisao,
    Modulo,

    EComercial,
    EComercialDuplo,
    BarraVertical,
    BarraVerticalDupla,
    Maior,
    Menor,
    MaiorOuIgual,
    Comparar,
    MenorOuIgual,
    Diferente,
    Negacao,

    NumeroInt(String),
    NumeroFloat(String),
    Texto(String),
    ConteudoChar(char),
    Identificador(String),

    InclusaoGlobal(String),  // para <iostream.h>, por exemplo
    InclusaoLocal(String),   // para "meu_arquivo.h", por exemplo
    Diretiva(String),        // para outras diretivas, ex: #define

    Ponto,
    QuebraLinha,

    Burro,
    Fundo,
}
