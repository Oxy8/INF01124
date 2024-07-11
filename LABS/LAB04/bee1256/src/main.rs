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
        let par: Vec<u16> = buffer.trim()
            .split_whitespace()
            .map(|v| v.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
        buffer.clear();

        let divisor = par[0];
        let lista_entrada_size = par[1];


        handle.read_line(&mut buffer).unwrap();
        let lista_entrada: Vec<u16> = buffer.trim()
            .split_whitespace()
            .map(|v| v.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
        buffer.clear();

        let mut acc_vec: Vec<Vec<u16>> =  vec![Vec::new(); divisor as usize];

        
        for val_index in 0..lista_entrada_size {
            
            let val = lista_entrada[val_index as usize];
            let resto = (val % divisor);

            acc_vec[resto as usize].push(val);
        }

        for indice in 0..divisor {
            print_indice_vec(indice as usize, &acc_vec, &mut buf);
        }
        writeln!(buf, "").unwrap();
    }

    buf.pop();
    print!("{}",buf);
}


fn print_indice_vec(indice: usize, vec: &Vec<Vec<u16>>, buf: &mut String) {
    write!(buf, "{} -> ", indice).unwrap();
    for num in &vec[indice] {
        write!(buf, "{} -> ", num).unwrap();
    }
    writeln!(buf, "\\").unwrap();
}
