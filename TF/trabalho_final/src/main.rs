use std::collections::LinkedList;
use std::vec;
use csv::Reader;
use std::fs::{read_to_string, write};
use std::fmt::{Write, Display};
use polars::prelude::*;
use std::time::Instant;


#[derive(Clone)]
struct AccContador {
    acc: f64,
    contador: usize,
}

#[derive(Debug)]
struct TabelaHash<T: Tabelavel + Clone> {
    tamanho: usize,
    tabela: Vec<LinkedList<T>>
}

trait Tabelavel {
    fn chave(&self) ->  usize;
}
#[derive(Clone, Debug)]
struct Jogador {
    sofifa_id: usize,
    name: String,
    player_positions: String,
    number_ratings: usize,
    ratings_sum: f64,
}

impl Tabelavel for Jogador {
    fn chave(&self) -> usize {
        return self.sofifa_id;
    }
}


fn main() { 
    
    // Inicio do timer.
    let timer = Instant::now();
    //-------------------------------------

    let mut df = CsvReadOptions::default().with_infer_schema_length(Some(0))
        .try_into_reader_with_file_path(Some("./arquivos-parte1/rating.csv".into()))
        .unwrap()
        .finish()
        .unwrap();


    let jogadores = read_jogadores("./arquivos-parte1/players.csv").unwrap();

    let mut tabela_jogadores = constroi_tabela_hash(10000, jogadores);
    
    let mut acc_contador: Vec<AccContador> = vec![AccContador{acc: 0.0, contador: 0}; 260000];
    for (id, rating) in df.column("sofifa_id").unwrap().str().unwrap().into_iter().zip(df.column("rating").unwrap().str().unwrap()) {
        // esse zip aqui encima ta horrivel, tem que melhorar isso ai.
        acc_contador[id.unwrap().parse::<usize>().unwrap()].acc += rating.unwrap().parse::<f64>().unwrap();
        acc_contador[id.unwrap().parse::<usize>().unwrap()].contador += 1;
    }

    //==========================
    //==========================
    //==========================
    // VER FAVORITOS FIREFOX
    //==========================
    //==========================
    //==========================


    let mut acc_contador: Vec<AccContador> = vec![AccContador{acc: 0.0, contador: 0}; 260000];
    for row_index in 0..df.height() {
        // esse zip aqui encima ta horrivel, tem que melhorar isso ai.
        let row = df.get(row_index).unwrap();
        let id = row[1].cast(&String);
        acc_contador[id.unwrap().parse::<usize>().unwrap()].acc += rating.unwrap().parse::<f64>().unwrap();
        acc_contador[id.unwrap().parse::<usize>().unwrap()].contador += 1;
    }

    

    for n in 0..acc_contador.len() {
        atualiza_rating(&mut tabela_jogadores, n,  acc_contador[n].clone());
    }




    //-------------------------------------
    let exec_time: u128 = timer.elapsed().as_nanos(); // Tempo em nanosegundos.
    let exec_time_millis: f64 = exec_time as f64/1_000_000.0; // Tempo em milissegundos
    println!("Exec time (ms) = {}", exec_time_millis);
    
    
   
   //print!("{:?}", tabela_jogadores);


}


fn read_jogadores(file_path: &str) -> Result<Vec<Jogador>, String> {
    let mut rdr = Reader::from_path(file_path).unwrap();
    let mut jogadores = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let jogador = Jogador {
            sofifa_id: record[0].parse::<usize>().unwrap(),
            name: record[1].to_string(),
            player_positions: record[2].to_string(),
            number_ratings: 0,
            ratings_sum: 0 as f64
        };
        jogadores.push(jogador);
    }

    Ok(jogadores)
}



fn constroi_tabela_hash<T: Tabelavel + Clone>(tamanho: usize, vetor_itens: Vec<T>) -> TabelaHash<T>{
    let mut tabela= cria_tabela_hash(tamanho);
    for item in vetor_itens {
        insere_tabela_hash(&mut tabela, item);
    }

    return tabela;
}


fn cria_tabela_hash<T: Tabelavel + Clone>(tamanho: usize) -> TabelaHash<T>{
    let tabela = vec![LinkedList::<T>::new(); tamanho];
    return TabelaHash {tamanho, tabela};
}

fn insere_tabela_hash<T: Tabelavel + Clone>(tabela: &mut TabelaHash<T> , item: T) {
    let chave_item = item.chave();
    let pos = (chave_item % tabela.tamanho);

    tabela.tabela[pos].push_back(item);
}

fn busca_tabela_hash<T: Tabelavel + Clone>(tabela: &TabelaHash<T> , chave: usize) -> Option<T> {
    let pos = (chave % tabela.tamanho);
    let lista = &tabela.tabela[pos];

    for nodo in lista {
        if nodo.chave() == chave {
            return Some(nodo.clone());
        }
    }

    return None;
}

fn atualiza_tabela_hash<T: Tabelavel + Clone>(tabela: &mut TabelaHash<T> , chave: usize, valor: T) {
    let pos = (chave % tabela.tamanho);
    let lista = &mut tabela.tabela[pos];

    for nodo in lista {
        if nodo.chave() == chave {
            *nodo = valor.clone();
            break;
        }
    }
}

fn atualiza_rating(tabela: &mut TabelaHash<Jogador> , chave: usize, acc_contador: AccContador) {
    let pos = (chave % tabela.tamanho);
    let lista = &mut tabela.tabela[pos];

    for nodo in lista {
        if nodo.chave() == chave {
            nodo.ratings_sum = acc_contador.acc;
            nodo.number_ratings = acc_contador.contador;
            break;
        }
    }
}