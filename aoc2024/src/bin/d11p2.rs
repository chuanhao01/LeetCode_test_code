// start 1840
// end 2200
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

    let mut sum = 0i128;
    let mut nums: HashMap<i128, i128> = HashMap::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        let nnums: Vec<i128> = l.split(" ").map(|num| num.parse().unwrap()).collect();
        for num in nnums {
            nums.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    println!("{:?}", nums);
    let ii = 75;
    // num, counts
    for _ in 0..ii {
        let mut new_nums: HashMap<i128, i128> = HashMap::new();
        for (k, v) in &nums {
            let num_c: Vec<char> = k.to_string().chars().collect();
            if k.clone() == 0 {
                new_nums
                    .entry(1)
                    .and_modify(|n| *n += v)
                    .or_insert(v.clone());
            } else if num_c.len() % 2 == 0 {
                let left_num: i128 = num_c[0..num_c.len() / 2]
                    .iter()
                    .map(|c| c.to_string())
                    .join("")
                    .parse()
                    .unwrap();
                let right_num: i128 = num_c[num_c.len() / 2..num_c.len()]
                    .iter()
                    .map(|c| c.to_string())
                    .join("")
                    .parse()
                    .unwrap();
                // new_nums.push(left_num);
                // new_nums.push(right_num);
                new_nums
                    .entry(left_num)
                    .and_modify(|n| *n += v)
                    .or_insert(v.clone());
                new_nums
                    .entry(right_num)
                    .and_modify(|n| *n += v)
                    .or_insert(v.clone());
            } else {
                // new_nums.push(num.clone() * 2024);
                new_nums
                    .entry(k.clone() * 2024)
                    .and_modify(|n| *n += v)
                    .or_insert(v.clone());
            }
        }
        nums = new_nums;
    }
    // sum = nums.len() as;
    println!("{:?}", nums);
    for (_, v) in nums {
        sum += v;
    }

    println!("sum: {}", sum);
    Ok(())
}
