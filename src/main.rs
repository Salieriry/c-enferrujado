use std::ffi::c_char;
use std::io;

mod token;

use crate::token::Token;

pub struct Lexer {
    fonte: Vec<char>,
    posicao: usize,
    caractere_atual: char,


}


fn main() {

    let mut code_block = String::new();

    io::stdin().read_line(&mut code_block).unwrap();

    println!("O valor da string é: {code_block}");

}

fn lexical_analysis(input: String) {
    let mut state: u8;
    state = 0;
    let mut i: usize;
    i = 0;

    let mut c: char;

    while 1 == 1 {
       match state{
           0 => {
               c = input[i];
               match c {




           }

        }
       }

    }
}


