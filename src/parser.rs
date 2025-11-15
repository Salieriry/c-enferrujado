
use core::panic;

use crate::token::Token;
#[derive(Debug)]
pub enum Operador {
    Mais,
    Menos,
    Asterisco,
    Divisao,
}
#[derive(Debug)]
pub enum Expr {
    Numero(f64),
    Binario {
        esquerda: Box<Expr>,
        operador: Operador,
        direita: Box<Expr>,
    },

    Agrupamento(Box<Expr>),
}

pub enum Stmt {
    Expressao(Expr),

    DeclaracaoVariavel{
        tipo: Token,
        nome: Token,
        inicializador: Option<Expr>,

    }
}

pub struct Parser {
    tokens: Vec<Token>,
    posicao_atual: usize,
    token_atual: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let token_atual = if tokens.is_empty() {
            Token::Fundo
        } else {
            tokens[0].clone()
        };

        Self {
            tokens,
            posicao_atual: 0,
            token_atual,
        }
    }

    pub fn avancar(&mut self) {
        if self.posicao_atual + 1 < self.tokens.len() {
            self.posicao_atual += 1;
            self.token_atual = self.tokens[self.posicao_atual].clone();
        } else {
            self.token_atual = Token::Fundo;
        }
    }

    fn espiadinha(&self) -> Token {
        if self.posicao_atual + 1 < self.tokens.len() {
            self.tokens[self.posicao_atual + 1].clone()
        } else {
            Token::Fundo
        }
    }

    pub fn parse_fator(&mut self) -> Expr {
        match self.token_atual.clone() {
            Token::Numero(valor_string) => {
                let valor = valor_string.parse::<f64>().unwrap();
                self.avancar();
                Expr::Numero(valor)
            }
            Token::AbreParentesis => {
                self.avancar();
                let expr = self.parse_expressao();
                if let Token::FechaParentesis = self.token_atual {
                    self.avancar();
                } else {
                    panic!("Esperado ')'");
                }
                Expr::Agrupamento(Box::new(expr))
            }
            _ => panic!("Esperado número ou '('"),
        }
    }

    pub fn parse_termo(&mut self) -> Expr {
        let mut expr = self.parse_fator();

        while let Token::Asterisco | Token::Divisao = self.token_atual {
            let operador = match self.token_atual.clone() {
                Token::Asterisco => Operador::Asterisco,
                Token::Divisao => Operador::Divisao,
                _ => unreachable!(),
            };
            self.avancar();
            let direita = self.parse_fator();
            expr = Expr::Binario {
                esquerda: Box::new(expr),
                operador,
                direita: Box::new(direita),
            };
        }
        expr
    }

    pub fn parse_expressao(&mut self) -> Expr {
        let mut expr = self.parse_termo();

        while let Token::Mais | Token::Menos = self.token_atual {
            let operador = match self.token_atual.clone() {
                Token::Mais => Operador::Mais,
                Token::Menos => Operador::Menos,
                _ => unreachable!(),
            };
            self.avancar();
            let direita = self.parse_termo();
            expr = Expr::Binario {
                esquerda: Box::new(expr),
                operador,
                direita: Box::new(direita),
            };
        }
        expr
    }

    pub fn parse_declaracao(&mut self) -> Stmt {
        let expr = self.parse_expressao();
        if self.token_atual != Token::PontoVirgula {
            panic!("Esperado ';' após a expressão");
        }
        self.avancar();

        Stmt::Expressao(expr)
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut declaracoes: Vec<Stmt> = Vec::new();

        while self.token_atual != Token::Fundo {
            declaracoes.push(self.parse_declaracao());
        }

        declaracoes
    }
}
