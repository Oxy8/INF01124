// Originalmente compilado com rustc 1.78.0-nightly (878c8a2a6 2024-02-29)

#![feature(iter_collect_into)]

use std::fs::{read_to_string, write};
use std::fmt::{Write, Display};
use std::time::Instant;

fn main() {
    // Le arquivo de entrada e coloca conteúdo em String
    let input_buffer = read_to_string("./entradas/entrada2.txt").unwrap();

    // Cria String para onde a saída será escrita.
    let mut saida_hoare_mediana = String::new();
    let mut saida_hoare_aleatorio = String::new();
    let mut saida_lomuto_mediana = String::new();
    let mut saida_lomuto_aleatorio = String::new();
    

    for input_line in input_buffer.lines() {
        
        let mut input_iter = input_line.trim().split_whitespace();
        let num_elements: usize = input_iter.next().unwrap().parse().unwrap();
        let mut to_be_sorted: Vec<u32> = Vec::with_capacity(num_elements);
        input_iter.map(|n| n.parse::<u32>().unwrap()).collect_into(&mut to_be_sorted);


        benchmark_quicksort(&mut to_be_sorted.clone(), quicksort_hoare, get_mo3_partitioner, saida_hoare_mediana);
        benchmark_quicksort(&mut to_be_sorted.clone(), quicksort_hoare, get_random_partitioner, saida_hoare_aleatorio);
        benchmark_quicksort(&mut to_be_sorted.clone(), quicksort_lomuto, get_mo3_partitioner, saida_lomuto_mediana);
        benchmark_quicksort(&mut to_be_sorted.clone(), quicksort_lomuto, get_random_partitioner, saida_lomuto_aleatorio);
        
    }

    // Escreve String de saída para o arquivo de saída.
    write("./saidas/stats-mediana-hoare.txt", saida_hoare_mediana).unwrap();
    write("./saidas/stats-aleatorio-hoare.txt", saida_hoare_aleatorio).unwrap();
    write("./saidas/stats-mediana-lomuto.txt", saida_lomuto_mediana).unwrap();
    write("./saidas/stats-aleatorio-lomuto.txt", saida_lomuto_aleatorio).unwrap();

}


fn benchmark_quicksort<T: Ord + Copy + Display>(vec: &mut [T], partitioning_scheme_f: fn(&mut [T], &mut u32, &mut u32, fn(&[T]) -> usize), partitioner_element_f: fn(&[T]) -> usize, string_saida: &mut String) {
    let vec_length = vec.len();
    
    // Inicio do timer.
    let timer = Instant::now();
    
    let mut n_swaps: u32 = 0;
    let mut n_recursions: u32 = 0;
    //-------------------------------------

    partitioning_scheme_f(vec, &mut n_swaps, &mut n_recursions, partitioner_element_f);

    //-------------------------------------
    // Fim do timer.
    let exec_time: u128 = timer.elapsed().as_nanos(); // Tempo em nanosegundos.
    let exec_time_millis: f64 = exec_time as f64/1_000_000.0; // Tempo em milissegundos

    // Escreve resultado para a String de saida.
    writeln!(string_saida, "TAMANHO ENTRADA {vec_length}\nSWAPS {n_swaps}\nRECURSOES {n_recursions}\nTEMPO {exec_time_millis:.6}").unwrap();
}

// swap do particionador é contabilizado.

fn quicksort_hoare<T: Ord + Copy>(vec: &mut [T], n_swaps: &mut u32, n_recursoes: &mut u32, partitioner_element_f: fn(&[T]) -> usize) {
    *n_recursoes += 1;
    
}


fn quicksort_lomuto<T: Ord + Copy>(vec: &mut [T], n_swaps: &mut u32, n_recursoes: &mut u32, partitioner_element_f: fn(&[T]) -> usize) {
    *n_recursoes += 1;

}

fn swap_w_counter() {

}



fn get_random_partitioner<T: Ord + Copy>(vec: &[T]) {

}


fn get_mo3_partitioner<T: Ord + Copy>(vec: &[T]) {

}





/*
---TEMPOS PROFESSOR--- (ms)

Tempos Hoare:
0.005
0.012
0.12
1.74
16
149

Tempos Lomuto:
0.004
0.014
0.13
1.31
14
181

*/