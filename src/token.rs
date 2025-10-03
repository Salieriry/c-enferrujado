#[derive(Debug)]
pub enum Token {
    Mais, Menos, Igual,

    PontoVirgula, AbreParentesis, FechaParentesis, AbreChave, FechaChave, AbreColchete, FechaColchete,

    Incremento, Decremento, Soma, Subtracao, Multiplicacao, Divisao, Modulo,

    Referenciador, Maior, Menor, MaiorOuIgual, Comparar, MenorOuIgual, Diferente, Negacao,

    Numero(String), Texto(String), ConteudoChar(String),
    Identificador(String),
    
    Burro, Fundo,
}