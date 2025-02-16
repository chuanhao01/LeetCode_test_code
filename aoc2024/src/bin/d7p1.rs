// start 2125
// end 2145

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
        // println!("{}", check);
        // println!("{:?}", nums);
        let mut possible: Vec<i64> = Vec::from([nums.remove(0)]);
        for num in nums {
            let mut new_possible = possible.clone();
            for i in 0..new_possible.len() {
                new_possible[i] += num;
            }
            for i in 0..possible.len() {
                new_possible.push(possible[i] * num);
            }
            possible = new_possible;
        }
        if possible.contains(&check) {
            sum += check;
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
