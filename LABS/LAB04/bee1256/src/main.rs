use std::fmt::Write;

use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    handle.read_line(&mut buffer);
    let num_casos = buffer.trim().parse::<u16>().unwrap();
    buffer.clear();

    let mut buf = String::new();
    
    for _caso in 0..num_casos {
        
        handle.read_line(&mut buffer).unwrap();
        let mut par = buffer.trim().split_whitespace();
        let divisor: u16 = par.next().unwrap().parse().unwrap();
        let lista_entrada_size: u16 = par.next().unwrap().parse().unwrap();
        buffer.clear();

        handle.read_line(&mut buffer).unwrap();
        let lista_entrada: Vec<u16> = buffer.trim()
            .split_whitespace()
            .map(|v| v.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
        buffer.clear();

        let mut acc_vec: Vec<Vec<u16>> =  vec![Vec::new(); divisor as usize];

        
        for val in lista_entrada.iter().take(lista_entrada_size as usize) {
            let resto = val % divisor;
            acc_vec[resto as usize].push(*val);
        }

        for (indice, sublist) in acc_vec.iter().enumerate() {
            write!(buf, "{} -> ", indice).unwrap();
            for num in sublist {
                write!(buf, "{} -> ", num).unwrap();
            }
            writeln!(buf, "\\").unwrap();
        }
        writeln!(buf, "").unwrap();
    }

    buf.pop();
    print!("{}",buf);
}