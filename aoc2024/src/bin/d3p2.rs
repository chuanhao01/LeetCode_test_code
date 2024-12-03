// start 21:52
// done 22:00
use regex::Regex;
use std::{
    fs::File,
    io::{Read, Result},
};

// NOTES:
// Naive approach based on notes in d2p2
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d3")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut line: String = String::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        line += l;
    }
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mut d = true;
    for (_, [s]) in re.captures_iter(&line).map(|c| c.extract()) {
        if s == "do()" {
            d = true;
        } else if s == "don't()" {
            d = false;
        } else {
            if d {
                let s = s.strip_prefix("mul(").unwrap();
                let s = s.strip_suffix(")").unwrap();
                let nums = s
                    .split(",")
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<i32>>();
                sum += nums[0] * nums[1];
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
