use std::io::{self, BufRead};
// TODO: Not done, could not solve in the middle of the night

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let nums: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.to_owned().parse().unwrap())
        .collect();
    let n = nums[0];
    let m = nums[1];
    let mut sets: Vec<(u64, u64)> = Vec::new();
    for _ in 0..m {}
}
