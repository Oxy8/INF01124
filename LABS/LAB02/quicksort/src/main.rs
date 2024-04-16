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


        benchmark_quicksort(&mut to_be_sorted.clone(), quicksort_hoare, get_mo3_pivot, &mut saida_hoare_mediana);
        benchmark_quicksort(&mut to_be_sorted.clone(), quicksort_hoare, get_random_pivot, &mut saida_hoare_aleatorio);
        benchmark_quicksort(&mut to_be_sorted.clone(), quicksort_lomuto, get_mo3_pivot, &mut saida_lomuto_mediana);
        benchmark_quicksort(&mut to_be_sorted.clone(), quicksort_lomuto, get_random_pivot, &mut saida_lomuto_aleatorio);
        
    }

    // Escreve String de saída para o arquivo de saída.
    write("./saidas/stats-mediana-hoare.txt", saida_hoare_mediana).unwrap();
    write("./saidas/stats-aleatorio-hoare.txt", saida_hoare_aleatorio).unwrap();
    write("./saidas/stats-mediana-lomuto.txt", saida_lomuto_mediana).unwrap();
    write("./saidas/stats-aleatorio-lomuto.txt", saida_lomuto_aleatorio).unwrap();

}


fn benchmark_quicksort<T: Ord + Copy + Display>(vec: &mut [T], partitioning_scheme_f: fn(&mut [T], &mut u32, fn(&[T]) -> usize) -> usize, pivot_selector_f: fn(&[T]) -> usize, string_saida: &mut String) {
    let vec_length = vec.len();
    
    // Inicio do timer.
    let timer = Instant::now();
    
    let mut n_swaps: u32 = 0;
    let mut n_recursions: u32 = 0;
    //-------------------------------------

    quicksort(vec, partitioning_scheme_f, pivot_selector_f, &mut n_swaps, &mut n_recursions);
    
    //-------------------------------------
    // Fim do timer.
    let exec_time: u128 = timer.elapsed().as_nanos(); // Tempo em nanosegundos.
    let exec_time_millis: f64 = exec_time as f64/1_000_000.0; // Tempo em milissegundos

    // Escreve resultado para a String de saida.
    writeln!(string_saida, "TAMANHO ENTRADA {vec_length}\nSWAPS {n_swaps}\nRECURSOES {n_recursions}\nTEMPO {exec_time_millis:.6}").unwrap();
}

fn quicksort<T: Ord + Copy>(vec: &mut [T], partitioning_scheme_f: fn(&mut [T], &mut u32, fn(&[T]) -> usize) -> usize, pivot_selector_f: fn(&[T]) -> usize, n_swaps: &mut u32, n_recursions: &mut u32) {
    *n_recursions += 1;
  // Maybe if vec.len() > 0 ?? need it to behave as in the c code.
  if vec.len() > 1 {

    let pivot_index: usize = partitioning_scheme_f(vec, n_swaps, pivot_selector_f);
    
    quicksort(&mut vec[..pivot_index], partitioning_scheme_f, pivot_selector_f, n_swaps, n_recursions);
    quicksort(&mut vec[pivot_index+1..], partitioning_scheme_f, pivot_selector_f, n_swaps, n_recursions);
  }
}


// retorno é o índice do elemento particionador utilizado.
fn quicksort_hoare<T: Ord + Copy>(vec: &mut [T], n_swaps: &mut u32, pivot_selector_f: fn(&[T]) -> usize) -> usize {

    let pivot_index = pivot_selector_f(vec);

    loop {
        let left: usize = vec.iter().position(|&x| x > vec[pivot_index]).unwrap();
        let right: usize = vec.iter().rposition(|&x| x < vec[pivot_index]).unwrap(); 

        if left >= right {
            break;
        }
        else {
            vec.swap(left, right);
            *n_swaps += 1;
        }

    }
    return pivot_index;    
}

// retorno é o índice do elemento particionador utilizado.
fn quicksort_lomuto<T: Ord + Copy>(vec: &mut [T], n_swaps: &mut u32, pivot_selector_f: fn(&[T]) -> usize) -> usize {
    
    let pivot = pivot_selector_f(vec);
    // Como que funciona a mediana de 3? Tem que mover o elemento para o começo da array e entao começar?


    let pivot: T = vec[pivot_index];

    let mut i: usize = 0; // posicao do ultimo numero menor que o particionador
    for num_pos in 0..vec.len() {
        if vec[num_pos] < pivot {
            vec.swap(num_pos, i);
            i += 1;
        }
    }

    vec.swap()


    

    return pivot_index; 
}


fn get_random_pivot<T: Ord + Copy>(vec: &mut [T]) -> T {

}

/////////////////////////////////////////////////////////
// um swap entre mesmos índices deve ser contabilizado?
/////////////////////////////////////////////////////////
fn get_mo3_pivot<T: Ord + Copy>(vec: &mut [T], n_swaps: &mut u32) -> T {
    // acha qual dos 3 é a mediana.
    // este valor deve ser movido para a primeira posição.
    // continuamos a partir dai, iterando a partir do indice 1 (2ª pos).

    let mut ord: [usize; 3] = [0, vec.len()/2, vec.len()];
    ord.sort_by_key(|&pos| vec[pos]);

    vec.swap(0, ord[1]);


    return vec[0];
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