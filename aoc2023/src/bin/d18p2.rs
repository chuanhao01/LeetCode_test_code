use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    f64::INFINITY,
    fs::File,
    io::{Read, Result},
    mem::Discriminant,
    slice::Split,
};

use itertools::izip;
use priority_queue::{DoublePriorityQueue, PriorityQueue};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn new(raw_direction: &str) -> Self {
        match raw_direction {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("Wrong raw_direction: {}", raw_direction),
        }
    }
    fn get_new_index(&self) -> (i64, i64) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}


fn p1(instructions: Vec<(Direction, i64)>) {
    // Using shoelace formular
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut current: (f64, f64) = (0_f64, 0_f64);
    let mut verticies: Vec<(f64, f64)> = vec![current];
    for instruction in &instructions {
        let (direction, steps) = instruction;
        let (y, x) = current;
        let (cy, cx) = direction.get_new_index();
        let (ny, nx) = (y + (cy * steps) as f64, x + (cx * steps) as f64);
        current = (ny, nx);
        verticies.push((ny, nx));
    }
    let mut area = 0_f64; // Accumulates area
    println!("{:?}", verticies);
    // Get area inside polygon
    let mut j = verticies.len() - 1;
    for i in 0..verticies.len() {
        let ii = verticies[i];
        let jj = verticies[j];
        area += (ii.0 + jj.0) * (ii.1 - jj.1);
        j = i;
    }
    area = area.abs();
    area /= 2_f64;
    println!("area: {}", area);
    // Using pick's therom
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let b = &instructions
        .iter()
        .fold(0_f64, |acc, instruction| acc + instruction.1 as f64);
    let i = area + 1_f64 - b / 2_f64;
    println!("area:{}", i + b);
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d18")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;
    let instructions = input
        .split('\n')
        .map(|row| row.split(' ').collect::<Vec<_>>()[2..3].to_vec())
        .map(|raw_row| {
            let hex_string = raw_row[0].replace(['(', ')', '#'], "");
            let raw_direction = match &hex_string[5..6]{
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                unkown_direction => panic!("Unkown direction no.: {}", unkown_direction)
            };
            (
                Direction::new(raw_direction),
                i64::from_str_radix(&hex_string[..5], 16).unwrap()
            )
        })
        .collect::<Vec<_>>();
    p1(instructions);

    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_direction_get() {}
}
