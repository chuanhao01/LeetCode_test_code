use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i64> = line.split(' ').map(|num| num.parse().unwrap()).collect();
        let mut towers = nums[..6].to_owned();
        let h1 = nums[6];
        let h2 = nums[7];
        towers.sort();
        for h in [h1, h2] {
            let r = h - towers[5];
            for i in 0..5 {
                for j in i + 1..5 {
                    let sum = towers[i] + towers[j];
                    if r == sum {
                        let smaller_towers: Vec<i64> = (0..5)
                            .filter(|&n| n != j && n != i)
                            .map(|n| towers[n])
                            .collect();
                        if smaller_towers.iter().sum::<i64>() == if h1 == h { h2 } else { h1 } {
                            println!(
                                "{} {} {} {} {} {}",
                                towers[5],
                                towers[j],
                                towers[i],
                                smaller_towers[2],
                                smaller_towers[1],
                                smaller_towers[0]
                            );
                        }
                        return;
                    }
                }
            }
        }
    }
}
