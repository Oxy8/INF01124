use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    
    /*
    let stdout = io::stdout();
    let mut buf = BufWriter::with_capacity(32768, stdout);
    */

    for line in stdin.lock().lines() {

        let string: String = line.unwrap();

        let mut input_iter = string.trim().split_whitespace();
        let num_elements: usize = input_iter.next().unwrap().parse().unwrap();
        
        if num_elements == 0 {
            std::process::exit(0);
        } else {
            let mut to_be_sorted: Vec<usize> = Vec::with_capacity(num_elements);
            to_be_sorted = input_iter.map(|n| n.parse::<usize>().unwrap()).collect();

            if magica(to_be_sorted)%2 == 1 {
                println!("Marcelo");
            } else {
                println!("Carlos");
            }
        }
    }
}




fn magica(mut vec: Vec<usize>) -> usize {

    let mut contador: usize = 0;

    for pos in 0..vec.len() {
        
        let expected_number: usize = pos+1;
        // valor do numero esperado no primeiro índice do ciclo que não estava certo.
        // Não precisa ser atualizado a cada loop, só verificar se chegamos nele.
        
        let mut prev_original: usize;
        // Para que depois que realizarmos a troca, a gente saiba qual a próxima posição
        // que deve ser acessada.

        while vec[pos] != expected_number {
            
            //print_u32_vec(&vec);

            prev_original = vec[pos];         
            vec[pos] = vec[prev_original-1];
            vec[prev_original-1] = prev_original;
            // Swap feito.

            // Ao invés de fazer swap de vec[pos] com vec[vec[pos]]

            //println!("pos = {}", pos);
            //println!("vec[pos] = {}", vec[pos]);

            //pos = original_atual-1;
            // pos mantem o mesmo.
        
            contador += 1;
        }
    }

    return contador;
}