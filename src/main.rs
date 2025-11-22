mod lexer;
mod parser;
mod token;

use crate::lexer::Lexer;
use crate::parser::{Expr, Operador, Parser, Stmt};
use crate::token::Token;

// este código foi feito para testar todas as funcionalidades da parte léxica e sintática
// do compilador
const CODIGO_COMPLETO: &str = r#"
// --- Diretivas de Pré-processador ---
#include <stdio.h>
#include "meu_projeto.h"
#define VERSAO 1.0

/*
 \* Comentário em bloco de múltiplas linhas
 \* para testar o lexer.
 */

// --- Declaração de Função ---
// Testa tipos de parâmetros: ponteiro, array vazio, array com tamanho
int calcular(int* p_valor, float dados[], int indices[10]) {
    
    // --- Declarações de Variáveis e Literais ---
    int a = 10;
    float b = 20.5;
    char c = 'Z';
    char newline = '\n'; // Char com escape
    const char* s = "Olá \"mundo\"!"; // String com escape

    // --- Declaração de Array e Ponteiro ---
    int meu_array[5];
    int* p_a = &a; // Op Unário (&)
    int array_vazio[]; // Declaração de array vazio

    // --- Atribuições (L-values) ---
    *p_a = 50; // Atribuição em ponteiro
    meu_array[0] = 1; // Atribuição em array (AcessoArray)
    a = a + 1;

    // --- Operadores Compostos e Inc/Dec ---
    a += 5;     // Atribuição Composta
    b -= 1.0;
    a *= 2;
    b /= 2.0;
    a %= 3;
    a++;        // Pósfixa
    --a;        // Prefixa (Unário)

    // --- Estrutura if-else if-else ---
    if (a > 10 && b < 30.0) {
        return 1;
    } else if (a == 10 || !true) {
        return 2;
    } else {
        // Operadores bitwise (convertidos de 0b101 para 5, etc.)
        int x = (5 & 3) | 8;
    }

    return 0;
}

// --- Função Principal ---
int main() {
    // Expressão Pura (Stmt::Expressao)
    10; 
    
    // Precedência de Operadores
    int y = 10 + 2 * 3; // 16
    int z = (10 + 2) * 3; // 36

    return 0;
}
"#;

