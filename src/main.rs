// Todos os modulos devem ser declarados no main.rs
// mesmo se eles não forem utilizados no main.rs
mod token;
mod lexer;

use crate::lexer::Lexer;
use crate::token::Token;



fn main() {
    let codigo = "let valor = 10.5 + ¨ (2*5); != ++--";

    let mut lexer = Lexer::new(codigo.to_string());

    println!("Analisando o código \"{}\"\n", codigo);

    loop {
        let token = lexer.prox_token();
        println!("Token -> {:?}", token);

        if let Token::Fundo = token {
            break;
        }
    }
}
