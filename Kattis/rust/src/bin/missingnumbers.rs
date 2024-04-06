use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    // Skip n
    let n: u64 = lines.next().unwrap().parse().unwrap();
    // let childs: Vec<u64> = lines.map(|num| num.parse().unwrap()).collect();
    let mut childs: Vec<u64> = Vec::new();
    for _ in 0..n {
        childs.push(lines.next().unwrap().parse::<u64>().unwrap());
    }
    let last = childs.last().unwrap().to_owned();
    if n == last {
        println!("good job");
        return;
    }

    let mut ci = childs.iter();
    let mut cc = ci.next().unwrap().to_owned();
    let mut nn = 1u64;
    while nn < last {
        if nn == cc {
            cc = ci.next().unwrap().to_owned();
        } else {
            println!("{}", nn);
        }
        nn += 1;
    }
}
