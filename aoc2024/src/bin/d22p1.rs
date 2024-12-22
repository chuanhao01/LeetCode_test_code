// start
// end

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
    vec,
};

use itertools::Itertools;
use queue::*;

// NOTES:
fn main() -> Result<()> {
    // let mut file_input = File::open("inputs/temp")?;
    let mut file_input = File::open("inputs/d22")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;

    let end_step = 2000;

    let mut lines = input.lines();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut num: i128 = line.parse().unwrap();
        for _ in 0..end_step {
            num = get_next_num(num);
            // println!("{}", num);
        }
        sum += num;
    }

    println!("sum: {}", sum);
    Ok(())
}

fn get_next_num(num: i128) -> i128 {
    let pp = 16777216;

    let mul_a = num * 64;
    let num = num ^ mul_a;
    let num = num % pp;

    let div_b = num / 32;
    let num = num ^ div_b;
    let num = num % pp;

    let mul_a = num * 2048;
    let num = num ^ mul_a;
    let num = num % pp;
    num
}
