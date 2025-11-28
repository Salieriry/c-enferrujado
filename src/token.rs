use serde::Serialize;

#[derive(Clone, PartialEq, Debug, Serialize)]
pub enum Token {
    // tipos de tokens
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

    InclusaoGlobal(String),
    InclusaoLocal(String),
    Diretiva(String),

    Ponto,
    QuebraLinha,

    Invalido,
    Fundo,

    DeslocamentoEsq,
    DeslocamentoDir,
}
