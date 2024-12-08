// start 1932
// end 2020

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

// NOTES:
// Thought too hard
// Basically on the same slanted line
// Also include 1 off, and check later
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d8")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut possible_chars: HashSet<char> = HashSet::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        map.push(l.chars().collect());
        for c in l.chars().unique() {
            if c != '.' {
                possible_chars.insert(c);
            }
        }
    }
    // println!("{:?}", map);
    // println!("{:?}", possible_chars);

    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();
    for pc in possible_chars {
        let mut locations: Vec<(i32, i32)> = Vec::new();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == pc {
                    locations.push((y as i32, x as i32));
                }
            }
        }
        println!("{}, {:?}", pc, locations);
        for ll in &locations {
            let current_location = ll;
            for l in &locations {
                if l == current_location {
                    continue;
                }
                let diff = (current_location.0 - l.0, current_location.1 - l.1);
                let mut ds: Vec<(i32, i32)> = Vec::new();
                for m in 1..(map.len().max(map[0].len()) as i32) {
                    let nd = (
                        current_location.0 - diff.0 * m,
                        current_location.1 - diff.1 * m,
                    );
                    let od = (l.0 + diff.0 * m, l.1 + diff.1 * m);
                    ds.push(nd);
                    ds.push(od);
                }
                for dd in ds {
                    if dd.0 < 0
                        || dd.0 >= map.len() as i32
                        || dd.1 < 0
                        || dd.1 >= map[0].len() as i32
                    {
                        continue;
                    } else {
                        anti_nodes.insert(dd);
                    }
                }
            }
        }
    }
    sum += anti_nodes.len();

    println!("sum: {}", sum);
    Ok(())
}
