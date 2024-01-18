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
        let raw_lens = raw_lens.split('=').collect::<Vec<_>>();
        let label = raw_lens[0].to_string();
        Self {
            label: label.clone(),
            fl: raw_lens[1].parse::<i64>().unwrap(),
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

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    let codes = input
        .split(',')
        .map(|code| {
            if code.contains('=') {
                let a = Lens::new(code);
                println!("{:?}", a);
                Instruction::Add(a)
            } else {
                Instruction::Remove(code[..code.len() - 1].to_string())
            }
        })
        .collect::<Vec<_>>();
    // println!("{:?}", codes);
    for code in codes {
        match code {
            Instruction::Add(lens) => {
                if let Some(ele) = boxes[lens._box as usize]
                    .iter_mut()
                    .find(|ele| *ele.label == lens.label)
                {
                    ele.fl = lens.fl
                } else {
                    boxes[lens._box as usize].push(lens);
                }
            }
            Instruction::Remove(label) => {
                boxes[Lens::get_hash(&label) as usize].retain(|ele| ele.label != label)
            }
        }
    }
    sum = boxes.iter().enumerate().fold(0, |acc, (box_num, _box)| {
        acc + _box.iter().enumerate().fold(0, |accc, (slot_num, lens)| {
            println!("{}", (box_num + 1) * (slot_num + 1) * lens.fl as usize);
            accc + (box_num + 1) * (slot_num + 1) * lens.fl as usize
        })
    });

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
