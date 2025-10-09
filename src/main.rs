// todos os modulos devem ser declarados no main.rs
// mesmo se eles não forem utilizados no main.rs
mod lexer;
mod token; // declara o módulo token // declara o módulo lexer

use crate::lexer::Lexer; // importa a estrutura Lexer do módulo lexer
use crate::token::Token; // importa a enumeração Token do módulo token

// função principal do programa
fn main() {
    // código fonte de exemplo para o analisador léxico
    let codigo = 
    "#include <iostream>

    // Função principal para testar o lexer.
    int main() {
        // Declaração de variáveis com diferentes tipos de literais
        int contador = 10;
        float saldo_conta = 150.75;
        char inicial = 'C';
        const char* mensagem = \"Olá, mundo! Teste 123.\";

        // Operadores de atribuição e incremento/decremento
        contador += 5;
        contador++;
        --saldo_conta;

        // Estrutura de controle com operadores de comparação
        if (contador >= 15 & saldo_conta < 200.0) {
            int resultado = (contador * 2) % 3;
            int arr[2];
            arr[0] = resultado;
        }

        // Verificação de outros operadores e pontuação
        bool sao_diferentes = (1 != 2);
        int* ponteiro = &contador;

        // Teste de caractere ilegal
        $

        return 0;
        }"; 

    let mut lexer = Lexer::new(codigo.to_string()); // cria um novo analisador léxico

    println!("Analisando o código \"{}\"\n", codigo); // imprime o código fonte

    // loop para obter e imprimir todos os tokens até o token de fim de arquivo (Fundo)
    loop {
        let token = lexer.prox_token();
        println!("Token -> {:?}", token);

        if let Token::Fundo = token {
            break;
        }
    }
}
