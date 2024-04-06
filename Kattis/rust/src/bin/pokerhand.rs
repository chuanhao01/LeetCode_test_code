use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).next().unwrap();
    let cards: Vec<&str> = lines.split(' ').collect();
    let mut hm: HashMap<char, u64> = HashMap::new();
    for card in cards {
        let rank = card.chars().next().unwrap();
        hm.entry(rank).and_modify(|count| *count += 1).or_insert(1);
    }
    println!("{}", hm.values().max().unwrap());
}
