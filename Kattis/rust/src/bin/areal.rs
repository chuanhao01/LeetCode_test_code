use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let a: i64 = line.parse().unwrap();
        println!("{}", 4f64 * (a as f64).sqrt());
    }
}
