use crate::token::Token;

pub struct Lexer {
    fonte: Vec<char>,
    posicao: usize,
    caractere_atual: char,
}

impl Lexer {
    pub fn new(codigo_fonte: String) -> Self {
        let fonte: Vec<char> = codigo_fonte.chars().collect();

        let caractere_atual = if fonte.is_empty() { '\0' } else { fonte[0] };

        Self {
            fonte,
            posicao: 0,
            caractere_atual,
        }
    }

    pub fn avancar(&mut self) {
        if self.posicao + 1 < self.fonte.len() {
            self.posicao += 1;
            self.caractere_atual = self.fonte[self.posicao];
        } else {
            self.caractere_atual = '\0';
        }
    }

    pub fn espiadinha(&self) -> char {
        if self.posicao + 1 < self.fonte.len() {
            self.fonte[self.posicao + 1]
        } else {
            '\0'
        }
    }

    pub fn ler_identificador(&mut self) -> String {

        let posicao = self.posicao;

        while self.caractere_atual.is_alphanumeric() || self.caractere_atual == '_' {

            self.avancar()
        }

        self.fonte[posicao..self.posicao].iter().collect()
    }

    pub fn ler_numero(&mut self) -> String {

        let posicao = self.posicao;

        while self.caractere_atual.is_digit(10) {
            self.avancar()
        }

        if self.caractere_atual == '.' && self.espiadinha().is_digit(10) {
            self.avancar();

            while self.caractere_atual.is_digit(10) {
                self.avancar()
            }
        }


        self.fonte[posicao..self.posicao].iter().collect()
    }

    pub fn ler_texto(&mut self) -> String {

        let posicao_inicial = self.posicao + 1;

        loop {
            self.avancar();
            if self.caractere_atual == '"' || self.caractere_atual == '\0' {
                break;
            }
        }

        self.fonte[posicao_inicial..self.posicao].iter().collect()
    }

    pub fn ler_char(&mut self) -> String {
        let posicao_inicial = self.posicao + 1;

        loop {
            self.avancar();
            if self.caractere_atual == '\'' || self.caractere_atual == '\0' {
                break;
            }
        }

        self.fonte[posicao_inicial..self.posicao].iter().collect()

    }
    pub fn prox_token(&mut self) -> Token {

        while self.caractere_atual.is_whitespace() {
            self.avancar();
        }

        let token = match self.caractere_atual {


            
            ';' => Token::PontoVirgula,
            '(' => Token::AbreParentesis,
            ')' => Token::FechaParentesis,
            '[' => Token::AbreColchete,
            ']' => Token::FechaColchete,

            '{' => Token::AbreChave,
            '}' => Token::FechaChave,

            '\'' => {
                let conteudo_char = self.ler_char();
                Token::ConteudoChar(conteudo_char)
            },

            '"' => {
                let texto = self.ler_texto();
                Token::Texto(texto)
                

            },

            '=' => {
                if self.espiadinha() == '=' {
                    self.avancar();
                    Token::Comparar
                } else {
                    Token::Igual
                }
            },
            
            '+' => {
                if self.espiadinha() == '+' {
                    self.avancar();
                    Token::Incremento
                } else if self.espiadinha() == '=' {
                    self.avancar();
                    Token::Soma
                } else {
                    Token::Mais
                }
            },
            
            '-' => {
                if self.espiadinha() == '-' {
                    self.avancar();
                    Token::Decremento
                } else if self.espiadinha() == '=' {
                    self.avancar();
                    Token::Subtracao
                } else {
                    Token::Menos
                }
            },


            '*' => Token::Multiplicacao,
            '/' => Token::Divisao,
            '%' => Token::Modulo,

            '&' => Token::Referenciador,
            
            '>' => {
                if self.espiadinha() == '=' {
                    self.avancar();
                    Token::MaiorOuIgual
                } else {
                    Token::Maior
                }
            },
            '<' => {
                if self.espiadinha() == '=' {
                    self.avancar();
                    Token::MenorOuIgual
                } else {
                    Token::Menor
                }
            },

            '!' => {
                if self.espiadinha() == '=' {
                    self.avancar();
                    Token::Diferente
                } else {
                    Token::Negacao
                }
            }

            '0' ..='9' => {
                let numero: String = self.ler_numero();
                return Token::Numero(numero);
            }

            '\0' => Token::Fundo,

            _ => {
                if self.caractere_atual.is_alphabetic() || self.caractere_atual == '_' {

                    let identificador =self.ler_identificador();
                    
                    return Token::Identificador(identificador);

                } else {
                    Token::Burro
                }
            }
        };
        
        self.avancar();

        token
    }
}
