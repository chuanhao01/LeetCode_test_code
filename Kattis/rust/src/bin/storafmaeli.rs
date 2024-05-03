use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let age: u64 = line.parse().unwrap();
        if age % 10 == 0 {
            println!("Jebb");
        } else {
            println!("Neibb");
        }
    }
}
