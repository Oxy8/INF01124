fn main() {
    let vec = vec! ["apple", "apricot", "apex", "apartment", "apology", "apparatus", "appendix", "applicant", "appliance", "appoint"];

    let mut aux: Vec<&[u8]> = vec.iter().map(|&str| str.as_bytes()).collect();
    radix_sort_msd(&mut aux, 0);

    for item in aux {
        println!("{}", std::str::from_utf8(item).unwrap());
    }

}



// tenho que ler o primeira caractere de cada palavra.

// usar enésimo caractere como índice

fn radix_sort_msd(vec: &mut [&[u8]], pos: usize) {
    
    let vec_len = vec.len();

    if vec_len == 0 {
        return;
    }

    let mut count: Vec<usize> = vec![0; 256];
    let mut too_short: usize = 0; // contador de palavras que são pequenas demais para serem ordenadas.

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
    for pos in 0..vec_len {
        vec[pos] = aux[pos];
    }

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







// Esperado 78k palavras.
// fazer leitura dos arquivos em 3 etapas, permite isolar testes.
// não ler tudo na memória (pelos menos n teste 2). ler linha por linha.
// 


// Passo 1:
// Le e ordena palavras (usar radix sort)

// Passo 2:
// Le linha, se valor igual ao último, add 1 ao último.
// Senão, criar nova Struct na array com o nome da string e o numero 1 do lado

// Passo 3:


// MSD Alg:
/*
Ve primeira Letra
Coloca em Buckets
(como fazer para evitar alocação de novos vetores e ao invés, fazer inplace
usando slices do rust.)




///////////////////////////////////////////////////////////////
/// Computando frequencia do caractere D (montando balde de cada caracter)
/// 
/// Distribute = joga para balde 
/// copy back = coloca no vetor na ordem
/// 
/// 
/// R+2 para que possamos somar sem o risco de invadir espaço extra
/// Provavelmente não necessário se implementar usando windows.
/// 
///////////////////////////////////////////////////////////////


 // sort from a[lo] to a[hi], starting at the dth character
    private static void sort(String[] a, int lo, int hi, int d, String[] aux) {

        // compute frequency counts
        int[] count = new int[R+2];
        for (int i = lo; i <= hi; i++) {
            int c = charAt(a[i], d);
            count[c+2]++;
        }
        
        // transform counts to indices
        for (int r = 0; r < R+1; r++)
            count[r+1] += count[r];

        // distribute
        for (int i = lo; i <= hi; i++) {
            int c = charAt(a[i], d);
            aux[count[c+1]++] = a[i];
        }

        // copy back
        for (int i = lo; i <= hi; i++)
            a[i] = aux[i - lo];


        // recursively sort for each character (excludes sentinel -1)
        for (int r = 0; r < R; r++)
            sort(a, lo + count[r], lo + count[r+1] - 1, d+1, aux);
    }

*/





/*

Parte 2 pede pra que a gente conte o numero de cada palavra que a gente tem.

Mas isso meio que ja é feito na parte 1 quando vamos criando os buckets.
Se eu segmentar o radix sort em varias funções, eu nao tenho que fazer nada adicional
senão contar o tamanho de cada bucket (chamar slice.len()).
// Parece um pouco bobagem fazer tudo isso, é mais fácil só ter 2 while e ir contando.
// Existe maneira melhor que um Vec de structs, onde cada struct contem nome e quant?






*/