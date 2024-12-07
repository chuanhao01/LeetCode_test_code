// part 2 does not work
// IDK edge case

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

// NOTES:
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d7")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        let mut l = l.split(": ");
        let check: i64 = l.next().unwrap().parse().unwrap();
        let mut nums: Vec<i64> = l
            .next()
            .unwrap()
            .split(" ")
            .map(|num| num.parse().unwrap())
            .collect();
        let new_check = nums.remove(0);
        nums.reverse();
        let mut possible: Vec<i64> = Vec::from([check]);
        for num in nums.clone() {
            let mut new_possible: Vec<i64> = possible.clone();
            for i in 0..new_possible.len() {
                if new_possible[i] % num == 0 {
                    new_possible[i] /= num;
                }
            }
            for i in 0..possible.len() {
                let num_c = num.to_string().chars().collect::<Vec<char>>().len();
                let mut possible_c = possible[i].to_string().chars().collect::<Vec<char>>();
                if possible_c.len() as i64 - num_c as i64 > 0 {
                    if possible_c[(possible_c.len() - num_c)..possible_c.len()]
                        .iter()
                        .map(|c| c.to_string())
                        .join("")
                        .parse::<i64>()
                        .unwrap()
                        == num
                    {
                        // println!("{}", possible[i]);
                        // println!("{}, {}", possible_c.len(), num_c);
                        let mut neg = false;
                        if possible_c[0] == '-' {
                            possible_c.remove(0);
                            neg = true;
                        }
                        if possible_c.len() as i32 - num_c as i32 > 0 {
                            let nn: i64 = possible_c[0..possible_c.len() - num_c]
                                .iter()
                                .map(|c| c.to_string())
                                .join("")
                                .parse()
                                .unwrap();
                            new_possible.push(if neg { -nn } else { nn });
                        } else if possible_c.len() - num_c == 0 {
                            new_possible.push(0);
                        }
                    }
                }
            }
            for i in 0..possible.len() {
                new_possible.push(possible[i] - num);
            }
            possible = new_possible;
        }
        if possible.contains(&new_check) {
            sum += check;
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
