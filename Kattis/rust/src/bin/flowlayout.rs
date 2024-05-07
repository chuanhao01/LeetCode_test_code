use std::io::{self, BufRead};

// TODO
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap()).peekable();
    let mut ans: Vec<String> = Vec::new();
    while lines.peek().unwrap() != "0" {
        let max_width: u64 = lines.next().unwrap().parse().unwrap();
        let mut boxes: Vec<(u64, u64)> = Vec::new();
        let mut new_boxes: Vec<(u64, u64)> = Vec::new();
        while lines.peek().unwrap() != "-1 -1" {
            let _box: Vec<u64> = lines
                .next()
                .unwrap()
                .split(' ')
                .map(|num| num.parse().unwrap())
                .collect();
            boxes.push((_box[0], _box[1]));
        }
    }
}
