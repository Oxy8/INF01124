#![feature(iter_collect_into)]
#![feature(let_chains)]

use std::collections::LinkedList;
use std::vec;
use csv::Reader;
use polars::prelude::*;
use std::time::Instant;
use tabled::{Tabled, Table};
use std::io::Write;


#[derive(Clone)]
struct AccContador {
    acc: f64,
    contador: usize,
}

#[derive(Clone, Default, Debug)]
struct TabelaHash<T: Tabelavel + Clone> {
    tamanho: usize,
    tabela: Vec<LinkedList<T>>
}

trait Tabelavel {
    fn chave(&self) ->  usize;
}

#[derive(Clone, Default, Debug, Tabled)]
struct Jogador {
    sofifa_id: u32,
    short_name: String,
    long_name: String,
    player_positions: String,
    nationality: String,
    club_name: String,
    league_name: String,
    number_ratings: usize,
    ratings_sum: f64,
    #[tabled(display_with = "format_float")]
    average_rating: f32
}



impl PartialEq for Jogador {
    fn eq(&self, other: &Self) -> bool {
        self.average_rating.eq(&other.average_rating)
    }
}


impl PartialOrd for Jogador {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.average_rating.partial_cmp(&self.average_rating)
        // Invertido de propósito pra não precisar lidar com inversão do quicksort 
        // e nem precisar usar rev(), o que prejudicaria a performance.
    }
}


impl Tabelavel for Jogador {
    fn chave(&self) -> usize {
        return self.sofifa_id as usize;
    }
}

#[derive(Clone, Default, Debug)]
struct NodoTrie {
    letra: char,
    fim_de_palavra: u32, // if != 0, contains player id.
    children: TabelaHash<NodoTrie>,
}

impl Tabelavel for NodoTrie {
    fn chave(&self) -> usize {
        return self.letra as usize;
    }
}

#[derive(Clone, Default, Debug, Copy)]
struct Avaliacao {
    sofifa_id: u32,
    rating: f32,
}

#[derive(Clone, Default, Debug, Tabled)]
struct AvaliacaoDetalhada {
    sofifa_id: u32,
    short_name: String,
    long_name: String,
    #[tabled(display_with = "format_float")]
    global_rating: f32,
    number_ratings: usize,
    rating: f32,
}

fn format_float(value: &f32) -> String {
    format!("{:.6}", value)
}


impl PartialEq for AvaliacaoDetalhada {
    fn eq(&self, other: &Self) -> bool {
        (self.rating.eq(&other.rating) && self.global_rating.eq(&other.global_rating))
    }
}


impl PartialOrd for AvaliacaoDetalhada {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if other.rating == self.rating {
            other.global_rating.partial_cmp(&self.global_rating)    
        } else {
            other.rating.partial_cmp(&self.rating)
        }
        // Invertido de propósito pra não precisar lidar com inversão do quicksort 
        // e nem precisar usar rev(), o que prejudicaria a performance.
    }
}

impl Tabelavel for &str {
    fn chave(&self) -> usize {
        return hash_string(self) as usize;
    }
}

fn hash_string(s: &str) -> u64 {
    let mut hash: u64 = 0;
    let prime: u64 = 31;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(prime).wrapping_add(byte as u64);
    }
    return hash;
}

