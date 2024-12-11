// start 1820
// end 1840
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

// NOTES:
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d11")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0i64;
    let mut nums: Vec<i64> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        nums = l.split(" ").map(|num| num.parse().unwrap()).collect();
    }
    let ii = 25;
    for _ in 0..ii {
        let mut new_nums: Vec<i64> = Vec::new();
        for num in &nums {
            let num_c: Vec<char> = num.to_string().chars().collect();
            if num.clone() == 0 {
                new_nums.push(1);
            } else if num_c.len() % 2 == 0 {
                let left_num: i64 = num_c[0..num_c.len() / 2]
                    .iter()
                    .map(|c| c.to_string())
                    .join("")
                    .parse()
                    .unwrap();
                let right_num: i64 = num_c[num_c.len() / 2..num_c.len()]
                    .iter()
                    .map(|c| c.to_string())
                    .join("")
                    .parse()
                    .unwrap();
                new_nums.push(left_num);
                new_nums.push(right_num);
            } else {
                new_nums.push(num.clone() * 2024);
            }
        }
        nums = new_nums;
    }
    sum = nums.len() as i64;

    println!("sum: {}", sum);
    Ok(())
}
