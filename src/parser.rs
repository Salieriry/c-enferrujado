use core::panic;

use crate::token::Token;

#[derive(Clone, Debug)]
pub struct Parametro {
    pub tipo: Vec<Token>,
    pub nome: Token,
    pub tamanho_array: Option<Expr>,
}

#[derive(Clone, Debug)]
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
}

#[derive(Clone, Debug)]
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

    CharLiteral(char),
    StringLiteral(String),
    ArrayDim,
}
#[derive(Debug)]
pub enum Stmt {
    Expressao(Expr),

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

            Token::Identificador(nome) if nome == "return" => {
                let operador = self.token_atual.clone();
                self.avancar();

                let direita = self.parse_atribuicao();

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
                    panic!("Esperado ')', mas foi recebido {:?}", self.token_atual);
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
            Token::Identificador(_) => Expr::Variavel(self.token_atual.clone()),
            _ => {
                if self.token_atual == Token::Burro {
                    panic!(
                        "Erro de Léxico: Caractere ilegal encontrado pelo lexer. Token: {:?}",
                        self.token_atual
                    );
                } else {
                    panic!(
                        "Erro de Sintaxe: Fator inesperado. Esperado num, '(', var ou op. unário, mas foi recebido {:?}",
                        self.token_atual
                    );
                }
            }
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
                        panic!(
                            "Esperando ']' após o índice do array, mas foi recebido {:?}",
                            self.token_atual
                        )
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
        let mut expr = self.parse_expressao();

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
                _ => panic!(
                    "Erro de Sintaxe: Alvo inválido para atribuição. A expressão da esquerda é: {:?}",
                    expr_esquerda
                ),
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
                        operador: operador,
                        valor: Box::new(valor),
                    };
                }
                _ => panic!(
                    "Erro de Sintaxe: Alvo inválido para atribuição composta. A expressão da esquerda é: {:?}",
                    expr_esquerda
                ),
            }
        }

        expr_esquerda
    }

    pub fn parse_declaracao(&mut self) -> Stmt {
        match &self.token_atual {
            Token::InclusaoGlobal(_) | Token::InclusaoLocal(_) => self.parse_diretiva_inclusao(),
            Token::Diretiva(_) => self.parse_diretiva_outra(),
            Token::Identificador(nome) if nome == "if" => {
                return self.parse_declaracao_if();
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
            panic!(
                "Esperado '(' após 'if', mas foi recebido {:?}",
                self.token_atual
            );
        }
        self.avancar();

        let condicao = self.parse_atribuicao();

        if self.token_atual != Token::FechaParentesis {
            panic!(
                "Esperado ')' após condição do 'if', mas foi recebido {:?}",
                self.token_atual
            );
        }
        self.avancar();

        let bloco_then = self.parse_bloco();

        let mut bloco_else: Option<Box<Stmt>> = None;

        if let Token::Identificador(nome) = &self.token_atual {
            if nome == "else" {
                self.avancar();

                if let Token::Identificador(nome) = &self.token_atual {
                    if nome == "if" {
                        bloco_else = Some(Box::new(self.parse_declaracao_if()));
                    } else {
                        bloco_else = Some(Box::new(self.parse_bloco()));
                    }
                } else {
                    bloco_else = Some(Box::new(self.parse_bloco()));
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
            panic!(
                "Esperado '{{' para iniciar o bloco, mas foi recebido {:?}",
                self.token_atual
            );
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
            panic!(
                "Esperado '}}' para fechar o bloco, mas foi recebido {:?}",
                self.token_atual
            );
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
            panic!(
                "Esperado nome da função após o tipo de retorno, mas foi recebido {:?}",
                self.token_atual
            );
        }
        let nome = self.token_atual.clone();
        self.avancar();

        if self.token_atual != Token::AbreParentesis {
            panic!(
                "Esperado '(' após o nome da função, mas foi recebido {:?}",
                self.token_atual
            );
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
                    panic!(
                        "Esperado nome do parâmetro na declaração da função, mas foi recebido {:?}",
                        self.token_atual
                    );
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
                        panic!(
                            "Esperado ']' em parâmetro de array, mas foi recebido {:?}",
                            self.token_atual
                        );
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
                    panic!(
                        "Esperado ',' ou ')' após parâmetro de função, mas foi recebido {:?}",
                        self.token_atual
                    );
                }
            }
        }

        if self.token_atual != Token::FechaParentesis {
            panic!(
                "Esperado ')' após os parâmetros da função, mas foi recebido {:?}",
                self.token_atual
            );
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
            panic!(
                "Esperado nome de variável após o tipo, mas foi recebido {:?}",
                self.token_atual
            );
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
                panic!(
                    "Esperado ']' após tamanho do array, mas foi recebido {:?}",
                    self.token_atual
                );
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
            panic!(
                "Esperado ';' após declaração de variável, mas foi recebido {:?}",
                self.token_atual
            );
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
            panic!(
                "Esperado ';' após expressão, mas foi recebido {:?}",
                self.token_atual
            );
        }
        self.avancar();

        Stmt::Expressao(expr)
    }
}
