use crate::token::Token;

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
}
