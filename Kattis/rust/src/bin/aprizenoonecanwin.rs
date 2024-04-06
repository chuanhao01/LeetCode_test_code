use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let fl: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    let n = fl[0];
    let x = fl[1];
    let mut nps: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    nps.sort();
    if n == 1 {
        println!("1");
        return;
    }
    for i in 0..(nps.len() - 1) {
        if nps[i + 1] + nps[i] > x {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", n);
}
