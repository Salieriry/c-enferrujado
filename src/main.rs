mod lexer;
mod token;
mod parser;


use crate::lexer::Lexer;
use crate::token::Token;
use crate::parser::{Expr, Operador, Parser, Stmt};


fn main() {
    // código de teste que usa declaração, atribuição e expressões
    let codigo = "int x = 5 + 10;\nfloat y;\nx = x * 2;";
    
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
        imprimir_stmt(stmt, 1); // Começa com indentação nível 1
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
        Stmt::DeclaracaoVariavel { tipo, nome, inicializador } => {
            println!("{}DeclaracaoVariavel:", prefix);
            print!("{}  Tipo: ", prefix);
            imprimir_token(tipo);
            print!("\n{}  Nome: ", prefix);
            imprimir_token(nome);
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
        Expr::Binario { esquerda, operador, direita } => {
            println!("{}Binario:", prefix);
            print!("{}  Op: ", prefix);
            imprimir_operador(operador);
            println!("\n{}  Esquerda:", prefix);
            imprimir_expr(esquerda, indent + 2);
            println!("{}  Direita:", prefix);
            imprimir_expr(direita, indent + 2);
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