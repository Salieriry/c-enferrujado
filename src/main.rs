// Todos os modulos devem ser declarados no main.rs
// mesmo se eles não forem utilizados no main.rs
mod token;
mod lexer;
use lexer::Lexer;

use std::io;



fn main() {
    let mut code_block = String::new();

    io::stdin().read_line(&mut code_block).unwrap();

    println!("O valor da string é: {code_block}");
}
