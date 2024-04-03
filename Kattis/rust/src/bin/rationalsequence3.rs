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
        let n: i64 = line.next().unwrap().parse().unwrap();
        fn get_pq(n: i64) -> (i64, i64) {
            if n == 1 {
                (1, 1)
            } else if n % 2 == 0 {
                let (p, q) = get_pq(n / 2);
                (p, p + q)
            } else {
                let (p, q) = get_pq((n - 1) / 2);
                (p + q, q)
            }
        }
        let (new_p, new_q) = get_pq(n);
        println!("{} {}/{}", case + 1, new_p, new_q);
    }
}
