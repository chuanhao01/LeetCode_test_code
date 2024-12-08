// start 1911
// end 1930

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

// NOTES:
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
        while !locations.is_empty() {
            let current_location = locations.pop().unwrap();
            for l in &locations {
                let diff = (current_location.0 - l.0, current_location.1 - l.1);
                let nd = (
                    current_location.0 - diff.0 * 2,
                    current_location.1 - diff.1 * 2,
                );
                let od = (l.0 + diff.0 * 2, l.1 + diff.1 * 2);
                for dd in [nd, od] {
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
    sum = anti_nodes.len();

    println!("sum: {}", sum);
    Ok(())
}
