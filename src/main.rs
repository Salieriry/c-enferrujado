mod lexer;
mod token;
mod parser;


use crate::lexer::Lexer;
use crate::token::Token;
use crate::parser::{Expr, Operador, Parser, Stmt};


fn main() {
    // código de teste que usa declaração, atribuição e expressões
    let codigo = "#include <iostream>

    // Função principal para testar o lexer.
    int main() {
        // Declaração de variáveis com diferentes tipos de literais
        int contador = 10;
        float saldo_conta = 150.75;
        char inicial = 'C';
        const char* mensagem = \"Olá, mundo! Teste 123.\";

        // Operadores de atribuição e incremento/decremento
        contador += 5;
        contador++;
        --saldo_conta;

        // Estrutura de controle com operadores de comparação
        if (contador >= 15 & saldo_conta < 200.0) {
            int resultado = (contador * 2) % 3;
            int arr[2];
            arr[0] = resultado;
        }

        // Verificação de outros operadores e pontuação
        bool sao_diferentes = (1 != 2);
        int* ponteiro = &contador;

        // Teste de caractere ilegal


        return 0;
        }";
    
    println!("Analisando o programa:\n\"{}\"\n", codigo);

    // lexer
    let mut lexer = Lexer::new(codigo.to_string());
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = lexer.prox_token();
        let is_fundo = if let Token::Fundo = token { true } else { false };
        tokens.push(token);
        if is_fundo {
            break;
        }
    }

    // parser
    println!("--- Árvore de Sintaxe Abstrata (AST) ---");
    let mut parser = Parser::new(tokens);
    let ast_programa = parser.parse();

    // visualização da AST
    imprimir_ast_programa(&ast_programa);
}


/// função de nível superior para imprimir o programa (um vetor de Stmt)
fn imprimir_ast_programa(programa: &Vec<Stmt>) {
    if programa.is_empty() {
        println!("(Programa vazio)");
        return;
    }
    for (i, stmt) in programa.iter().enumerate() {
        println!("\nDeclaração [{}]:", i + 1);
        imprimir_stmt(stmt, 1);
    }
}

/// imprime recursivamente uma Declaração (Stmt)
fn imprimir_stmt(stmt: &Stmt, indent: usize) {
    let prefix = "  ".repeat(indent);
    match stmt {
        Stmt::Expressao(expr) => {
            println!("{}Expressao:", prefix);
            imprimir_expr(expr, indent + 1);
        }
        Stmt::DeclaracaoVariavel { tipo, nome, tamanho_array, inicializador } => {
            println!("{}DeclaracaoVariavel:", prefix);
            print!("{}  Tipo: ", prefix);
            for (i, token) in tipo.iter().enumerate() {
                if i > 0 { print!(" "); }
                imprimir_token(token);
            }
            print!("\n{}  Nome: ", prefix);
            imprimir_token(nome);

            match tamanho_array {
                Some(expr) => {
                    println!("\n{}  Array Size:", prefix);
                    imprimir_expr(expr, indent + 2);
                }
                None => {} // não imprime nada se não for array
            }

            match inicializador {
                Some(expr) => {
                    println!("\n{}  Inicializador:", prefix);
                    imprimir_expr(expr, indent + 2);
                }
                None => println!("\n{}  Inicializador: None", prefix),
            }
        }
        Stmt::Inclusao { path, is_global } => {
            println!("{}Inclusao:", prefix);
            println!("{}  Path: {}", prefix, path);
            println!("{}  Global: {}", prefix, is_global);
        }
        Stmt::Diretiva(cmd) => {
            println!("{}Diretiva: {}", prefix, cmd);
        }

        Stmt::Bloco { declaracoes } => {
            println!("{}Bloco:", prefix);
            if declaracoes.is_empty() {
                println!("{}  (Vazio)", prefix);
            }
            // chama 'imprimir_stmt' recursivamente para cada declaração dentro do bloco
            for s in declaracoes {
                imprimir_stmt(s, indent + 1);
            }
        }

        Stmt::If { condicao, bloco_then, bloco_else } => {
            println!("{}If:", prefix);
            println!("{}  Condicao:", prefix);
            imprimir_expr(condicao, indent + 2);
            
            println!("{}  Bloco Then:", prefix);
            imprimir_stmt(bloco_then, indent + 2);

            match bloco_else {
                Some(else_stmt) => {
                    println!("{}  Bloco Else:", prefix);

                    imprimir_stmt(else_stmt, indent + 2);
                }
                None => {
                    println!("{}  Bloco Else: None", prefix);
                }
            }
        }

        Stmt::DeclaracaoFuncao { tipo_retorno, nome, corpo } => {
            println!("{}DeclaracaoFuncao:", prefix);
            print!("{}  Tipo Retorno: ", prefix);
            for (i, token) in tipo_retorno.iter().enumerate() {
                if i > 0 { print!(" "); }
                imprimir_token(token);
            }
            print!("\n{}  Nome: ", prefix);
            imprimir_token(nome);
            println!("\n{}  Corpo:", prefix);
            // chama 'imprimir_stmt' recursivamente para o Stmt::Bloco
            imprimir_stmt(corpo, indent + 2);
        }
    }
}
/// imprime recursivamente uma Expressão (Expr)
fn imprimir_expr(expr: &Expr, indent: usize) {
    let prefix = "  ".repeat(indent);
    match expr {
        Expr::Numero(val) => {
            println!("{}Numero({})", prefix, val);
        }
        Expr::Variavel(token) => {
            print!("{}Variavel(", prefix);
            imprimir_token(token);
            println!(")");
        }
        Expr::Atribuicao { nome, valor } => {
            println!("{}Atribuicao:", prefix);
            print!("{}  Nome: ", prefix);
            imprimir_token(nome);
            println!("\n{}  Valor:", prefix);
            imprimir_expr(valor, indent + 2);
        }
        Expr::Agrupamento(inner_expr) => {
            println!("{}Agrupamento:", prefix);
            imprimir_expr(inner_expr, indent + 1);
        }
        Expr::Unario { operador, direita } => {
            println!("{}Unario:", prefix);
            print!("{}  Operador: ", prefix);
            imprimir_token(operador);
            println!("\n{}  Direita:", prefix);
            imprimir_expr(direita, indent + 2);
        }
        Expr::Binario { esquerda, operador, direita } => {
            println!("{}Binario:", prefix);
            print!("{}  Op: ", prefix);
            imprimir_operador(operador);
            println!("\n{}  Esquerda:", prefix);
            imprimir_expr(esquerda, indent + 2);
            println!("{}  Direita:", prefix);
            imprimir_expr(direita, indent + 2);
        }
        Expr::CharLiteral(val) => {
            println!("{}CharLiteral('{}')", prefix, val);
        }
        Expr::StringLiteral(val) => {
            println!("{}StringLiteral(\"{}\")", prefix, val);
        }
        Expr::AtribuicaoComposta { nome, operador, valor } => {
            println!("{}AtribuicaoComposta:", prefix);
            print!("{}  Nome: ", prefix);
            imprimir_token(nome);
            print!("\n{}  Operador: ", prefix);
            imprimir_token(operador);
            println!("\n{}  Valor:", prefix);
            imprimir_expr(valor, indent + 2);
        }
        Expr::Posfixa { expressao, operador } => {
            println!("{}Posfixa:", prefix);
            print!("{}  Operador: ", prefix);
            imprimir_token(operador);
            println!("\n{}  Expressao:", prefix);
            imprimir_expr(expressao, indent + 2);
        }
        Expr::AcessoArray { nome, indice } => {
            println!("{}AcessoArray:", prefix);
            println!("{}  Nome:", prefix);
            imprimir_expr(nome, indent + 2);
            println!("{}  Indice:", prefix);
            imprimir_expr(indice, indent + 2);
        }
        Expr::AtribuicaoArray { nome, indice, valor } => {
            println!("{}AtribuicaoArray:", prefix);
            println!("{}  Nome:", prefix);
            imprimir_expr(nome, indent + 2);
            println!("{}  Indice:", prefix);
            imprimir_expr(indice, indent + 2);
            println!("{}  Valor:", prefix);
            imprimir_expr(valor, indent + 2);
        }
    }
}

