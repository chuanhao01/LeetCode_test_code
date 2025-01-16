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

#[derive(Debug)]
enum Schematic {
    Key([i64; 5]),
    Lock([i64; 5]),
}
impl Schematic {
    fn from_input(input: Vec<Vec<char>>) -> Self {
        let mut heights = [0; 5];
        for x in 0..5 {
            heights[x] = (1..6).filter(|y| input[*y][x] == '#').count() as i64;
        }
        if input[0][0] == '#' {
            // lock
            Self::Lock(heights)
        } else {
            // key
            Self::Key(heights)
        }
    }
}

// NOTES:
fn main() -> Result<()> {
    // let mut file_input = File::open("inputs/temp")?;
    let mut file_input = File::open("inputs/d25")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;

    let mut lines = input.lines();
    let mut schematics: Vec<Schematic> = Vec::new();
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in input.split("\n") {
        if line.is_empty() {
            if !schematic.is_empty() {
                schematics.push(Schematic::from_input(schematic));
                schematic = Vec::new();
            }
            continue;
        }
        schematic.push(line.chars().collect());
    }
    println!("{:?}", schematics);

    let mut pairs: HashSet<(usize, usize)> = HashSet::new();
    let locks = schematics
        .iter()
        .fold(Vec::new(), |mut acc: Vec<[i64; 5]>, schematic| {
            if let Schematic::Lock(a) = schematic {
                acc.push(a.clone())
            };
            acc
        });
    let keys = schematics
        .iter()
        .fold(Vec::new(), |mut acc: Vec<[i64; 5]>, schematic| {
            if let Schematic::Key(a) = schematic {
                acc.push(a.clone())
            };
            acc
        });
    for (l, lock) in locks.iter().enumerate() {
        for (k, key) in keys.iter().enumerate() {
            let mut fit = true;
            for x in 0..5 {
                if lock[x] + key[x] > 5 {
                    fit = false;
                    break;
                }
            }
            if fit {
                pairs.insert((l, k));
            }
        }
    }
    sum = pairs.len();

    println!("sum: {}", sum);
    Ok(())
}
