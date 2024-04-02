use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line.len() <= 2 {
            // Smaller than 99
            println!("{}", 99);
            continue;
        }
        let n: i64 = line.parse().unwrap();
        let r = 99 - n % 100;
        if r <= 50 {
            println!("{}", n - n % 100 + 99);
        } else {
            println!("{}", n - n % 100 - 1);
        }
    }
}