/// imprime um Operador (da AST)
fn imprimir_operador(op: &Operador) {
    match op {
        Operador::Mais => print!("Mais"),
        Operador::Menos => print!("Menos"),
        Operador::Asterisco => print!("Asterisco"),
        Operador::Divisao => print!("Divisao"),
        Operador::Comparar => print!("Comparar (==)"),
        Operador::Diferente => print!("Diferente (!=)"),
        Operador::Maior => print!("Maior (>)"),
        Operador::Menor => print!("Menor (<)"),
        Operador::MaiorOuIgual => print!("MaiorOuIgual (>=)"),
        Operador::MenorOuIgual => print!("MenorOuIgual (<=)"),
        Operador::EComercial => print!("EComercial (&)"),
        Operador::Modulo => print!("Modulo (%)"),
    }
}

/// imprime um Token
fn imprimir_token(token: &Token) {
    match token {
        Token::Numero(val) => print!("Numero({})", val),
        Token::Texto(val) => print!("Texto(\"{}\")", val),
        Token::ConteudoChar(val) => print!("ConteudoChar('{}')", val),
        Token::Identificador(val) => print!("Identificador({})", val),
        Token::Mais => print!("Mais"),
        Token::Menos => print!("Menos"),
        Token::Igual => print!("Igual"),
        Token::PontoVirgula => print!("PontoVirgula"),
        Token::AbreParentesis => print!("AbreParentesis"),
        Token::FechaParentesis => print!("FechaParentesis"),
        Token::AbreChave => print!("AbreChave"),
        Token::FechaChave => print!("FechaChave"),
        Token::AbreColchete => print!("AbreColchete"),
        Token::FechaColchete => print!("FechaColchete"),
        Token::Incremento => print!("Incremento"),
        Token::Decremento => print!("Decremento"),
        Token::Soma => print!("Soma"),
        Token::Subtracao => print!("Subtracao"),
        Token::Asterisco => print!("Asterisco"),
        Token::Divisao => print!("Divisao"),
        Token::Modulo => print!("Modulo"),
        Token::EComercial => print!("EComercial"),
        Token::Maior => print!("Maior"),
        Token::Menor => print!("Menos"),
        Token::MaiorOuIgual => print!("MaiorOuIgual"),
        Token::Comparar => print!("Comparar"),
        Token::MenorOuIgual => print!("MenorOuIgual"),
        Token::Diferente => print!("Diferente"),
        Token::Negacao => print!("Negacao"),
        Token::Burro => print!("Burro"),
        Token::Fundo => print!("Fundo"),
        Token::InclusaoGlobal(p) => print!("InclusaoGlobal({})", p),
        Token::InclusaoLocal(p) => print!("InclusaoLocal({})", p),
        Token::Diretiva(c) => print!("Diretiva({})", c),
        Token::QuebraLinha => print!("QuebraDeLinha"),
        Token::Ponto => print!("Ponto"),
    }
}