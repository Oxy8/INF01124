
/*

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
            let mut to_be_sorted: Vec<usize> = Vec::with_capacity(num_elements);
            to_be_sorted = input_iter.map(|n| n.parse::<usize>().unwrap()).collect();

            if quantidade_trocas_impar(to_be_sorted) {
                println!("Marcelo");
            } else {
                println!("Carlos");
            }
        }
    }
}


fn quantidade_trocas_impar(mut vec: Vec<usize>) -> bool {

    let mut contador: usize = 0;

    for pos in 0..vec.len() {
        
        let expected_number: usize = pos+1;
        // valor do numero esperado no primeiro índice do ciclo que não estava certo.
        // Não precisa ser atualizado a cada loop, só verificar se chegamos nele.
        
        let mut prev_original: usize;

        /*
        Todos os números que se encontrarem fora de suas posições podem
        ser representados por um conjunto de ciclos orientados disjuntos.
        Por serem ciclos, sabemos que eventualmente chegaremos ao nosso índice
        inicial ao analizar o conteúdo interno de outro elemento no mesmo ciclo.
        (Mesma situação de um amigo secreto).
         */
        while vec[pos] != expected_number {
            
            prev_original = vec[pos];         
            vec[pos] = vec[prev_original-1];
            vec[prev_original-1] = prev_original;
        
            contador += 1;
        }
    }
    return contador & 1 == 1;
}

/*
        let mut new_pos: usize;
        
        /*
        Todos os números que se encontrarem fora de suas posições podem
        ser representados por um conjunto de ciclos orientados disjuntos.
        Por serem ciclos, sabemos que eventualmente chegaremos ao nosso índice
        inicial ao analizar o conteúdo interno de outro elemento no mesmo ciclo.
        (Mesma situação de um amigo secreto).
        */

        while vec[pos] != expected_number && vec[pos] != 0 {
            new_pos = vec[pos]-1;
            vec[pos] = 0;
            pos = new_pos;
        
            contador += 1;
        }
        //  \\
        // gera menos acessos por end, mas não sei se compensa.
*/