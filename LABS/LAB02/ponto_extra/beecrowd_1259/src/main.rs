use std::fmt::Write;
use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    let mut input_line = String::new();
    
    handle.read_line(&mut input_line).unwrap();

    let num: usize = input_line.trim().parse::<usize>().unwrap();

    let mut vec: Vec<u32> = Vec::with_capacity(num);
    for _ in 0..num {
        input_line.clear();
        handle.read_line(&mut input_line).unwrap();
        vec.extend(input_line.split_whitespace().map(|s| s.parse::<u32>().unwrap()));
    }
    pares_impares(&mut vec);


    let mut buf = String::new();
    for item in vec {
        writeln!(buf, "{:?}", item).unwrap();
    } 
    print!("{}",buf);


}

fn pares_impares(vec: &mut [u32]) {
    
    if vec.len() > 1 {
        
        let mut i = 0;
        let mut j = vec.len() - 1;
        
        // pares à esquerda a impares à direita.
        while i < j {
            while (vec[i] % 2 == 0) && i < j { i += 1 };
            while (vec[j] % 2 == 1) && i < j { j -= 1 };
            
            vec.swap(i, j);
        }
        
        vec[..i].sort_unstable();
        vec[i..].sort_unstable_by(|a,b| b.cmp(a));
    }
}
