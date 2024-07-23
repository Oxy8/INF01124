use std::collections::LinkedList;

struct TabelaHash<T: Tabelavel + Clone> {
    tamanho: usize,
    tabela: Vec<LinkedList<T>>
}

trait Tabelavel {
    fn chave(&self) ->  usize;
}
#[derive(Clone)]
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
    let tabela_hash: TabelaHash<Jogador> = cria_tabela_hash(37);
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

fn busca_tabela_hash<T: Tabelavel + Clone>(tabela: &mut TabelaHash<T> , chave: usize) -> Option<T> {
    let pos = (chave % tabela.tamanho);
    let lista = &tabela.tabela[pos];

    for nodo in lista {
        if nodo.chave() == chave {
            return Some(nodo.clone());
        }
    }

    return None;
}
