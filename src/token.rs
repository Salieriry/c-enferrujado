pub enum Token {
    Mais, Menos, Igual,

    PontoVirgula, AbreParentesis, FechaParentesis, AbreChave, FechaChave, AbreColchete, FechaColchete,
    AspasSimples, Aspas,

    Incremento, Decremento, Soma, Subtracao, Multiplicacao, Divisao, Modulo,

    Referenciador, Maior, Menor, MaiorOuIgual, Comparar, MenorOuIgual, Diferente, Negacao,

    Numero(String), Texto(String), Char(char),
    Identificador(String), Boolean(bool),
}