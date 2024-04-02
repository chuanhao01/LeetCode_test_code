use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n: i64 = lines.next().unwrap().parse().unwrap();
    for case in 0..n {
        lines.next();
        let cs: Vec<i64> = lines
            .next()
            .unwrap()
            .split(' ')
            .map(|c| c.parse().unwrap())
            .collect();
        let mut codes = HashSet::new();
        for c in cs {
            match codes.get(&c) {
                Some(_) => codes.remove(&c),
                None => codes.insert(c),
            };
        }
        for c in codes {
            println!("Case #{}: {}", case + 1, c);
        }
    }
}
