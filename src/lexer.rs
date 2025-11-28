use crate::token::Token; // importa a enumeração Token do módulo token

// estrutura do analisador léxico
pub struct Lexer {
    fonte: Vec<char>,
    posicao: usize,
    caractere_atual: char,
    linha: usize, // Contador de linhas
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
            linha: 1,
        }
    }

    // avança para o próximo caractere na fonte
    pub fn avancar(&mut self) {
        self.posicao += 1;

        if self.caractere_atual == '\n' {
            self.linha += 1;
        }

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
    pub fn ler_numero(&mut self) -> Token {
        let posicao = self.posicao;
        let mut is_float = false;

        // um número começa com um dígito e pode conter mais dígitos e um ponto decimal
        while self.caractere_atual.is_digit(10) {
            self.avancar()
        }

        // verifica se há um ponto decimal seguido por mais dígitos
        if self.caractere_atual == '.' {
            is_float = true;
            self.avancar();

            while self.caractere_atual.is_digit(10) {
                self.avancar()
            }
        }

        let numero_str: String = self.fonte[posicao..self.posicao].iter().collect();

        if is_float {
            Token::NumeroFloat(numero_str)
        } else {
            Token::NumeroInt(numero_str)
        }
    }

    // lê uma string entre aspas
    pub fn ler_texto(&mut self) -> String {
        let mut chars: Vec<char> = Vec::new();

        // lê até encontrar a próxima aspas ou o fim da fonte
        loop {
            self.avancar();

            if self.caractere_atual == '\\' {
                self.avancar();
                match self.caractere_atual {
                    'n' => chars.push('\n'),
                    't' => chars.push('\t'),
                    '"' => chars.push('"'),
                    '\\' => chars.push('\\'),
                    _ => chars.push(self.caractere_atual),
                }
                continue;
            }

            if self.caractere_atual == '"' || self.caractere_atual == '\0' {
                break;
            }

            chars.push(self.caractere_atual)
        }

        // coleta os caracteres do texto e os converte em uma string, retornando o resultado
        return chars.iter().collect();
    }

    // lê um caractere entre aspas simples
    pub fn ler_char(&mut self) -> char {
        self.avancar();

        let c: char;
        if self.caractere_atual == '\\' {
            self.avancar();
            c = match self.caractere_atual {
                'n' => '\n',
                't' => '\t',
                '\'' => '\'',
                '\\' => '\\',
                _ => self.caractere_atual,
            };
            self.avancar();
        } else {
            c = self.caractere_atual;
            self.avancar();
        }

        if self.caractere_atual != '\'' {
            panic!(
                "Erro Léxico na linha {}: Char literal não fechado ou longo demais.",
                self.linha
            );
        }

        return c;
    }

    pub fn ler_diretiva_pre_processador(&mut self) -> Token {
        self.avancar(); // avança para o próximo caractere após '#'

        while self.caractere_atual.is_whitespace() {
            self.avancar();
        }

        let comando = self.ler_identificador();

        if comando == "include" {
            while self.caractere_atual.is_whitespace() {
                self.avancar();
            }

            if self.caractere_atual == '<' {
                let path = self.ler_path_delimitado('>');
                return Token::InclusaoGlobal(path);
            } else if self.caractere_atual == '"' {
                let path = self.ler_path_delimitado('"');
                return Token::InclusaoLocal(path);
            } else {
                return Token::Invalido; // formato inválido de inclusão             
            }
        } else {
            return Token::Diretiva(comando);
        }
    }

    pub fn ler_path_delimitado(&mut self, delimitador: char) -> String {
        let posicao_inicial = self.posicao + 1;

        // lê até encontrar o delimitador de fechamento ou o fim da fonte
        loop {
            self.avancar();
            if self.caractere_atual == delimitador || self.caractere_atual == '\0' {
                break;
            }
        }

        let path: String = self.fonte[posicao_inicial..self.posicao].iter().collect();

        self.avancar(); // avança para o próximo caractere após o delimitador de fechamento

        return path;
    }

    // CORREÇÃO AQUI: prox_token agora gere corretamente o avanço
    pub fn prox_token(&mut self) -> (Token, usize) {
        loop {
            // pula espaços em branco
            while self.caractere_atual.is_whitespace() && self.caractere_atual != '\n' {
                self.avancar();
            }

            let linha_token = self.linha;

            // determina o tipo de token com base no caractere atual
            let token = match self.caractere_atual {
                '\n' => Token::QuebraLinha,
                ';' => Token::PontoVirgula,
                '(' => Token::AbreParentesis,
                ')' => Token::FechaParentesis,
                '[' => Token::AbreColchete,
                ']' => Token::FechaColchete,
                '{' => Token::AbreChave,
                '}' => Token::FechaChave,
                '.' => Token::Ponto,
                ',' => Token::Virgula,

                '\'' => {
                    let conteudo_char = self.ler_char();
                    Token::ConteudoChar(conteudo_char)
                }

                '"' => {
                    let texto = self.ler_texto();
                    Token::Texto(texto)
                }

                '#' => {
                    let t = self.ler_diretiva_pre_processador();
                    // Diretivas já consomem o necessário, retornamos direto
                    return (t, linha_token);
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
                        Token::SomaIgual
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
                        Token::SubtracaoIgual
                    } else {
                        Token::Menos
                    }
                }

                '*' => {
                    if self.espiadinha() == '=' {
                        self.avancar();
                        Token::MultiplicacaoIgual
                    } else {
                        Token::Asterisco
                    }
                }
                '/' => {
                    if self.espiadinha() == '/' {
                        // comentário de linha
                        while self.caractere_atual != '\n' && self.caractere_atual != '\0' {
                            self.avancar();
                        }
                        continue;
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
                        continue;
                    } else if self.espiadinha() == '=' {
                        self.avancar();
                        Token::DivisaoIgual
                    } else {
                        Token::Divisao
                    }
                }

                '%' => {
                    if self.espiadinha() == '=' {
                        self.avancar();
                        Token::ModuloIgual
                    } else {
                        Token::Modulo
                    }
                }
                '&' => {
                    if self.espiadinha() == '&' {
                        self.avancar();
                        Token::EComercialDuplo
                    } else {
                        Token::EComercial
                    }
                }

                '|' => {
                    if self.espiadinha() == '|' {
                        self.avancar();
                        Token::BarraVerticalDupla
                    } else {
                        Token::BarraVertical
                    }
                }

                '>' => {
                    if self.espiadinha() == '=' {
                        self.avancar();
                        Token::MaiorOuIgual
                    } else if self.espiadinha() == '>' {
                        self.avancar();
                        Token::DeslocamentoDir
                    } else {
                        Token::Maior
                    }
                }
                '<' => {
                    if self.espiadinha() == '=' {
                        self.avancar();
                        Token::MenorOuIgual
                    } else if self.espiadinha() == '<' {
                        self.avancar();
                        Token::DeslocamentoEsq
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

                // [IMPORTANTE] Números consomem até o delimitador, então retornamos ANTES do self.avancar() final
                '0'..='9' => {
                    let t = self.ler_numero();
                    return (t, linha_token);
                }

                '\0' => Token::Fundo,

                // [IMPORTANTE] Identificadores consomem até o delimitador, retornamos ANTES do self.avancar() final
                _ => {
                    if self.caractere_atual.is_alphabetic() || self.caractere_atual == '_' {
                        let identificador = self.ler_identificador();
                        return (Token::Identificador(identificador), linha_token);
                    } else {
                        Token::Invalido
                    }
                }
            };

            // Para caracteres simples, strings e operadores, precisamos avançar mais um para consumir o char atual
            self.avancar();

            return (token, linha_token);
        }
    }
}
