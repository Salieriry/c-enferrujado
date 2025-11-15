use crate::token::Token; // importa a enumeração Token do módulo token

// estrutura do analisador léxico
pub struct Lexer {
    fonte: Vec<char>,
    posicao: usize,
    caractere_atual: char,
}

// implementação dos métodos do analisador léxico
impl Lexer {
    // construtor do analisador léxico
    pub fn new(codigo_fonte: String) -> Self {
        let fonte: Vec<char> = codigo_fonte.chars().collect(); // converte a string em um vetor de caracteres

        let caractere_atual = if fonte.is_empty() { '\0' } else { fonte[0] }; // caractere atual ou nulo se a fonte estiver vazia

        // inicializa o analisador léxico
        Self {
            fonte,
            posicao: 0,
            caractere_atual,
        }
    }

    // avança para o próximo caractere na fonte
    pub fn avancar(&mut self) {
        self.posicao += 1;

        if self.posicao < self.fonte.len() {
            self.caractere_atual = self.fonte[self.posicao];
        } else {
            self.caractere_atual = '\0';
        }
    }

    // espiadinha retorna o próximo caractere sem avançar a posição
    pub fn espiadinha(&self) -> char {
        if self.posicao + 1 < self.fonte.len() {
            self.fonte[self.posicao + 1]
        } else {
            '\0'
        }
    }

    // lê um identificador (nome de variável, função, etc.)
    pub fn ler_identificador(&mut self) -> String {
        let posicao = self.posicao;

        // um identificador começa com uma letra ou sublinhado e pode conter letras, dígitos ou sublinhados
        while self.caractere_atual.is_alphanumeric() || self.caractere_atual == '_' {
            self.avancar() // avança para o próximo caractere
        }

        // coleta os caracteres do identificador e os converte em uma string
        self.fonte[posicao..self.posicao].iter().collect()
    }

    // lê um número (inteiro ou ponto flutuante)
    pub fn ler_numero(&mut self) -> String {
        let posicao = self.posicao;

        // um número começa com um dígito e pode conter mais dígitos e um ponto decimal
        while self.caractere_atual.is_digit(10) {
            self.avancar()
        }

        // verifica se há um ponto decimal seguido por mais dígitos
        if self.caractere_atual == '.' && self.espiadinha().is_digit(10) {
            self.avancar();

            while self.caractere_atual.is_digit(10) {
                self.avancar()
            }
        }

        // coleta os caracteres do número e os converte em uma string
        self.fonte[posicao..self.posicao].iter().collect()
    }

    // lê uma string entre aspas
    pub fn ler_texto(&mut self) -> String {
        let posicao_inicial = self.posicao + 1;

        // lê até encontrar a próxima aspas ou o fim da fonte
        loop {
            self.avancar();
            if self.caractere_atual == '"' || self.caractere_atual == '\0' {
                break;
            }
        }

        // coleta os caracteres do texto e os converte em uma string, retornando o resultado
        self.fonte[posicao_inicial..self.posicao].iter().collect()
    }

    // lê um caractere entre aspas simples
    pub fn ler_char(&mut self) -> String {
        let posicao_inicial = self.posicao + 1;

        // lê até encontrar a próxima aspas simples ou o fim da fonte
        loop {
            self.avancar();
            if self.caractere_atual == '\'' || self.caractere_atual == '\0' {
                break;
            }
        }

        // coleta os caracteres do char e os converte em uma string, retornando o resultado
        self.fonte[posicao_inicial..self.posicao].iter().collect()
    }

    // obtém o próximo token da fonte
    pub fn prox_token(&mut self) -> Token {
        // pula espaços em branco
        while self.caractere_atual.is_whitespace() {
            self.avancar();
        }

        // determina o tipo de token com base no caractere atual
        let token = match self.caractere_atual {
            // símbolos de pontuação
            ';' => Token::PontoVirgula,
            '(' => Token::AbreParentesis,
            ')' => Token::FechaParentesis,
            '[' => Token::AbreColchete,
            ']' => Token::FechaColchete,
            '#' => Token::Hashtag,
            '{' => Token::AbreChave,
            '}' => Token::FechaChave,

            // caractere entre aspas simples
            '\'' => {
                let conteudo_char = self.ler_char();
                Token::ConteudoChar(conteudo_char)
            }

            // string entre aspas duplas
            '"' => {
                let texto = self.ler_texto();
                Token::Texto(texto)
            }

            // operadores e símbolos
            '=' => {
                if self.espiadinha() == '=' {
                    self.avancar();
                    Token::Comparar
                } else {
                    Token::Igual
                }
            }

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
            }

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
            }

            '*' => Token::Asterisco,

            '/' => {
                if self.espiadinha() == '/' {
                    // comentário de linha
                    while self.caractere_atual != '\n' && self.caractere_atual != '\0' {
                        self.avancar();
                    }
                    return self.prox_token(); // chama recursivamente para obter o próximo token após o comentário
                } else if self.espiadinha() == '*' {
                    // comentário de bloco
                    self.avancar(); // avança para o '*'
                    self.avancar(); // avança para o próximo caractere após '*'
                    while !(self.caractere_atual == '*' && self.espiadinha() == '/')
                        && self.caractere_atual != '\0'
                    {
                        self.avancar();
                    }
                    if self.caractere_atual == '*' && self.espiadinha() == '/' {
                        self.avancar(); // avança para o '*'
                        self.avancar(); // avança para o '/'
                    }
                    return self.prox_token(); // chama recursivamente para obter o próximo token após o comentário
                } else {
                    Token::Divisao
                }
            }

            '%' => Token::Modulo,

            '&' => Token::EComercial,

            // operadores de comparação
            '>' => {
                if self.espiadinha() == '=' {
                    self.avancar();
                    Token::MaiorOuIgual
                } else {
                    Token::Maior
                }
            }
            '<' => {
                if self.espiadinha() == '=' {
                    self.avancar();
                    Token::MenorOuIgual
                } else {
                    Token::Menor
                }
            }

            '!' => {
                if self.espiadinha() == '=' {
                    self.avancar();
                    Token::Diferente
                } else {
                    Token::Negacao
                }
            }

            // números (inteiros e ponto flutuante)
            '0'..='9' => {
                let numero: String = self.ler_numero();
                return Token::Numero(numero);
            }

            '\0' => Token::Fundo,

            // identificadores (nomes de variáveis, funções, etc.)
            _ => {
                if self.caractere_atual.is_alphabetic() || self.caractere_atual == '_' {
                    let identificador = self.ler_identificador(); // lê o identificador

                    return Token::Identificador(identificador); // retorna o token de identificador
                } else {
                    Token::Burro // caractere desconhecido
                }
            }
        };

        self.avancar(); // avança para o próximo caractere após reconhecer o token

        token
    }
}
