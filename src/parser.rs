use core::panic;

use crate::token::Token;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Parametro {
    pub tipo: Vec<Token>,
    pub nome: Token,
    pub tamanho_array: Option<Expr>,
}

#[derive(Clone, Debug, Serialize)]
pub enum Operador {
    Mais,
    Menos,
    Asterisco,
    Divisao,
    Comparar,
    Maior,
    Menor,
    MaiorOuIgual,
    MenorOuIgual,
    Diferente,
    Modulo,
    EComercial,
    EComercialDuplo,
    BarraVertical,
    BarraVerticalDupla,
    DeslocamentoEsq,
    DeslocamentoDir,
}

#[derive(Clone, Debug, Serialize)]
pub enum Expr {
    NumeroInt(i64),
    NumeroFloat(f64),
    Binario {
        esquerda: Box<Expr>,
        operador: Operador,
        direita: Box<Expr>,
    },
    Agrupamento(Box<Expr>),
    Variavel(Token),
    Atribuicao {
        alvo: Box<Expr>,
        valor: Box<Expr>,
    },
    AtribuicaoComposta {
        alvo: Box<Expr>,
        operador: Token,
        valor: Box<Expr>,
    },
    Unario {
        operador: Token,
        direita: Box<Expr>,
    },
    Posfixa {
        expressao: Box<Expr>,
        operador: Token,
    },
    AcessoArray {
        nome: Box<Expr>,
        indice: Box<Expr>,
    },
    ChamadaFuncao {
        callee: Box<Expr>,
        argumentos: Vec<Expr>,
    },
    CharLiteral(char),
    StringLiteral(String),
    ArrayDim,
}

#[derive(Debug, Serialize)]
pub enum Stmt {
    Expressao(Expr),
    Retorno(Option<Expr>),
    DeclaracaoVariavel {
        tipo: Vec<Token>,
        nome: Token,
        tamanho_array: Option<Expr>,
        inicializador: Option<Expr>,
    },
    Inclusao {
        path: String,
        is_global: bool,
    },
    Diretiva(String),
    DeclaracaoFuncao {
        tipo_retorno: Vec<Token>,
        nome: Token,
        parametros: Vec<Parametro>,
        corpo: Box<Stmt>,
    },
    If {
        condicao: Expr,
        bloco_then: Box<Stmt>,
        bloco_else: Option<Box<Stmt>>,
    },
    Bloco {
        declaracoes: Vec<Stmt>,
    },
    Using {
        namespace: String,
    },
}

pub struct Parser {
    tokens: Vec<(Token, usize)>,
    posicao_atual: usize,
    token_atual: Token,
    linha_atual: usize, 
}

impl Parser {
    pub fn new(tokens: Vec<(Token, usize)>) -> Self {
        let (token_atual, linha_atual) = if tokens.is_empty() {
            (Token::Fundo, 0)
        } else {
            tokens[0].clone()
        };

        Self {
            tokens,
            posicao_atual: 0,
            token_atual,
            linha_atual,
        }
    }

    pub fn avancar(&mut self) {
        if self.posicao_atual + 1 < self.tokens.len() {
            self.posicao_atual += 1;
            let (tok, lin) = self.tokens[self.posicao_atual].clone();
            self.token_atual = tok;
            self.linha_atual = lin;
        } else {
            self.token_atual = Token::Fundo;
        }
    }

    fn espiadinha(&self) -> Token {
        if self.posicao_atual + 1 < self.tokens.len() {
            self.tokens[self.posicao_atual + 1].0.clone()
        } else {
            Token::Fundo
        }
    }

    fn espiar_dois_passos(&self) -> Token {
        if self.posicao_atual + 2 < self.tokens.len() {
            self.tokens[self.posicao_atual + 2].0.clone()
        } else {
            Token::Fundo
        }
    }

    fn erro(&self, mensagem: String) -> ! {
        panic!("Erro na linha {}: {}", self.linha_atual, mensagem);
    }