fn main() { 
    
    
    //=========================================================================
    //======Construção das Estruturas==========================================
    //=========================================================================

    //=========================================================================
    // Inicio do timer.
    let timer = Instant::now();
    //=========================================================================

    let jogadores = read_jogadores("./arquivos-parte1/players.csv").unwrap();
    let mut tabela_jogadores = constroi_tabela_hash(10000, jogadores.clone());


    let mut df = CsvReadOptions::default().with_infer_schema_length(Some(0))
        .try_into_reader_with_file_path(Some("./arquivos-parte1/rating.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let mut vec_avaliacoes: Vec<Vec<Avaliacao>> = vec![vec![Avaliacao{..Default::default()}; 0]; 140000];
    let mut vec_acc_contador: Vec<AccContador> = vec![AccContador { acc: 0.0, contador: 0 }; 260000];

    let user_ids = df.column("user_id").unwrap().str().unwrap();
    let player_ids = df.column("sofifa_id").unwrap().str().unwrap();
    let ratings = df.column("rating").unwrap().str().unwrap();

    for i in 0..df.height() {
        let user_id = user_ids.get(i).unwrap().parse::<usize>().unwrap();
        let player_id = player_ids.get(i).unwrap().parse::<u32>().unwrap();
        let rating = ratings.get(i).unwrap().parse::<f32>().unwrap();

        vec_acc_contador[player_id as usize].acc += rating as f64;
        vec_acc_contador[player_id as usize].contador += 1;

        vec_avaliacoes[user_id].push(Avaliacao{sofifa_id: player_id, rating: rating});
    
    }
    
    for n in 0..vec_acc_contador.len() {
        atualiza_rating(&mut tabela_jogadores, n,  vec_acc_contador[n].clone());
    }

    let mut arvore_trie: NodoTrie = NodoTrie{letra: '#', fim_de_palavra: 0, children: cria_tabela_hash(30)};
    for jogador in jogadores.clone() {
        insere_jogador_trie(&mut arvore_trie, jogador);
    }

    //=========================================================================

    let mut vetor_posicoes: Vec<Vec<Jogador>> = vec![vec![Jogador{..Default::default()}; 0]; 16];

    for lista in &tabela_jogadores.tabela {
        for jogador in lista {

            if jogador.number_ratings >= 1000 {
                let posicoes: Vec<&str> = jogador.player_positions.split(",").map(|pos| pos.trim()).collect();
                let indices = position_to_index(&posicoes);
                
                for indice in indices {
                    vetor_posicoes[indice].push(jogador.clone());
                }
            }
        }
    }

    for posicao in &mut vetor_posicoes {
        quicksort(posicao, quicksort_hoare, get_mo3_pivot);
    }
    
    //=========================================================================

    let df_tags = CsvReadOptions::default().with_infer_schema_length(Some(0))
        .try_into_reader_with_file_path(Some("./arquivos-parte1/tags.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let player_ids = df_tags.column("sofifa_id").unwrap().str().unwrap();
    let tags = df_tags.column("tag").unwrap().str().unwrap();

    let mut lista_tags: Vec<&str> = Vec::new();
    let mut lista_tags_jogadores: Vec<Vec<u32>> = vec![vec![10; 0]; 1000];


    for i in 0..df_tags.height() {
        let player_id = player_ids.get(i).unwrap().parse::<u32>().unwrap();
        
        if let Some(tag) = tags.get(i) {

            if let Some(position) = lista_tags.iter().position(|&t| t == tag) {
                lista_tags_jogadores[position].push(player_id);
            } else {
                let position = lista_tags.len();
                lista_tags.push(tag);
                lista_tags_jogadores[position].push(player_id);
            }

        }       
    }


    //=========================================================================
    // Fim do timer.
    let exec_time: u128 = timer.elapsed().as_nanos(); // Tempo em nanosegundos.
    let exec_time_millis: f64 = exec_time as f64/1_000_000.0; // Tempo em milissegundos
    println!("\nTempo para construção das estruturas: {:.2} ms.\n", exec_time_millis);
    //=========================================================================

    //=========================================================================
    //======Loop Principal=====================================================
    //=========================================================================
    

    

    loop {
        
        print!("Insira a consulta desejada: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.starts_with("exit") { break; }

        if    input.starts_with("player ")
            ||input.starts_with("user ")
            ||input.starts_with("top ")
            ||input.starts_with("tags ") {

            if let Some((cmd, arg)) = input.split_once(" ") {
                match cmd {
                    "player" => {
                        pesquisa_1(arvore_trie.clone(), &mut tabela_jogadores, &arg.to_lowercase());
                    }
                    "user" => {
                        if let Ok(num) = arg.parse::<u32>() {
                            pesquisa_2(&vec_avaliacoes, &mut tabela_jogadores, num);
                        } else {
                            println!("Não foi possível fazer o parsing de {} para um número. Tente novamente.", arg);
                        }
                    }
                    "top" => {
                        if let Some((posicao, len)) = arg.split_once(" ") {
                            if let Ok(num) = len.parse::<u32>() {
                                
                                let valid_positions = vec!["CAM", "CB", "CDM", "CF", "CM", "CWB", "GK", "LB", "LM", "LW", "LWB", "RB", "RM", "RW", "RWB", "ST"];
                                let upper = posicao.to_uppercase();
                                let inter: &str = &upper;
                                if !(valid_positions.contains(&inter)) {
                                    println!("A posição inserida {} não é uma posição válida. Tente novamente.", posicao);
                                    continue;
                                }

                                pesquisa_3(vetor_posicoes.clone(), inter, num as usize);

                            } else {
                                println!("Não foi possível fazer o parsing de {} para um número. Tente novamente.", len);
                            }
                        
                        } else {
                            println!("Comando inválido. Tente novamente.");
                        }
                    }
                    "tags" => {
                        let tags = parse_apostrophe_quoted_strings(arg);
                        pesquisa_4(lista_tags.clone(), lista_tags_jogadores.clone(), &mut tabela_jogadores, &tags);
                    }
                    _ => println!("Comando desconhecido."),
                }

            } else {
                println!("Comando inválido. Tente novamente.");
                continue;
            }

        } else {
            println!("Comando inválido. Tente novamente.");
        }
    }


    // Fim do timer.
    let exec_time: u128 = timer.elapsed().as_nanos(); // Tempo em nanosegundos.
    let exec_time_millis: f64 = exec_time as f64/1_000_000.0; // Tempo em milissegundos
    println!("\nTempo para construção + execução: {:.2} ms.\n", exec_time_millis);
     
}

fn pesquisa_1(arvore_trie: NodoTrie, tabela_jogadores: &mut TabelaHash<Jogador>, prefixo: &str) {
    let vec_player_ids: Vec<u32> = obtem_jogadores_trie(arvore_trie, prefixo).unwrap_or_default();
    let mut vec_players: Vec<Jogador> = Vec::new();

    for player_id in vec_player_ids {
        let jogador = busca_tabela_hash(tabela_jogadores, player_id as usize).unwrap().clone();
        vec_players.push(jogador);
    }

    quicksort(&mut vec_players, quicksort_hoare, get_mo3_pivot);

    let table = Table::new(vec_players).to_string();
    println!("{}", table);

}

fn pesquisa_2(vec_avaliacoes: &Vec<Vec<Avaliacao>>, tabela_jogadores: &mut TabelaHash<Jogador>, user_id: u32) {

    let vec_avaliacoes_user: Vec<Avaliacao> = vec_avaliacoes[user_id as usize].clone();
    let mut vec_avaliacoes_detalhadas_user: Vec<AvaliacaoDetalhada> = vec_avaliacoes_user.iter().map(|a| obtem_avaliacao_detalhada(a, tabela_jogadores)).collect();
    quicksort(&mut vec_avaliacoes_detalhadas_user, quicksort_hoare, get_mo3_pivot);
    vec_avaliacoes_detalhadas_user.truncate(20);

    let table = Table::new(vec_avaliacoes_detalhadas_user).to_string();
    println!("{}", table);

    fn obtem_avaliacao_detalhada(avaliacao: &Avaliacao, tabela_jogadores: &mut TabelaHash<Jogador>) -> AvaliacaoDetalhada {
        
        let jogador: &mut Jogador = busca_tabela_hash(tabela_jogadores, avaliacao.sofifa_id as usize).unwrap();
        return AvaliacaoDetalhada{
            sofifa_id: avaliacao.sofifa_id,
            short_name: jogador.short_name.clone(),
            long_name: jogador.long_name.clone(),
            rating: avaliacao.rating,
            number_ratings: jogador.number_ratings,
            global_rating: jogador.average_rating,
        }
    }
    
}

fn pesquisa_3(vetor_posicoes: Vec<Vec<Jogador>>, posicao: &str, mut len: usize) {

    let index = position_to_index(&[posicao]);
    let posicao = &vetor_posicoes[index[0]];

    if len > posicao.len() {len = posicao.len()};
    let mut posicao2: Vec<Jogador> = Vec::new();
    for i in 0..len {
        posicao2.push(posicao[i].clone());
    }
    let table = Table::new(posicao2).to_string();
    println!("{}", table);
}

fn pesquisa_4(lista_tags: Vec<&str>, lista_tags_jogadores: Vec<Vec<u32>>, tabela_jogadores: &mut TabelaHash<Jogador>, tags: &[&str]) {
    //pega tags particiona elas, acessa em lista_tags_jogadores cada uma delas,
    // faz a intersecção das tags selecionadas e finalmente converte de ids pra jogadores e joga na tela.
    let mut lista_tags_jogadores_selecionadas: Vec<Vec<u32>> = Vec::new();
    
    for &tag in tags {
        if let Some(position) = lista_tags.iter().position(|&t| t == tag) {
            lista_tags_jogadores_selecionadas.push(lista_tags_jogadores[position].clone());
        }
    }

    let mut intersecao = intersect_vectors(lista_tags_jogadores_selecionadas);
    intersecao.dedup();

    let mut players: Vec<Jogador> = Vec::new();
    for id in intersecao {
        let jogador = busca_tabela_hash(tabela_jogadores, id as usize).unwrap();
        players.push(jogador.clone());
    }

    quicksort(&mut players, quicksort_hoare, get_mo3_pivot);

    let table = Table::new(players).to_string();
    println!("{}", table);

}

fn intersect_vectors(vectors: Vec<Vec<u32>>) -> Vec<u32> {
    if vectors.is_empty() {
        return Vec::new();
    }

    let first_vec = &vectors[0];

    let mut common_elements = Vec::new();

    for &item in first_vec {
        if vectors.iter().all(|vec| vec.contains(&item)) {
            common_elements.push(item);
        }
    }

    return common_elements;
}

fn parse_apostrophe_quoted_strings(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut in_quotes = false;
    let mut start_idx = None;

    for (i, ch) in input.char_indices() {
        match ch {
            '\'' => {
                if in_quotes {
                    if let Some(start) = start_idx {
                        result.push(&input[start..i]);
                        start_idx = None;
                    }
                } else {
                    start_idx = Some(i + 1); // Start after the quote
                }
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {}
            _ => {}
        }
    }

    // Handle case where input might end with a quote
    if in_quotes && start_idx.is_some() {
        result.push(&input[start_idx.unwrap()..]);
    }

    result
}

fn position_to_index(positions: &[&str]) -> Vec<usize> {
    let valid_positions = vec![ "GK", "RB", "CB", "LB", "CDM", "CM", "CAM", "RM", "LM", "RW", "LW", "LWB", "CF", "ST", "CWB", "RWB" ];

    positions
        .iter()
        .map(|pos| valid_positions.iter().position(|&v| v == *pos).unwrap())
        .collect()
}

fn insere_jogador_trie(nodo: &mut NodoTrie, jogador: Jogador) {

    let mut nodo_movel: &mut NodoTrie = nodo;

    for char in jogador.long_name.to_lowercase().chars() {
        
        let pos = (nodo_movel.letra as usize % nodo_movel.children.tamanho);
        let lista = &nodo_movel.children.tabela[pos];

        let mut encontrado = false;

        for nodo_1 in lista {
            if nodo_1.chave() == char as usize {
                encontrado = true;
            }
        }
        
        if !encontrado {
            let novo_nodo: NodoTrie = NodoTrie{letra: char, fim_de_palavra: 0, children: cria_tabela_hash(30)};
            insere_tabela_hash(&mut (nodo_movel.children), novo_nodo);
        }
        nodo_movel = busca_tabela_hash(&mut nodo_movel.children, char as usize).unwrap();
    }
    nodo_movel.fim_de_palavra = jogador.sofifa_id;
}

fn obtem_jogadores_trie(mut nodo: NodoTrie, prefixo: &str) -> Option<Vec<u32>> {

    let mut nodo_movel: &mut NodoTrie = &mut nodo;

    for char in prefixo.chars() {
        
        let pos = (char as usize % 30);

        let lista = &nodo_movel.children.tabela[pos];

        let mut encontrado = false;


        for nodo_1 in lista {
            if nodo_1.chave() == char as usize {
                encontrado = true;
            }
        }
        
        if !encontrado {
            return None;    
        }
        nodo_movel = busca_tabela_hash(&mut nodo_movel.children, char as usize).unwrap();
    }

    let nodo_temp = nodo_movel.clone();
    let mut vec_ids: Vec<u32> = Vec::new();
    le_arvore_trie_recursiva(nodo_temp, &mut vec_ids);

    return Some(vec_ids);
}

fn le_arvore_trie_recursiva(mut nodo: NodoTrie, vec: &mut Vec<u32>) {

    if nodo.fim_de_palavra != 0 {
        vec.push(nodo.fim_de_palavra);
    }

    let tabela = nodo.children.tabela;

    for list in tabela {
        for nodo_interno in list {
            le_arvore_trie_recursiva(nodo_interno, vec);
        }
    }
}

fn read_jogadores(file_path: &str) -> Result<Vec<Jogador>, String> {
    let mut rdr = Reader::from_path(file_path).unwrap();
    let mut jogadores = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let jogador = Jogador {
            sofifa_id: record[0].parse::<u32>().unwrap(),
            short_name: record[1].to_string(),
            long_name: record[2].to_string(),
            player_positions: record[3].to_string(),
            nationality: record[4].to_string(),
            club_name: record[5].to_string(),
            league_name: record[6].to_string(),
            number_ratings: 0,
            ratings_sum: 0 as f64,
            average_rating: 0 as f32
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

fn busca_tabela_hash<T: Tabelavel + Clone>(tabela: &mut TabelaHash<T> , chave: usize) -> Option<&mut T> {
    
    if tabela.tamanho == 0 {return None};
    
    let pos = (chave % tabela.tamanho);
    let lista = &mut tabela.tabela[pos];

    for mut nodo in lista {
        if nodo.chave() == chave {
            return Some(nodo);
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
            nodo.average_rating = (acc_contador.acc/acc_contador.contador as f64) as f32;
            break;
        }
    }
}

fn quicksort<T: PartialOrd + Clone>(vec: &mut [T], partitioning_scheme_f: fn(&mut [T], fn(&[T]) -> usize) -> usize, pivot_selector_f: fn(&[T]) -> usize) {

    if vec.len() > 1 {
        let pivot_index: usize = partitioning_scheme_f(vec, pivot_selector_f);
        
        quicksort(&mut vec[..pivot_index], partitioning_scheme_f, pivot_selector_f,);
        quicksort(&mut vec[pivot_index+1..], partitioning_scheme_f, pivot_selector_f);
    }

}


// retorno é o índice do elemento particionador utilizado.
fn quicksort_hoare<T: PartialOrd + Clone>(vec: &mut [T], pivot_selector_f: fn(&[T]) -> usize) -> usize {

    let pivot_index = pivot_selector_f(vec);
    vec.swap(0, pivot_index);
    let pivot = vec[0].clone();

    let mut i = 0;
    let mut j = vec.len() - 1;
    

    while i < j {
        while vec[j] > pivot && i < j { j -= 1 };
        while vec[i] <= pivot && i < j { i += 1 };
        
        vec.swap(i, j);
    }

    vec.swap(0, j);

    return i;
    
}


fn get_mo3_pivot<T: PartialOrd + Clone>(vec: &[T]) -> usize {

    let a = vec[0].clone();
    let b = vec[(vec.len()-1)/2].clone();
    let c = vec[vec.len()-1].clone();

    if b <= a && a <= c {
        return 0;
    } else if a <= b && b <= c {
        return (vec.len()-1)/2;
    } else {
        return vec.len()-1;
    }
}
