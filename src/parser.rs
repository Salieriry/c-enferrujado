use core::panic;

use crate::token::Token;

pub enum Operador {
    Mais,
    Menos,
    Asterisco,
    Divisao,
}

pub enum Expr {
    Numero(f64),
    Binario {
        esquerda: Box<Expr>,
        operador: Operador,
        direita: Box<Expr>,
    },

    Agrupamento(Box<Expr>),

    Variavel(Token),

    Atribuicao {
        nome: Token,
        valor: Box<Expr>,
    },
}

pub enum Stmt {
    Expressao(Expr),

    DeclaracaoVariavel {
        tipo: Token,
        nome: Token,
        inicializador: Option<Expr>,
    },
    Inclusao {
        path: String,
        is_global: bool,
    },
    Diretiva(String),

    DeclaracaoFuncao {
        tipo_retorno: Token,
        nome: Token,
    },
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

    fn espiar_dois_passos(&self) -> Token {
        if self.posicao_atual + 2 < self.tokens.len() {
            self.tokens[self.posicao_atual + 2].clone()
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
            Token::Identificador(_) => {
                let var_token = self.token_atual.clone();
                self.avancar();
                Expr::Variavel(var_token)
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

    pub fn parse_atribuicao(&mut self) -> Expr {
        let expr_esquerda = self.parse_expressao();

        if self.token_atual == Token::Igual {
            self.avancar();
            let valor = self.parse_atribuicao();

            if let Expr::Variavel(var_nome) = expr_esquerda {
                return Expr::Atribuicao {
                    nome: var_nome,
                    valor: Box::new(valor),
                };
            } else {
                panic!("Erro de Sintaxe: Alvo inválido para atribuição.");
            }
        }

        expr_esquerda
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut declaracoes: Vec<Stmt> = Vec::new();

        while self.token_atual != Token::Fundo {
            if self.token_atual == Token::QuebraLinha {
                self.avancar();
                continue;
            }
            declaracoes.push(self.parse_declaracao());
        }

        declaracoes
    }

    pub fn parse_declaracao(&mut self) -> Stmt {
        match &self.token_atual {
            Token::InclusaoGlobal(_) | Token::InclusaoLocal(_) => self.parse_diretiva_inclusao(),
            Token::Diretiva(_) => self.parse_diretiva_outra(),
            Token::Identificador(_) => {
                if let Token::Identificador(_) = self.espiadinha() {
                    if let Token::AbreParentesis = self.espiar_dois_passos() {
                        self.parse_declaracao_funcao()
                    } else {
                        self.parse_declaracao_variavel()
                    }
                } else {
                    self.parse_declaracao_expressao()
                }
            }
            _ => self.parse_declaracao_expressao(),
        }
    }

    pub fn parse_diretiva_inclusao(&mut self) -> Stmt {
        let token_clonado = self.token_atual.clone();

        self.avancar();

        match token_clonado {
            Token::InclusaoGlobal(path) => Stmt::Inclusao {
                path,
                is_global: true,
            },
            Token::InclusaoLocal(path) => Stmt::Inclusao {
                path,
                is_global: false,
            },
            _ => unreachable!(),
        }
    }

    pub fn parse_diretiva_outra(&mut self) -> Stmt {
        let comando = if let Token::Diretiva(cmd) = self.token_atual.clone() {
            cmd
        } else {
            unreachable!()
        };

        self.avancar();

        while self.token_atual != Token::Fundo && self.token_atual != Token::QuebraLinha {
            self.avancar();
        }

        Stmt::Diretiva(comando)
    }

    pub fn parse_declaracao_funcao(&mut self) -> Stmt {
        let tipo_retorno = self.token_atual.clone();
        self.avancar();

        let nome = self.token_atual.clone();
        self.avancar();

        if self.token_atual != Token::AbreParentesis {
            panic!("Esperado '(' após o nome da função.");
        }
        self.avancar();

        while self.token_atual != Token::FechaParentesis && self.token_atual != Token::Fundo {
            self.avancar();
        }

        if self.token_atual != Token::FechaParentesis {
            panic!("Esperado ')' após os parâmetros da função.");
        }
        self.avancar();

        if self.token_atual != Token::AbreChave {
            panic!("Esperava '{{' para iniciar o corpo da função");
        }
        self.avancar();

        while self.token_atual != Token::FechaChave && self.token_atual != Token::Fundo {
            self.avancar();
        }

        if self.token_atual != Token::FechaChave {
             panic!("Esperava '}}' para fechar o corpo da função");
        }
        self.avancar();

        Stmt::DeclaracaoFuncao {
            tipo_retorno,
            nome
        }
    }

    pub fn parse_declaracao_variavel(&mut self) -> Stmt {
        let tipo = self.token_atual.clone();
        self.avancar();

        let nome = self.token_atual.clone();
        self.avancar();

        let inicializador: Option<Expr>;
        if self.token_atual == Token::Igual {
            self.avancar();
            inicializador = Some(self.parse_expressao());
        } else {
            inicializador = None;
        }

        if self.token_atual != Token::PontoVirgula {
            panic!("Esperado ';' após declaração de variável.");
        }
        self.avancar();

        Stmt::DeclaracaoVariavel {
            tipo,
            nome,
            inicializador,
        }
    }

    pub fn parse_declaracao_expressao(&mut self) -> Stmt {
        let expr = self.parse_atribuicao();

        if self.token_atual != Token::PontoVirgula {
            panic!("Esperado ';' após expressão.");
        }
        self.avancar();

        Stmt::Expressao(expr)
    }
}
