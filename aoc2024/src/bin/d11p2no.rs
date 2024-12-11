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
    let original_nums = nums.clone();
    for original_num in original_nums {
        let mut ii = 0;
        let mut dp: HashMap<i64, Vec<i64>> = HashMap::new();
        // num, (first, second*)
        let mut seen: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut nums = vec![original_num];
        while !nums.is_empty() {
            let mut new_nums: Vec<i64> = Vec::new();
            for num in &nums {
                if let Some(old_nums) = dp.get(num) {
                    // Skip and remove no
                    // seen.entry(num.clone()).and_modify(|v| v.push(ii));
                    continue;
                } else {
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
                    dp.insert(num.clone(), new_nums.clone());
                    // seen.insert(num.clone(), vec![ii]);
                }
            }
            println!("{}, {:?}", ii, nums);
            nums = new_nums;
            ii += 1;
        }
        // println!("{:?}", seen);
        let mut nums_count: HashMap<i64, i64> = HashMap::from([(original_num, 1i64)]);
        for _ in 0..4 {
            let mut new_nums_counts: HashMap<i64, i64> = HashMap::new();
            for (k, v) in &nums_count {
                let next_nums = dp.get(k).unwrap();
                println!("{:?}, {}, {}", next_nums, k, v);
                for next_num in next_nums {
                    new_nums_counts
                        .entry(next_num.clone())
                        .and_modify(|n| *n += v.clone())
                        .or_insert(v.clone());
                }
            }
            nums_count = new_nums_counts;
        }
        println!("{:?}", nums_count);
        break;
    }
    // println!(
    //     "{:?}",
    //     seen.clone()
    //         .into_iter()
    //         .filter(|(k, v)| v.len() > 1)
    //         .collect::<HashMap<i64, Vec<i64>>>()
    // );
    // sum = nums.len() as i64;

    println!("sum: {}", sum);
    Ok(())
}
