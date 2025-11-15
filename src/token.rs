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

    InclusaoGlobal(String),  // para <iostream.h>, por exemplo
    InclusaoLocal(String),   // para "meu_arquivo.h", por exemplo
    Diretiva(String),        // para outras diretivas, ex: #define

    Ponto,

    Burro,
    Fundo,
}
