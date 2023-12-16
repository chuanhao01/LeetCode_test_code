use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    fl: i64,
    _box: u32,
}
impl Lens {
    fn new(raw_lens: &str) -> Self {
        let label = raw_lens[..2].to_string();
        Self {
            label: label.clone(),
            fl: raw_lens[3..4].parse::<i64>().unwrap(),
            _box: Self::get_hash(&label),
        }
    }
    fn get_hash(label: &str) -> u32 {
        label.chars().fold(0, |acc, c| {
            let mut acc = acc + c as u32;
            acc *= 17;
            acc %= 256;
            acc
        })
    }
}
#[derive(Debug, Clone)]
enum Instruction {
    Remove(String),
    Add(Lens),
}


fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d15")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let codes = input
        .split(',')
        .map(|code| {
            let label = code[..2].to_string();
            if &code[2..3] == "=" {
                let a = Lens::new(code);
                println!("{:?}", a);
                Instruction::Add(a)
            } else {
                Instruction::Remove(label)
            }
        })
        .collect::<Vec<_>>();
    // let boxes = vec![Vec::<Lens>new(); 256];
    println!("{:?}", codes);

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
