use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let p: i64 = lines.next().unwrap().parse().unwrap();
    for case in 0..p {
        let line = lines.next().unwrap();
        let mut line = line.split(' ');
        // Skip case number
        line.next();
        let nums: Vec<i64> = line
            .next()
            .unwrap()
            .split('/')
            .map(|num| num.parse().unwrap())
            .collect();
        let p = nums[0];
        let q = nums[1];
        fn rec_n(p: i64, q: i64) -> i64 {
            if p == 1 && q == 1 {
                return 1;
            }
            if p > q {
                // right side
                2 * rec_n(p - q, q) + 1
            } else {
                2 * rec_n(p, q - p)
            }
        }
        println!("{} {}", case + 1, rec_n(p, q));
    }
}
