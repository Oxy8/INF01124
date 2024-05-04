use std::fs::{read_to_string, write};
use std::fmt::Write;
use std::vec;

fn main() {

    // 1.2
    let input_frankenstein = read_to_string("./entradas/frankenstein.txt").unwrap();
    ordena_palavras_arquivo(input_frankenstein, "./saidas/frankenstein_sorted.txt");
    let input_war_and_peace = read_to_string("./entradas/war_and_peace.txt").unwrap();
    ordena_palavras_arquivo(input_war_and_peace, "./saidas/war_and_peace_sorted.txt");


    // 1.3
    let input_frankenstein_sorted = read_to_string("./saidas/frankenstein_sorted.txt").unwrap();
    conta_palavras_arquivo(input_frankenstein_sorted, "./saidas/frankenstein_counted.txt");
    let input_war_and_peace_sorted = read_to_string("./saidas/war_and_peace_sorted.txt").unwrap();
    conta_palavras_arquivo(input_war_and_peace_sorted, "./saidas/war_and_peace_counted.txt");


    // 1.4
    let input_frankenstein_counted = read_to_string("./saidas/frankenstein_counted.txt").unwrap();
    gera_ranking_palavras_arquivo(input_frankenstein_counted, "./saidas/frankenstein_ranked.txt");
    let input_war_and_peace_counted = read_to_string("./saidas/war_and_peace_counted.txt").unwrap();
    gera_ranking_palavras_arquivo(input_war_and_peace_counted, "./saidas/war_and_peace_ranked.txt");

}


fn radix_sort_msd(vec: &mut [&[u8]], pos: usize) {
    
    let vec_len = vec.len();
    if vec_len <= 1 { return; }

    let mut count: Vec<usize> = vec![0; 256];
    let mut too_short: usize = 0;

    // calcula frequencia
    for &mut string in &mut *vec {
        if let Some(c) = char_at(string, pos) {
            count[c as usize] += 1;
        }
        else {
            too_short += 1;
        }
    }

    // converte frequencias em acumuladores
    count[0] += too_short;
    for val_index in 1..256 {
        count[val_index] += count[val_index-1];
    }

    // distribui no vetor auxiliar.
    let mut aux: Vec<&[u8]> = vec![&[0]; vec_len];
    for &string in vec.iter().rev() {
        if let Some(c) = char_at(string, pos) {
            aux[count[c as usize] - 1] = string;
            count[c as usize] -= 1;
        }
        else {
            aux[too_short-1] = string;
            too_short -= 1;
        }
    }

    // copia de volta para vec
    vec.copy_from_slice(&aux);

    // recursão
    count.push(vec_len);
    for char in 0..256 {
        radix_sort_msd(&mut vec[count[char]..count[char+1]], pos+1); 
    }

}



fn char_at(string: &[u8], pos: usize) -> Option<u8> {
    if pos >= string.len() {
        return None;
    }
    else {
        return Some(string[pos]);
    }
}



fn ordena_palavras_arquivo(input: String, path: &str) {

    let mut vec: Vec<&[u8]> = input
        .split_whitespace()
        .map(|string| string.as_bytes())
        .collect();

    radix_sort_msd(&mut vec, 0);

    let mut output_buffer = String::new();
    for item in vec {
        writeln!(output_buffer, "{}", std::str::from_utf8(item).unwrap()).unwrap();
    }
    write(path, output_buffer).unwrap();

}



fn conta_palavras_arquivo(input: String, path: &str) {

    let mut vec_quantidade_palavras: Vec<u16> = Vec::new();
    let mut lista_palavras: Vec<&str> = Vec::new();

    for palavra in input.split_whitespace() {
        
        if lista_palavras.last().cloned().unwrap_or_default() == palavra { // default value ensures verification will fail.
            let mut last: u16 = vec_quantidade_palavras.pop().unwrap();
            last += 1;
            vec_quantidade_palavras.push(last);
        } else {
            lista_palavras.push(palavra);
            vec_quantidade_palavras.push(1);
        }
    }

    let mut output_buffer = String::new();
    lista_palavras
        .iter()
        .zip(vec_quantidade_palavras.iter())
        .for_each(|(&palavra, &quantidade)| {
            writeln!(output_buffer, "{} {}", palavra, quantidade).unwrap()
        });
    
    write(path, output_buffer).unwrap();

}



fn gera_ranking_palavras_arquivo(input: String, path: &str) {
    let (palavras, quantidades): (Vec<&str>, Vec<&str>) = input
        .trim()
        .split('\n')
        .map(|str| str.split_once(' ').unwrap())
        .unzip();

    let quantidades = quantidades
        .iter()
        .map(|val| val.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    let mut vec_pares = palavras
        .into_iter()
        .zip(quantidades.into_iter())
        .collect::<Vec<(&str, u16)>>();

    merge_sort_tupla_rev(&mut vec_pares);
    vec_pares.reverse();

    let mut output_buffer = String::new();
    for i in 0..2000 {
        writeln!(output_buffer, "{} {}", vec_pares[i].0, vec_pares[i].1).unwrap()
    }
    
    write(path, output_buffer).unwrap();
}



fn merge_sort_tupla_rev<A: Copy, B: Ord + Copy>(vec: &mut [(A, B)]) {
    let vec_len = vec.len();
    if vec_len> 1 {
        let (mut left, mut right) = vec.split_at_mut(vec_len/2);
        merge_sort_tupla_rev(&mut left);
        merge_sort_tupla_rev(&mut right);
    }
    merge_tupla_rev(vec);
}

fn merge_tupla_rev<A: Copy, B: Ord + Copy>(vec: &mut [(A, B)]) {
    
    let (left, right) = vec.split_at(vec.len()/2);
    
    let mut aux: Vec<(A, B)> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i].1 < right[j].1 {
            aux.push(left[i]);
            i += 1;
        } else {
            aux.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        aux.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        aux.push(right[j]);
        j += 1;
    }


    vec.copy_from_slice(&aux);
}




/*
Frankenstein: 7270 palavras diferentes, 78444 palavras no total.
War and Peace: 17601 palavras diferentes, 586447 palavras no total.
*/