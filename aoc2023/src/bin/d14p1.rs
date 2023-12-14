use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

#[derive(Debug, Clone)]
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

fn roll_map(map: Vec<Vec<Legend>>) -> Vec<Vec<Legend>> {
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

    let mut round_rocks: Vec<(usize, usize)> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if let Legend::Round = item {
                round_rocks.push((y, x));
            }
        }
    }

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
    new_map
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
    let new_map = roll_map(map);
    sum = new_map.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .filter(|item| matches!(item, Legend::Round))
            .count()
            * (new_map.len() - y)
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
