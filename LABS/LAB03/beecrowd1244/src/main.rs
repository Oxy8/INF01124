use std::fmt::Write;
use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    let mut input_line = String::new();
    
    handle.read_line(&mut input_line).unwrap();

    let num: usize = input_line.trim().parse::<usize>().unwrap();

//    let mut vec_palavras: Vec<&str>;

    let mut buf = String::new();

    for _ in 0..num {
        input_line.clear();
        handle.read_line(&mut input_line).unwrap();
        
        let mut vec_palavras: Vec<&str> = input_line.split_whitespace().collect(); // Rust 1.43.0 asks for realloc?
        vec_palavras.sort_by(|&a, &b| b.len().partial_cmp(&(a.len())).unwrap());
        
        // must be stable, try using radix sort here.

        for item in vec_palavras {
            write!(buf, "{} ", item).unwrap();
        } 
        
        buf.pop();
        writeln!(buf).unwrap();
    }
    //buf.pop();

    print!("{}",buf);


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