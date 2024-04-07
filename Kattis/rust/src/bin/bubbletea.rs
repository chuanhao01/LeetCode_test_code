use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let N: u64 = lines.next().unwrap().parse().unwrap();
    let Ns: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    let M: u64 = lines.next().unwrap().parse().unwrap();
    let Ms: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    let mut cheapest_so_far = None;
    for n in 0..N {
        let nums: Vec<u64> = lines
            .next()
            .unwrap()
            .split(' ')
            .map(|num| num.parse().unwrap())
            .collect();
        let K = nums[0];
        for k in 0..K {
            let price = Ns[n as usize] + Ms[(nums[(k + 1) as usize] - 1) as usize];
            cheapest_so_far = match cheapest_so_far {
                None => Some(price),
                Some(p) => Some(p.min(price)),
            };
        }
    }
    let X: u64 = lines.next().unwrap().parse().unwrap();
    if X < cheapest_so_far.unwrap() {
        println!("0");
    } else {
        println!("{}", X / cheapest_so_far.unwrap() - 1);
    }
}
