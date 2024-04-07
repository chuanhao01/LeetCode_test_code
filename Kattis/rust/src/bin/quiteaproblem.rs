use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let line = line.to_lowercase();
        if line.contains("problem") {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