// 2. NOVA FUNÇÃO MAIN
fn main() {
    println!("Analisando o seguinte código-fonte:\n");
    println!("{}", CODIGO_COMPLETO);
    println!("--------------------------------------------");

    // --- 1. Fase Léxica (Lexer) ---
    println!("\n============================================");
    println!("  FASE 1: ANÁLISE LÉXICA (Tokens)");
    println!("============================================");

    let mut lexer = Lexer::new(CODIGO_COMPLETO.to_string());
    let mut tokens: Vec<Token> = Vec::new();

    let mut token_count = 0;
    loop {
        let token = lexer.prox_token();
        let is_fundo = matches!(token, Token::Fundo);

        // formata a impressão dos tokens
        if token == Token::QuebraLinha {
            println!();
            token_count = 0;
            tokens.push(token);
            continue;
        }

        if is_fundo {
            tokens.push(token);
            break;
        }

        // imprime o token
        print!("[");
        imprimir_token(&token);
        print!("] ");

        token_count += 1;
        if token_count > 5 {
            // quebra a linha a cada 6 tokens
            println!();
            token_count = 0;
        }

        tokens.push(token);
    }

    // --- 2. Fase Sintática (Parser) ---
    println!("\n\n============================================");
    println!("  FASE 2: ANÁLISE SINTÁTICA (AST)");
    println!("============================================");

    let mut parser = Parser::new(tokens);
    let ast_programa = parser.parse();

    println!("\n--- SAÍDA DE DEBUG DA AST (RAW) ---");
    println!("{:#?}", ast_programa);
    println!("--- FIM DA SAÍDA DE DEBUG ---\n");

    println!("\n--- IMPRESSÃO FORMATADA DA AST ---");
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
        Stmt::Retorno(expr_opt) => {
            print!("return");
            if let Some(expr) = expr_opt {
                print!(" ");
                // Chama a função que imprime/avalia a expressão
                imprimir_expr(expr, indent + 1);
            }
            println!(";");
        }
        Stmt::DeclaracaoVariavel {
            tipo,
            nome,
            tamanho_array,
            inicializador,
        } => {
            println!("{}DeclaracaoVariavel:", prefix);
            print!("{}  Tipo: ", prefix);
            for (i, token) in tipo.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
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

        Stmt::If {
            condicao,
            bloco_then,
            bloco_else,
        } => {
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

        Stmt::DeclaracaoFuncao {
            tipo_retorno,
            nome,
            parametros,
            corpo,
        } => {
            println!("{}DeclaracaoFuncao:", prefix);

            // imprime tipo de Retorno
            print!("{}  Tipo Retorno: ", prefix);
            for (i, token) in tipo_retorno.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                imprimir_token(token);
            }

            // imprime Nome da Função
            print!("\n{}  Nome: ", prefix);
            imprimir_token(nome);

            println!("\n{}  Parâmetros:", prefix);
            if parametros.is_empty() {
                println!("{}    (Vazio)", prefix);
            } else {
                let param_prefix = "  ".repeat(indent + 2);
                for (idx, param) in parametros.iter().enumerate() {
                    print!("{}    [{}]: ", prefix, idx);

                    // imprime tipo do param
                    for (i, token) in param.tipo.iter().enumerate() {
                        if i > 0 {
                            print!(" ");
                        }
                        imprimir_token(token);
                    }

                    print!(" ");
                    imprimir_token(&param.nome);
                    println!(); // Nova linha

                    if let Some(tamanho) = &param.tamanho_array {
                        println!("{}      Array Size:", param_prefix);
                        imprimir_expr(tamanho, indent + 4);
                    }
                }
            }

            println!("{}  Corpo:", prefix);
            imprimir_stmt(corpo, indent + 2);
        }
    }
}
/// imprime recursivamente uma Expressão (Expr)
fn imprimir_expr(expr: &Expr, indent: usize) {
    let prefix = "  ".repeat(indent);
    match expr {
        Expr::NumeroInt(val) => {
            println!("{}Numero Inteiro({})", prefix, val);
        }
        Expr::NumeroFloat(val) => {
            println!("{}Numero Float({})", prefix, val);
        }
        Expr::Variavel(token) => {
            print!("{}Variavel(", prefix);
            imprimir_token(token);
            println!(")");
        }
        Expr::Atribuicao { alvo, valor } => {
            println!("{}Atribuicao:", prefix);
            println!("{}  Alvo:", prefix);
            imprimir_expr(alvo, indent + 2);
            println!("{}  Valor:", prefix);
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
        Expr::Binario {
            esquerda,
            operador,
            direita,
        } => {
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
        Expr::AtribuicaoComposta {
            alvo,
            operador,
            valor,
        } => {
            println!("{}AtribuicaoComposta:", prefix);
            println!("{}  Alvo:", prefix);
            imprimir_expr(alvo, indent + 2);
            print!("\n{}  Operador: ", prefix);
            imprimir_token(operador);
            println!("\n{}  Valor:", prefix);
            imprimir_expr(valor, indent + 2);
        }
        Expr::Posfixa {
            expressao,
            operador,
        } => {
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
        Expr::ArrayDim => {
            println!("{}ArrayDim ( [] )", prefix);
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
        Operador::EComercialDuplo => print!("EComercialDuplo (&&)"),
        Operador::Modulo => print!("Modulo (%)"),
        Operador::BarraVertical => print!("BarraVertical (|)"),
        Operador::BarraVerticalDupla => print!("BarraVerticalDupla (||)"),
    }
}

/// imprime um Token
fn imprimir_token(token: &Token) {
    match token {
        Token::NumeroInt(val) => print!("Numero Inteiro ({})", val),
        Token::NumeroFloat(val) => print!("Numero Float ({})", val),
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
        Token::SomaIgual => print!("SomaIgual"),
        Token::SubtracaoIgual => print!("SubtracaoIgual"),
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
        Token::Invalido => print!("Invalido"),
        Token::Fundo => print!("Fundo"),
        Token::InclusaoGlobal(p) => print!("InclusaoGlobal({})", p),
        Token::InclusaoLocal(p) => print!("InclusaoLocal({})", p),
        Token::Diretiva(c) => print!("Diretiva({})", c),
        Token::QuebraLinha => print!("QuebraDeLinha"),
        Token::Ponto => print!("Ponto"),
        Token::Virgula => print!("Vírgula",),
        Token::MultiplicacaoIgual => print!("MultiplicacaoIgual"),
        Token::DivisaoIgual => print!("DivisaoIgual"),
        Token::ModuloIgual => print!("ModuloIgual"),
        Token::EComercialDuplo => print!("EComercialDuplo"),
        Token::BarraVertical => print!("BarraVertical"),
        Token::BarraVerticalDupla => print!("BarraVerticalDupla"),
    }
}
