use crate::parser::{Expr, Operador};

pub fn avaliar(expressao: &Expr) -> f64 {
    match expressao {
        Expr::Numero(valor) => *valor,
        Expr::Binario { esquerda, operador, direita } => {
            let valor_esquerda = avaliar(esquerda);
            let valor_direita = avaliar(direita);

            match operador {
                Operador::Mais => valor_esquerda + valor_direita,
                Operador::Menos => valor_esquerda - valor_direita,
                Operador::Asterisco => valor_esquerda * valor_direita,
                Operador::Divisao => {
                    if valor_direita == 0.0 {
                        panic!("Erro: Divisão por zero");
                    }
                    valor_esquerda / valor_direita
                }
            }
        }
        Expr::Agrupamento(expr) => avaliar(expr),
    }
}