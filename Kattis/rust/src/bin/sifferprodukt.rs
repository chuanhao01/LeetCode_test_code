use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut num = stdin.lock().lines().map(|l| l.unwrap()).next().unwrap();
    while num.len() > 1 {
        let mut n = 1u64;
        for c in num.chars() {
            let cc: u64 = c.to_string().parse().unwrap();
            if cc > 0 {
                n *= cc;
            }
        }
        num = n.to_string();
    }
    println!("{}", num);
}
