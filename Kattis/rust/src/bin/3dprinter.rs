use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let n: i64 = line.parse().unwrap();
        let mut days = n;
        let mut nn = 0i64;
        while 2i64.pow(nn as u32) <= n {
            let printers = 2i64.pow(nn as u32);
            let new_days = if n % printers == 0 {
                nn + n / printers
            } else {
                nn + n / printers + 1
            };
            days = days.min(new_days);
            nn += 1;
        }
        println!("{}", days);
    }
}
