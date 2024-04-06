use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut nums: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    nums.sort();
    let a = nums[0];
    let b = nums[1];
    let c = nums[2];
    let order: Vec<char> = lines.next().unwrap().chars().collect();
    let mut f: Vec<String> = Vec::new();
    for ch in order {
        let cc = match ch {
            'A' => a.to_string(),
            'B' => b.to_string(),
            'C' => c.to_string(),
            _ => panic!("whoops"),
        };
        f.push(cc);
    }
    println!("{}", f.join(" "));
}
