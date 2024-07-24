use std::collections::LinkedList;
use std::vec;
use csv::Reader;
use std::time::Instant;
use std::fs::{read_to_string, write};
use std::fmt::{Write, Display};


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
}

impl Tabelavel for Jogador {
    fn chave(&self) -> usize {
        return self.sofifa_id;
    }
}


fn main() { 

    let jogadores = read_jogadores("./arquivos-suporte/players.csv").unwrap();
    let consultas = read_consultas("./arquivos-suporte/consultas.csv").unwrap();


    let tamanhos_tabelas: Vec<usize> = vec![997, 1999, 3989, 7993];

    for tamanho in tamanhos_tabelas {
        let mut saida_experimento = String::new();

        let tabela_hash: TabelaHash<Jogador> = benchmark_tabela_hash(tamanho, jogadores.clone(), &mut saida_experimento);
        benchmark_consultas(&tabela_hash, &consultas, &mut saida_experimento);


        let path: String = format!("./saidas/experimento{}.txt", tamanho);
        write(path, saida_experimento).unwrap();
    }

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
        };
        jogadores.push(jogador);
    }

    Ok(jogadores)
}

fn read_consultas(file_path: &str) -> Result<Vec<u32>, String>{
    let input_buffer = read_to_string(file_path).unwrap();
    let vec_ints: Vec<u32> = input_buffer.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
    
    return Ok(vec_ints);
}

fn benchmark_consultas(tabela: &TabelaHash<Jogador>, consultas: &Vec<u32>, saida_experimento: &mut String) {
    
    let mut buffer_saida = String::new();

    // Inicio do timer.
    let timer = Instant::now();
    //-------------------------------------
    
    let mut media: u32 = 0;
    let mut validos: u32 = 0;
    let mut max: u32 = 0;

    for chave in consultas {
        let mut consultas: u32 = 0;
        
        if let Some(jogador) = busca_tabela_hash(&tabela, *chave as usize, &mut consultas) {
            writeln!(buffer_saida, "{} {} {}", chave, jogador.name, consultas).unwrap();
            media += consultas;
            validos += 1;
            max = std::cmp::max(max, consultas);
        }
        else {
            writeln!(buffer_saida, "{} NAO ENCONTRADO {}", chave, consultas).unwrap();           
        }
        
    }
    media = media/validos;

    //-------------------------------------
    let exec_time: u128 = timer.elapsed().as_nanos(); // Tempo em nanosegundos.
    let exec_time_millis: f64 = exec_time as f64/1_000_000.0; // Tempo em milissegundos

    write!(saida_experimento, "
PARTE 2: ESTATISTICAS DAS CONSULTAS
TEMPO PARA REALIZACAO DE TODAS CONSULTAS {}
", exec_time_millis, ).unwrap();

    write!(saida_experimento, "{}", buffer_saida).unwrap();

    write!(saida_experimento, 
"MAXIMO NUMERO DE TESTES POR NOME ENCONTRADO {}
MEDIA NUMERO DE TESTES POR NOME ENCONTRADO {}
    ", max, media).unwrap();

}

fn benchmark_tabela_hash<T: Tabelavel + Clone>(tamanho: usize, vetor_itens: Vec<T>, saida_experimento: &mut String) -> TabelaHash<T> {
    
    // Inicio do timer.
    let timer = Instant::now();
    //-------------------------------------
    let tabela_hash: TabelaHash<T> = constroi_tabela_hash(tamanho, vetor_itens);
    //-------------------------------------
    let exec_time: u128 = timer.elapsed().as_nanos(); // Tempo em nanosegundos.
    let exec_time_millis: f64 = exec_time as f64/1_000_000.0; // Tempo em milissegundos

    let mut acc: u32 = 0;
    let mut max: u32 = 0;
    let mut ocupadas: u32 = 0;
    for lista in &tabela_hash.tabela {
        let len = lista.len() as u32;
        
        if len != 0 {
            ocupadas += 1;
            acc += len;
            max = std::cmp::max(max, len);
        }
    }

    let taxa_de_ocupacao: f64 = (ocupadas as f64)/(tamanho as f64);
    
    write!(saida_experimento, 
"PARTE 1: ESTATISTICAS DA TABELA HASH
TEMPO DE CONSTRUCAO DA TABELA {}
TAXA DE OCUPACAO {:.6}
TAMANHO MAXIMO DE LISTA {}
TAMANHO MEDIO DE LISTA {}
", exec_time_millis, taxa_de_ocupacao, max, acc/ocupadas).unwrap();


    return tabela_hash;
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

fn busca_tabela_hash<T: Tabelavel + Clone>(tabela: &TabelaHash<T> , chave: usize, consultas: &mut u32) -> Option<T> {
    let pos = (chave % tabela.tamanho);
    let lista = &tabela.tabela[pos];

    for nodo in lista {
        *consultas += 1;
        if nodo.chave() == chave {
            return Some(nodo.clone());
        }
    }

    return None;
}
