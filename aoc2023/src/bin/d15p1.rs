use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d15")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let codes = input.split(",").map(|code| {
        code.chars().fold(0, |acc, c| {
            println!("{}", c);
            let mut acc = acc + c as u32;
            acc *= 17;
            acc %= 256;
            acc
        })
    }).collect::<Vec<_>>();
    println!("{:?}", codes);
    sum = codes.iter().sum();

    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_pos() {}
}
