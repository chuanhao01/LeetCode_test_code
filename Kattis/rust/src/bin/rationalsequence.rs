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
        let mut p = nums[0];
        let mut q = nums[1];
        fn next_pq(p: i64, q: i64) -> (i64, i64) {
            if p == 1 && q == 1 {
                return (1, 2);
            }
            if p > q {
                // right side
                if q == 1 {
                    (1, p + 1)
                } else {
                    let (prev_p, prev_q) = next_pq(p - q, q);
                    (prev_p, prev_p + prev_q)
                }
            } else {
                let prev_q = q - p;
                (q, prev_q)
            }
        }
        // while q != 1 {
        //     let (new_p, new_q) = next_pq(p, q);
        //     println!("{} {}/{}", case + 1, new_p, new_q);
        //     p = new_p;
        //     q = new_q;
        // }
        let (new_p, new_q) = next_pq(p, q);
        println!("{} {}/{}", case + 1, new_p, new_q);
    }
}
