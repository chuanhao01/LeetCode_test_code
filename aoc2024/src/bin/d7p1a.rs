use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

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
        for num in nums {
            let mut new_possible: Vec<i64> = possible.clone();
            for i in 0..new_possible.len() {
                if new_possible[i] % num == 0 {
                    new_possible[i] /= num;
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
