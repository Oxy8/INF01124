// Nome: Estevan Küster
// Beecrowd ID: EstevanKuster8

/*
=====================================================================================
A sacada de que é permitido realizar swaps de elementos de posições não
adjacentes pelo fato de que estes podem ser representado por uma quantidade
sempre ímpar de swaps entre elemento adjacentes não é minha, os créditos vão
para o autor do seguinte blog:
https://hsjunior.wordpress.com/2010/10/21/bolhas-e-baldes-jogo-de-ordenar-sequencia/
=====================================================================================
*/


use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        let string: String = line.unwrap();

        let mut input_iter = string.trim().split_whitespace();
        let num_elements: usize = input_iter.next().unwrap().parse().unwrap();
        
        if num_elements == 0 {
            std::process::exit(0);
        } else {
            let mut to_be_sorted = input_iter.map(|n| n.parse::<usize>().unwrap()).collect();

            if quantidade_trocas_impar(&mut to_be_sorted) {
                println!("Marcelo");
            } else {
                println!("Carlos");
            }
        }
    }
}


fn quantidade_trocas_impar(vec: &mut Vec<usize>) -> bool {

    let mut contador: u32 = 0;

    for pos in 1..vec.len() {
        corrige_rec(pos, vec, pos, &mut contador);
    }
    return contador & 1 == 1;
}


/*
Todos os números que se encontrarem fora de suas posições podem
ser representados por um conjunto de ciclos direcionados disjuntos.
Por serem ciclos, sabemos que eventualmente chegaremos ao nosso índice
inicial ao analizar o conteúdo interno do próximo elemento
(Mesma situação de um amigo secreto, sempre fecha ciclos).
*/
fn corrige_rec(pos: usize, vec: &mut Vec<usize>, expected: usize, contador: &mut u32) {
    if vec[pos-1] != expected {
        corrige_rec(vec[pos-1], vec, expected, contador);
        *contador += 1;
    }
    vec[pos-1] = pos;
}
