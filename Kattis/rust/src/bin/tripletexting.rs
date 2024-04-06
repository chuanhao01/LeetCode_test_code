use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let line = lines.next().unwrap();
    let cs: Vec<char> = line.chars().collect();
    let l = cs.len() / 3;
    let first: String = cs[0..l].iter().collect();
    let second: String = cs[l..l * 2].iter().collect();
    let third: String = cs[l * 2..l * 3].iter().collect();
    if first == second || first == third {
        println!("{}", first);
    } else {
        println!("{}", second);
    }
}
