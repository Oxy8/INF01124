// Originalmente compilado com rustc 1.78.0-nightly (878c8a2a6 2024-02-29)

// Compilado com RUSTFLAGS="-C opt-level=0" porque as otimizações estavam
// beneficiando muito mais um dos sistemas de particionamento do que o outro,
// e isso estava distorcendo os resultados.

#![feature(iter_collect_into)]
#![feature(let_chains)]

use std::fs::{read_to_string, write};
use std::fmt::{Write, Display};
use std::time::Instant;
use rand::Rng;
//use std::fmt::Debug;

fn main() {
    // Le arquivo de entrada e coloca conteúdo em String
    let input_buffer = read_to_string("./entradas/entrada.txt").unwrap();

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

        let mut to_be_sorted_copy = to_be_sorted.clone();
        benchmark_quicksort(&mut to_be_sorted_copy, quicksort_lomuto, get_mo3_pivot, &mut saida_lomuto_mediana);
        //print_ordered(&to_be_sorted_copy);

        let mut to_be_sorted_copy = to_be_sorted.clone();
        benchmark_quicksort(&mut to_be_sorted_copy, quicksort_lomuto, get_random_pivot, &mut saida_lomuto_aleatorio);
        //print_ordered(&to_be_sorted_copy);

        let mut to_be_sorted_copy = to_be_sorted.clone();
        benchmark_quicksort(&mut to_be_sorted_copy, quicksort_hoare, get_mo3_pivot, &mut saida_hoare_mediana);
        //print_ordered(&to_be_sorted_copy);

        let mut to_be_sorted_copy = to_be_sorted.clone();
        benchmark_quicksort(&mut to_be_sorted_copy, quicksort_hoare, get_random_pivot, &mut saida_hoare_aleatorio);
        //print_ordered(&to_be_sorted_copy);

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

    if vec.len() > 1 {
        let pivot_index: usize = partitioning_scheme_f(vec, n_swaps, pivot_selector_f);
        
        quicksort(&mut vec[..pivot_index], partitioning_scheme_f, pivot_selector_f, n_swaps, n_recursions);
        quicksort(&mut vec[pivot_index+1..], partitioning_scheme_f, pivot_selector_f, n_swaps, n_recursions);
    }

}


// retorno é o índice do elemento particionador utilizado.
fn quicksort_hoare<T: Ord + Copy>(vec: &mut [T], n_swaps: &mut u32, pivot_selector_f: fn(&[T]) -> usize) -> usize {

    let pivot_index = pivot_selector_f(vec);
    vec.swap(0, pivot_index);
    *n_swaps += 1;
    let pivot = vec[0];

    let mut i = 0;
    let mut j = vec.len() - 1;
    

    while i < j {
        while vec[j] > pivot && i < j { j -= 1 };
        while vec[i] <= pivot && i < j { i += 1 };
        
        vec.swap(i, j);
        *n_swaps += 1;
    }

    vec.swap(0, j);
    *n_swaps += 1;
    
    return i;
    
}

// retorno é o índice do elemento particionador utilizado.
fn quicksort_lomuto<T: Ord + Copy>(vec: &mut [T], n_swaps: &mut u32, pivot_selector_f: fn(&[T]) -> usize) -> usize {
    
    let pivot_index = pivot_selector_f(vec); // posicao do elemento particionador escolhido.
    vec.swap(0, pivot_index);
    *n_swaps += 1;
    let pivot = vec[0];

    let mut i: usize = 1; // posicao do ultimo numero menor que o particionador
    for num_pos in 1..vec.len() {
        if vec[num_pos] < pivot {
            vec.swap(num_pos, i);
            *n_swaps += 1;
            i += 1;
        }
    }

    // Coloca elemento particionador no seu lugar.
    vec.swap(0, i-1);
    *n_swaps += 1;
    
    // posicao do elemento particionador após organização.
    return i-1; 

}

fn get_random_pivot<T: Ord + Copy>(vec: &[T]) -> usize {

    let mut rng = rand::thread_rng();

    return rng.gen_range(0..vec.len());
}

fn get_mo3_pivot<T: Ord + Copy>(vec: &[T]) -> usize {

    let a = vec[0];
    let b = vec[(vec.len()-1)/2];
    let c = vec[vec.len()-1];

    if b <= a && a <= c {
        return 0;
    } else if a <= b && b <= c {
        return (vec.len()-1)/2;
    } else {
        return vec.len()-1;
    }
}

/*
fn print_vec<T: Debug>(vec: &[T]) {
    for item in vec {
        print!("{:?} ", item);
    }
    println!("");
}
*/

/*
fn print_ordered<T: Ord>(slice: &[T]) {
    if slice.windows(2).all(|w| w[0] <= w[1]) {
        println!("ordered");
    } else {
        println!("not ordered");
    }
}
*/
