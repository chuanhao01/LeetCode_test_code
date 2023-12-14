use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read, Result},
};

use itertools::izip;
use num::iter::Range;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Map {
    direction: Direction,
    body: Vec<Vec<Legend>>,
}
// impl Map {
//     fn check_body(&self, other: &Self) -> bool {
//         for (s_row, o_row) in izip!(&self.body, &other.body) {
//             for (s, o) in izip!(s_row, o_row) {
//                 if !matches!(s, o) {
//                     return false;
//                 }
//             }
//         }
//         true
//     }
// }
// impl PartialEq for Map {
//     fn eq(&self, other: &Self) -> bool {
//         self.direction == other.direction && self.check_body(other)
//     }
// }
// impl Eq for Map {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Legend {
    Round,
    Cube,
    Empty,
}
impl Legend {
    fn new(raw_legend: &str) -> Self {
        match raw_legend {
            "O" => Self::Round,
            "#" => Self::Cube,
            "." => Self::Empty,
            _ => panic!("Unkown raw_legend: {}", raw_legend),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn get_rocks(&self, map: &Vec<Vec<Legend>>) -> Vec<(usize, usize)> {
        let max_x = map[0].len();
        let mut round_rocks: Vec<(usize, usize)> = Vec::new();
        match self {
            Self::North => {
                for (y, row) in map.iter().enumerate() {
                    for (x, item) in row.iter().enumerate() {
                        if let Legend::Round = item {
                            round_rocks.push((y, x));
                        }
                    }
                }
            }
            Self::South => {
                for (y, row) in map.iter().enumerate().rev() {
                    for (x, item) in row.iter().enumerate() {
                        if let Legend::Round = item {
                            round_rocks.push((y, x));
                        }
                    }
                }
            }
            Self::East => {
                for x in (0..max_x).rev() {
                    for y in 0..map.len() {
                        let item = &map[y][x];
                        if let Legend::Round = item {
                            round_rocks.push((y, x));
                        }
                    }
                }
            }
            Self::West => {
                for x in 0..max_x {
                    for y in 0..map.len() {
                        let item = &map[y][x];
                        if let Legend::Round = item {
                            round_rocks.push((y, x));
                        }
                    }
                }
            }
        };
        round_rocks
    }
    fn move_rocks(&self, map: Vec<Vec<Legend>>) -> Vec<Vec<Legend>> {
        let mut new_map = map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| {
                        if let Legend::Round = item {
                            Legend::Empty
                        } else {
                            item.clone()
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let round_rocks = self.get_rocks(&map);
        match self {
            Self::North => {
                for round_rock in round_rocks {
                    let (rock_y, rock_x) = round_rock;
                    let mut possible_y: Option<usize> = None;
                    for y in (0..(rock_y + 1)).rev() {
                        if let Legend::Empty = new_map[y][rock_x] {
                            possible_y = Some(y);
                        } else {
                            break;
                        }
                    }
                    if let Some(y) = possible_y {
                        new_map[y][rock_x] = Legend::Round;
                    }
                }
            }
            Self::South => {
                for round_rock in round_rocks {
                    let (rock_y, rock_x) = round_rock;
                    let mut possible_y: Option<usize> = None;
                    for y in (rock_y)..new_map.len() {
                        if let Legend::Empty = new_map[y][rock_x] {
                            possible_y = Some(y);
                        } else {
                            break;
                        }
                    }
                    if let Some(y) = possible_y {
                        new_map[y][rock_x] = Legend::Round;
                    }
                }
            }
            Self::East => {
                for round_rock in round_rocks {
                    let (rock_y, rock_x) = round_rock;
                    let mut possible_x: Option<usize> = None;
                    for x in rock_x..new_map[0].len() {
                        if let Legend::Empty = new_map[rock_y][x] {
                            possible_x = Some(x);
                        } else {
                            break;
                        }
                    }
                    if let Some(x) = possible_x {
                        new_map[rock_y][x] = Legend::Round;
                    }
                }
            }
            Self::West => {
                for round_rock in round_rocks {
                    let (rock_y, rock_x) = round_rock;
                    let mut possible_x: Option<usize> = None;
                    for x in (0..rock_x + 1).rev() {
                        if let Legend::Empty = new_map[rock_y][x] {
                            possible_x = Some(x);
                        } else {
                            break;
                        }
                    }
                    if let Some(x) = possible_x {
                        new_map[rock_y][x] = Legend::Round;
                    }
                }
            }
        }
        new_map
    }
}

fn roll_map(map: Vec<Vec<Legend>>) -> i64 {
    let mut cache: HashMap<Vec<Vec<Legend>>, (i64, Vec<Vec<Legend>>)> = HashMap::new();
    let r = 300;
    let mut i = 0;
    let mut vals = Vec::new();
    let mut map = map;
    for _ in 0..r {
        if let Some(new_map) = cache.get(&map) {
            // Off by 1
            // Pattern is first 200 to get mapping
            // Repeat 28 times
            // Need to get to the next map
            if new_map.0 == 187 {
                println!("ans: {}", count_map(&new_map.1));
                return 0;
            }
            vals.push(new_map.0);
            map = new_map.1.clone();
        } else {
            let mut new_map = map.clone();
            for dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                new_map = dir.move_rocks(new_map.clone());
            }
            cache.insert(map.clone(), (i, new_map.clone()));
            vals.push(i);
            i += 1;
            map = new_map;
        }
    }
    // Using math and heuristics, find the recurring pattern manually
    // Could code it out i guess
    println!("{:?}", vals);
    println!("{:?}", cache.len());
    count_map(&map)
}
fn find_seq(vals: &Vec<i64>) -> Option<(usize, Vec<i64>)> {
    for i in 0..vals.len() {
        let check = vals[i..].to_vec();
        if check.len() % 2 == 0 {
            let left = check[0..check.len() / 2].to_vec();
            let right = check[check.len() / 2..].to_vec();
            if izip!(left.clone(), right).all(|(left, right)| left == right) {
                return Some((i, left));
            }
        }
    }
    None
}
fn count_map(map: &Vec<Vec<Legend>>) -> i64 {
    map.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .filter(|item| matches!(item, Legend::Round))
            .count()
            * (map.len() - y)
    }) as i64
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d14")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let map = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| Legend::new(&c.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    sum = roll_map(map);

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
