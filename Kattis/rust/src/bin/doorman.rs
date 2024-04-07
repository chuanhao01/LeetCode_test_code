use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let x: i64 = lines.next().unwrap().parse().unwrap();
    let ppll = lines.next().unwrap();
    let mut ppl = ppll.chars().peekable();
    let mut m = 0i64;
    let mut f = 0i64;
    while ppl.peek().is_some() {
        // Just take in ppl as per normal
        match ppl.next().unwrap() {
            'M' => m += 1,
            'W' => f += 1,
            _ => panic!("nah"),
        };
        if (m - f).abs() > x {
            // Process next and check again (assuming a skip)
            match ppl.next().unwrap() {
                'M' => m += 1,
                'W' => f += 1,
                _ => panic!("nah"),
            };
            if (m - f).abs() > x {
                println!("{}", m + f - 2);
                return;
            }
        }
    }
    println!("{}", m + f);
}