    pub fn parse_primario(&mut self) -> Expr {
        match &self.token_atual {
            Token::Menos
            | Token::Decremento
            | Token::Incremento
            | Token::Negacao
            | Token::EComercial
            | Token::Asterisco => {
                let operador = self.token_atual.clone();
                self.avancar();
                let direita = self.parse_primario();
                return Expr::Unario {
                    operador,
                    direita: Box::new(direita),
                };
            }

            Token::AbreParentesis => {
                self.avancar();
                let expr = self.parse_atribuicao();
                if let Token::FechaParentesis = self.token_atual {
                    self.avancar();
                } else {
                    self.erro(format!(
                        "Esperado ')', mas foi recebido {:?}",
                        self.token_atual
                    ));
                }
                return Expr::Agrupamento(Box::new(expr));
            }

            _ => {}
        }

        let expr = match &self.token_atual {
            Token::NumeroInt(valor_string) => {
                let valor = valor_string.parse::<i64>().unwrap();
                Expr::NumeroInt(valor)
            }

            Token::NumeroFloat(valor_string) => {
                let valor = valor_string.parse::<f64>().unwrap();
                Expr::NumeroFloat(valor)
            }

            Token::ConteudoChar(valor_char) => Expr::CharLiteral(*valor_char),

            Token::Texto(valor_string) => Expr::StringLiteral(valor_string.to_string()),

            Token::Identificador(_) => {
                if self.espiadinha() == Token::AbreParentesis {
                    let nome = self.token_atual.clone();
                    self.avancar();

                    self.avancar();

                    let mut argumentos = Vec::new();

                    if self.token_atual != Token::FechaParentesis {
                        loop {
                            argumentos.push(self.parse_atribuicao());
                            if self.token_atual == Token::Virgula {
                                self.avancar();
                            } else {
                                break;
                            }
                        }
                    }

                    if self.token_atual != Token::FechaParentesis {
                        self.erro(format!(
                            "esperado ')' após argumentos, recebido {:?}",
                            self.token_atual
                        ));
                    }

                    Expr::ChamadaFuncao {
                        callee: Box::new(Expr::Variavel(nome)),
                        argumentos,
                    }
                } else {
                    Expr::Variavel(self.token_atual.clone())
                }
            }

            _ => self.erro(format!(
                "Esperado primário, recebido {:?}",
                self.token_atual
            )),
        };

        self.avancar();
        expr
    }

    pub fn parse_fator(&mut self) -> Expr {
        let mut expr = self.parse_primario();

        loop {
            match &self.token_atual {
                Token::Incremento | Token::Decremento => {
                    let operador_posfixo = self.token_atual.clone();
                    self.avancar();
                    expr = Expr::Posfixa {
                        expressao: Box::new(expr),
                        operador: operador_posfixo,
                    };
                }

                Token::AbreColchete => {
                    self.avancar();
                    let indice = self.parse_atribuicao();
                    if self.token_atual != Token::FechaColchete {
                        self.erro(format!(
                            "Esperando ']' após o índice do array, mas foi recebido {:?}",
                            self.token_atual
                        ));
                    }
                    self.avancar();
                    expr = Expr::AcessoArray {
                        nome: Box::new(expr),
                        indice: Box::new(indice),
                    }
                }

                _ => break,
            }
        }
        expr
    }

