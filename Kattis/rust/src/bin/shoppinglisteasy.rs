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
    let mut item_count: HashMap<String, u64> = HashMap::new();
    for _ in 0..n {
        for item in lines.next().unwrap().split(' ') {
            item_count
                .entry(item.to_owned())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    let mut items: Vec<String> = item_count
        .iter()
        .filter(|(_k, &v)| v == n)
        .map(|(v, _)| v.to_owned())
        .collect();
    items.sort();
    println!("{}", items.len());
    for item in items {
        println!("{}", item);
    }
}
