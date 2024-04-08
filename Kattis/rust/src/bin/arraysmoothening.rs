// Not fully solved
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

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
    let k = fl[1];
    let ns: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    let mut counts: HashMap<u64, u64> = HashMap::new();
    for nn in ns {
        counts
            .entry(nn)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    // TODO: optimize/find a way to directly calculate
    // Larger than N
    let min_max = 300000;
    let mut values = counts.into_values().collect::<Vec<u64>>();
    for _ in 0..k {
        values.sort();
        values.reverse();
        values[0] -= 1;
    }
    values.sort();
    values.reverse();
    println!("{}", values[0]);
}
