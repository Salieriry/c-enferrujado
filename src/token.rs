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

    Incremento,
    Decremento,
    Soma,
    Subtracao,
    Asterisco,
    Divisao,
    Modulo,

    EComercial,
    Maior,
    Menor,
    MaiorOuIgual,
    Comparar,
    MenorOuIgual,
    Diferente,
    Negacao,

    Numero(String),
    Texto(String),
    ConteudoChar(String),
    Identificador(String),

    Hashtag,

    Burro,
    Fundo,
}
