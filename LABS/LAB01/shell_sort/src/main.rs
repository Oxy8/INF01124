// Originalmente compilado com rustc 1.78.0-nightly (878c8a2a6 2024-02-29)

#![feature(iter_collect_into)]

use std::fmt::Display;
use std::fs::{read_to_string, write};
use std::fmt::Write;
use std::time::Instant;

static SHELL: [u32; 21] = [1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192,16384,32768,65536,131072,262144,524288,1048576];
static KNUTH: [u32; 14] = [1,4,13,40,121,364,1093,3280,9841,29524,88573,265720,797161,2391484];
static CIURA: [u32; 17] = [1,4,10,23,57,132,301,701,1577,3548,7983,17961,40412,90927,204585,460316,1035711];

static SPECS: &str = "2.1 GHz Quad-Core Intel(R) Core(TM) i7-3612QM";

fn main() {
    testes1();
    testes2();
}

fn testes1() {
    // Le arquivo de entrada e coloca conteúdo em String
    let input_buffer = read_to_string("./entradas/entrada1.txt").unwrap();

    // Cria String para onde a saída será escrita.
    let mut output_buffer = String::new();
    

    for input_line in input_buffer.lines() {
        
        let mut input_iter = input_line.trim().split_whitespace();
        let num_elements: usize = input_iter.next().unwrap().parse().unwrap();
        let mut to_be_sorted: Vec<u32> = Vec::with_capacity(num_elements);
        input_iter.map(|n| n.parse::<u32>().unwrap()).collect_into(&mut to_be_sorted);

        let mut to_be_sorted_copy = to_be_sorted.clone();
        shell_sort_write(&mut to_be_sorted_copy, &SHELL, "SHELL", &mut output_buffer);

        let mut to_be_sorted_copy = to_be_sorted.clone();
        shell_sort_write(&mut to_be_sorted_copy, &KNUTH, "KNUTH", &mut output_buffer);
        
        let mut to_be_sorted_copy = to_be_sorted.clone();
        shell_sort_write(&mut to_be_sorted_copy, &CIURA, "CIURA", &mut output_buffer);
        
    }

    // Escreve String de saída para o arquivo de saída.
    write("./saidas/saida1.txt", output_buffer).unwrap();
}

fn testes2() {
    // Le arquivo de entrada e coloca conteúdo em String
    let input_buffer = read_to_string("./entradas/entrada2.txt").unwrap();

    // Cria String para onde a saída será escrita.
    let mut output_buffer = String::new();
    

    for input_line in input_buffer.lines() {
        
        let mut input_iter = input_line.trim().split_whitespace();
        let num_elements: usize = input_iter.next().unwrap().parse().unwrap();
        let mut to_be_sorted: Vec<u32> = Vec::with_capacity(num_elements);
        input_iter.map(|n| n.parse::<u32>().unwrap()).collect_into(&mut to_be_sorted);

        let mut to_be_sorted_copy = to_be_sorted.clone();
        shell_sort(&mut to_be_sorted_copy, &SHELL, "SHELL", &mut output_buffer);

        let mut to_be_sorted_copy = to_be_sorted.clone();
        shell_sort(&mut to_be_sorted_copy, &KNUTH, "KNUTH", &mut output_buffer);
        
        let mut to_be_sorted_copy = to_be_sorted.clone();
        shell_sort(&mut to_be_sorted_copy, &CIURA, "CIURA", &mut output_buffer);
        
    }

    // Escreve String de saída para o arquivo de saída.
    write("./saidas/saida2.txt", output_buffer).unwrap();
}

// Escreve os resultados para a String passada como argumento.
// A escrita ocorre durante a execução do algoritmo, portanto, não serve para o benchmark.
//
fn shell_sort_write<T: Ord + Copy + Display>(vec: &mut [T], sequence: &[u32], sequence_name: &str, string_saida: &mut String) {
    
    let vec_length: usize = vec.len();

    // Obtem índice de maior valor da sequência, este será utilizado no ordenamento.
    let seq_index: usize = sequence.iter().position(|&x| x >= vec_length as u32).unwrap();

    // Imprime sequência antes de se iniciar o ordenamento junto com o nome do sequência
    for i in 0..vec_length { write!(string_saida, "{} ", vec[i]).unwrap() };
    writeln!(string_saida, "SEQ={}", sequence_name).unwrap();

    // Imprime sequência a cada passada do algoritmo, informando o 'h' utilizado.
    for h in (0..seq_index).rev() {
        
        h_insertion_sort(vec, sequence[h] as usize);
        
        for i in 0..vec_length { write!(string_saida, "{} ", vec[i]).unwrap() };
        writeln!(string_saida, "INCR={}", sequence[h]).unwrap();
    }

}

fn shell_sort<T: Ord + Copy + Display>(vec: &mut [T], sequence: &[u32], sequence_name: &str, string_saida: &mut String) {
    
    // Inicio do timer.
    let instant = Instant::now();
    //-------------------------------------

    let vec_length: usize = vec.len();

    // Obtem índice de maior valor da sequência, este será utilizado no ordenamento.
    let seq_index: usize = sequence.iter().position(|&x| x >= vec_length as u32).unwrap();

    for h in (0..seq_index).rev() {
        h_insertion_sort(vec, sequence[h] as usize);
    }

    //-------------------------------------
    // Fim do timer.
    let tempo_exec: u128 = instant.elapsed().as_nanos(); // Tempo em nanosegundos.
    let tempo_exec_millis: f64 = tempo_exec as f64/1_000_000.0; // Tempo em milissegundos

    // Escreve resultado para a String de saida.
    writeln!(string_saida, "{sequence_name},{vec_length},{tempo_exec_millis:.6},{SPECS}").unwrap();

}


fn h_insertion_sort<T: Ord + Copy>(vec: &mut [T], h: usize) {

    let vec_length: usize = vec.len();

    // Estabelece diferentes offsets para que possamos atingir todos os elementos do vetor.
    for offset in 0..h {

        // Coleta os elementos de h em h.
        for mut walk_index in (offset..vec_length).step_by(h) {
            
            let number: T = vec[walk_index];

            while walk_index >= h && vec[walk_index-h] > number  {
                vec[walk_index] = vec[walk_index-h];
                walk_index -= h;
            }
            
            vec[walk_index] = number;
        }
    }
}