    pub fn parse_termo(&mut self) -> Expr {
        let mut expr = self.parse_fator();

        while let Token::Asterisco | Token::Divisao | Token::Modulo = &self.token_atual {
            let operador = match &self.token_atual {
                Token::Asterisco => Operador::Asterisco,
                Token::Divisao => Operador::Divisao,
                Token::Modulo => Operador::Modulo,
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

        while let Token::Mais | Token::Menos = &self.token_atual {
            let operador = match &self.token_atual {
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

    pub fn parse_bitwise_and(&mut self) -> Expr {
        let mut expr = self.parse_comparacao();

        while let Token::EComercial = &self.token_atual {
            let operador = match &self.token_atual {
                Token::EComercial => Operador::EComercial,
                _ => unreachable!(),
            };
            self.avancar();
            let direita = self.parse_comparacao();
            expr = Expr::Binario {
                esquerda: Box::new(expr),
                operador,
                direita: Box::new(direita),
            };
        }
        expr
    }

    pub fn parse_bitwise_or(&mut self) -> Expr {
        let mut expr = self.parse_bitwise_and();

        while let Token::BarraVertical = &self.token_atual {
            let operador = Operador::BarraVertical;
            self.avancar();
            let direita = self.parse_bitwise_and();
            expr = Expr::Binario {
                esquerda: Box::new(expr),
                operador,
                direita: Box::new(direita),
            };
        }
        expr
    }

    pub fn parse_logical_and(&mut self) -> Expr {
        let mut expr = self.parse_bitwise_or();

        while let Token::EComercialDuplo = &self.token_atual {
            let operador = Operador::EComercialDuplo;
            self.avancar();
            let direita = self.parse_bitwise_or();
            expr = Expr::Binario {
                esquerda: Box::new(expr),
                operador,
                direita: Box::new(direita),
            };
        }
        expr
    }

    pub fn parse_logical_or(&mut self) -> Expr {
        let mut expr = self.parse_logical_and();

        while let Token::BarraVerticalDupla = &self.token_atual {
            let operador = Operador::BarraVerticalDupla;
            self.avancar();
            let direita = self.parse_logical_and();
            expr = Expr::Binario {
                esquerda: Box::new(expr),
                operador,
                direita: Box::new(direita),
            };
        }
        expr
    }

    pub fn parse_comparacao(&mut self) -> Expr {
        let mut expr = self.parse_shift();

        while let Token::Maior
        | Token::Menor
        | Token::MaiorOuIgual
        | Token::MenorOuIgual
        | Token::Comparar
        | Token::Diferente = &self.token_atual
        {
            let operador = match &self.token_atual {
                Token::Maior => Operador::Maior,
                Token::Menor => Operador::Menor,
                Token::MaiorOuIgual => Operador::MaiorOuIgual,
                Token::MenorOuIgual => Operador::MenorOuIgual,
                Token::Comparar => Operador::Comparar,
                Token::Diferente => Operador::Diferente,
                _ => unreachable!(),
            };
            self.avancar();
            let direita = self.parse_shift();
            expr = Expr::Binario {
                esquerda: Box::new(expr),
                operador,
                direita: Box::new(direita),
            };
        }
        expr
    }

    pub fn parse_shift(&mut self) -> Expr {
        let mut expr = self.parse_expressao();

        while let Token::DeslocamentoEsq | Token::DeslocamentoDir = &self.token_atual {
            let operador = match &self.token_atual {
                Token::DeslocamentoEsq => Operador::DeslocamentoEsq,
                Token::DeslocamentoDir => Operador::DeslocamentoDir,
                _ => unreachable!(),
            };
            self.avancar();
            let direita = self.parse_expressao();
            expr = Expr::Binario {
                esquerda: Box::new(expr),
                operador,
                direita: Box::new(direita),
            };
        }
        expr
    }

    pub fn parse_atribuicao(&mut self) -> Expr {
        let expr_esquerda = self.parse_logical_or();

        if self.token_atual == Token::Igual {
            self.avancar();
            let valor = self.parse_atribuicao();

            match expr_esquerda {
                Expr::Variavel(_) | Expr::AcessoArray { .. } | Expr::Unario { .. } => {
                    return Expr::Atribuicao {
                        alvo: Box::new(expr_esquerda),
                        valor: Box::new(valor),
                    };
                }
                _ => self.erro(format!("Erro de Sintaxe: Alvo inválido para atribuição. A expressão da esquerda é: {:?}", expr_esquerda)),
            }
        } else if let Token::SomaIgual
        | Token::SubtracaoIgual
        | Token::MultiplicacaoIgual
        | Token::DivisaoIgual
        | Token::ModuloIgual = &self.token_atual
        {
            let operador = self.token_atual.clone();
            self.avancar();
            let valor = self.parse_atribuicao();

            match expr_esquerda {
                Expr::Variavel(_) | Expr::AcessoArray { .. } | Expr::Unario { .. } => {
                    return Expr::AtribuicaoComposta {
                        alvo: Box::new(expr_esquerda),
                        operador,
                        valor: Box::new(valor),
                    };
                }
                _ => self.erro(format!("Erro de Sintaxe: Alvo inválido para atribuição composta. A expressão da esquerda é: {:?}", expr_esquerda)),
            }
        }
        expr_esquerda
    }

    pub fn parse_declaracao(&mut self) -> Stmt {
        match &self.token_atual {
            Token::InclusaoGlobal(_) | Token::InclusaoLocal(_) => self.parse_diretiva_inclusao(),

            Token::Diretiva(_) => self.parse_diretiva_outra(),

            Token::AbreChave => self.parse_bloco(),

            Token::Identificador(nome) if nome == "if" => {
                return self.parse_declaracao_if();
            }

            Token::Identificador(nome) if nome == "return" => {
                self.avancar();

                let valor = if self.token_atual == Token::PontoVirgula {
                    None
                } else {
                    Some(self.parse_atribuicao())
                };

                if self.token_atual != Token::PontoVirgula {
                    self.erro("esperado ';' após return".to_string());
                }
                self.avancar();
                return Stmt::Retorno(valor);
            }

            Token::Identificador(nome) if nome == "using" => {
                self.avancar();

                if let Token::Identificador(ns_kw) = &self.token_atual {
                    if ns_kw != "namespace" {
                        self.erro("Esperado 'namespace' após 'using'".to_string());
                    }
                } else {
                    self.erro("Esperado 'namespace' após 'using'".to_string());
                }

                self.avancar();

                let namespace_nome = if let Token::Identificador(nome) = &self.token_atual {
                    nome.clone()
                } else {
                    self.erro("Esperado nome do namespace".to_string());
                    unreachable!()
                };

                self.avancar();

                if self.token_atual != Token::PontoVirgula {
                    self.erro("esperado ';' após using namespace".to_string());
                }

                self.avancar();

                return Stmt::Using {
                    namespace: namespace_nome,
                };
            }

            Token::Identificador(_) => {
                if let Token::Identificador(_) = self.espiadinha() {
                    if let Token::AbreParentesis = self.espiar_dois_passos() {
                        self.parse_declaracao_funcao()
                    } else {
                        self.parse_declaracao_variavel()
                    }
                } else if let Token::Asterisco = self.espiadinha() {
                    self.parse_declaracao_variavel()
                } else {
                    self.parse_declaracao_expressao()
                }
            }

            _ => self.parse_declaracao_expressao(),
        }
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

    pub fn parse_declaracao_if(&mut self) -> Stmt {
        self.avancar();

        if self.token_atual != Token::AbreParentesis {
            self.erro(format!(
                "Esperado '(' após 'if', mas foi recebido {:?}",
                self.token_atual
            ));
        }

        self.avancar();
        let condicao = self.parse_atribuicao();

        if self.token_atual != Token::FechaParentesis {
            self.erro(format!(
                "Esperado ')' após condição do 'if', mas foi recebido {:?}",
                self.token_atual
            ));
        }

        self.avancar();
        let bloco_then = self.parse_declaracao();

        let mut bloco_else: Option<Box<Stmt>> = None;

        if let Token::Identificador(nome) = &self.token_atual {
            if nome == "else" {
                self.avancar();
                if let Token::Identificador(nome) = &self.token_atual {
                    if nome == "if" {
                        bloco_else = Some(Box::new(self.parse_declaracao_if()));
                    } else {
                        bloco_else = Some(Box::new(self.parse_declaracao()));
                    }
                } else {
                    bloco_else = Some(Box::new(self.parse_declaracao()));
                }
            }
        }

        Stmt::If {
            condicao,
            bloco_then: Box::new(bloco_then),
            bloco_else,
        }
    }

    pub fn parse_bloco(&mut self) -> Stmt {
        if self.token_atual != Token::AbreChave {
            self.erro(format!(
                "Esperado '{{' para iniciar o bloco, mas foi recebido {:?}",
                self.token_atual
            ));
        }

        self.avancar();
        let mut declaracoes: Vec<Stmt> = Vec::new();

        while self.token_atual != Token::FechaChave && self.token_atual != Token::Fundo {
            if self.token_atual == Token::QuebraLinha {
                self.avancar();
                continue;
            }
            declaracoes.push(self.parse_declaracao());
        }

        if self.token_atual != Token::FechaChave {
            self.erro(format!(
                "Esperado '}}' para fechar o bloco, mas foi recebido {:?}",
                self.token_atual
            ));
        }

        self.avancar();
        Stmt::Bloco { declaracoes }
    }

    pub fn parse_diretiva_outra(&mut self) -> Stmt {
        let comando = if let Token::Diretiva(cmd) = &self.token_atual {
            cmd.clone()
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
        let mut tipo_retorno: Vec<Token> = Vec::new();

        while self.token_atual != Token::Fundo {
            if let Token::Identificador(_) = self.token_atual {
                if let Token::AbreParentesis = self.espiadinha() {
                    break;
                }
            }
            tipo_retorno.push(self.token_atual.clone());
            self.avancar();
        }

        if !matches!(self.token_atual, Token::Identificador(_)) {
            self.erro(format!(
                "Esperado nome da função após o tipo de retorno, mas foi recebido {:?}",
                self.token_atual
            ));
        }

        let nome = self.token_atual.clone();
        self.avancar();

        if self.token_atual != Token::AbreParentesis {
            self.erro(format!(
                "Esperado '(' após o nome da função, mas foi recebido {:?}",
                self.token_atual
            ));
        }

        self.avancar();
        let mut parametros: Vec<Parametro> = Vec::new();

        if self.token_atual != Token::FechaParentesis {
            loop {
                let mut tipo_param: Vec<Token> = Vec::new();
                let mut tamanho_array: Option<Expr> = None;

                while self.token_atual != Token::Fundo {
                    if let Token::Identificador(_) = self.token_atual {
                        if let Token::Virgula | Token::FechaParentesis | Token::AbreColchete =
                            self.espiadinha()
                        {
                            break;
                        }
                    }
                    tipo_param.push(self.token_atual.clone());
                    self.avancar();
                }

                if !matches!(self.token_atual, Token::Identificador(_)) {
                    self.erro(format!(
                        "Esperado nome do parâmetro na declaração da função, mas foi recebido {:?}",
                        self.token_atual
                    ));
                }

                let nome_param = self.token_atual.clone();
                self.avancar();

                if self.token_atual == Token::AbreColchete {
                    self.avancar();
                    if self.token_atual == Token::FechaColchete {
                        tamanho_array = Some(Expr::ArrayDim);
                    } else {
                        tamanho_array = Some(self.parse_atribuicao());
                    }

                    if self.token_atual != Token::FechaColchete {
                        self.erro(format!(
                            "Esperado ']' em parâmetro de array, mas foi recebido {:?}",
                            self.token_atual
                        ));
                    }
                    self.avancar();
                }

                parametros.push(Parametro {
                    tipo: tipo_param,
                    nome: nome_param,
                    tamanho_array,
                });

                if self.token_atual == Token::Virgula {
                    self.avancar();
                    continue;
                }

                if self.token_atual == Token::FechaParentesis {
                    break;
                } else {
                    self.erro(format!(
                        "Esperado ',' ou ')' após parâmetro de função, mas foi recebido {:?}",
                        self.token_atual
                    ));
                }
            }
        }

        if self.token_atual != Token::FechaParentesis {
            self.erro(format!(
                "Esperado ')' após os parâmetros da função, mas foi recebido {:?}",
                self.token_atual
            ));
        }

        self.avancar();
        while self.token_atual == Token::QuebraLinha {
            self.avancar();
        }

        let corpo = self.parse_bloco();

        Stmt::DeclaracaoFuncao {
            tipo_retorno,
            nome,
            parametros,
            corpo: Box::new(corpo),
        }
    }

    pub fn parse_declaracao_variavel(&mut self) -> Stmt {
        let mut tipo: Vec<Token> = Vec::new();

        while self.token_atual != Token::Fundo {
            if let Token::Identificador(_) = self.token_atual {
                if let Token::Igual | Token::PontoVirgula | Token::AbreColchete = self.espiadinha()
                {
                    break;
                }
            }
            tipo.push(self.token_atual.clone());
            self.avancar();
        }

        if !matches!(self.token_atual, Token::Identificador(_)) {
            self.erro(format!(
                "Esperado nome de variável após o tipo, mas foi recebido {:?}",
                self.token_atual
            ));
        }

        let nome = self.token_atual.clone();
        self.avancar();

        let tamanho_array: Option<Expr>;

        if self.token_atual == Token::AbreColchete {
            self.avancar();
            if self.token_atual == Token::FechaColchete {
                tamanho_array = Some(Expr::ArrayDim);
            } else {
                tamanho_array = Some(self.parse_atribuicao());
            }

            if self.token_atual != Token::FechaColchete {
                self.erro(format!(
                    "Esperado ']' após tamanho do array, mas foi recebido {:?}",
                    self.token_atual
                ));
            }
            self.avancar();
        } else {
            tamanho_array = None;
        }

        let inicializador: Option<Expr>;

        if self.token_atual == Token::Igual {
            self.avancar();
            inicializador = Some(self.parse_atribuicao());
        } else {
            inicializador = None;
        }

        if self.token_atual != Token::PontoVirgula {
            self.erro(format!(
                "Esperado ';' após declaração de variável, mas foi recebido {:?}",
                self.token_atual
            ));
        }

        self.avancar();

        Stmt::DeclaracaoVariavel {
            tipo,
            nome,
            tamanho_array,
            inicializador,
        }
    }

    pub fn parse_declaracao_expressao(&mut self) -> Stmt {
        let expr = self.parse_atribuicao();

        if self.token_atual != Token::PontoVirgula {
            self.erro(format!(
                "Esperado ';' após expressão, mas foi recebido {:?}",
                self.token_atual
            ));
        }

        self.avancar();
        Stmt::Expressao(expr)
    }
}
