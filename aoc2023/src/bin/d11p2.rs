use core::panic;
use queue::*;
use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Result},
    ops::{Add, IndexMut},
};

use itertools::izip;

#[derive(Debug, Clone)]
enum Thing {
    Galaxy,
    Nothing,
}
impl Thing {
    fn from(s: &str) -> Self {
        match s {
            "#" => Self::Galaxy,
            "." => Self::Nothing,
            _ => panic!("Wrong input thing: {}", s),
        }
    }
}

struct Map {
    body: Vec<Vec<Thing>>,
    new_rows: Vec<usize>,
    new_columns: Vec<usize>,
}
impl Map {
    fn new(map: Vec<Vec<Thing>>) -> Self {
        let max_x = map[0].len();
        let mut rows_to_add = Vec::new();
        let mut columns_to_add = Vec::new();
        for (y, row) in map.iter().enumerate() {
            if row.iter().all(|thing| matches!(thing, Thing::Nothing)) {
                rows_to_add.push(y);
            }
        }
        for x in 0..max_x {
            let column = map.iter().map(|row| row[x].clone()).collect::<Vec<_>>();
            if column.iter().all(|thing| matches!(thing, Thing::Nothing)) {
                columns_to_add.push(x);
            }
        }
        Self {
            body: map,
            new_rows: rows_to_add,
            new_columns: columns_to_add,
        }
    }
    fn solve_qn(&self) {
        let mut galaxies = Vec::new();
        for (y, row) in self.body.iter().enumerate() {
            for (x, thing) in row.iter().enumerate() {
                if let Thing::Galaxy = thing {
                    galaxies.push((y, x));
                }
            }
        }
        let mut sum = 0;
        for i1 in 0..galaxies.len() {
            let left = galaxies[i1];
            (i1..galaxies.len()).for_each(|i2| {
                let right = galaxies[i2];
                sum += left.0.abs_diff(right.0) + left.1.abs_diff(right.1);
                sum += self.new_rows.iter().map(|row|{
                    let ll = left.0.min(right.0);
                    let rr = left.0.max(right.0);
                    if ll < *row && *row < rr{
                        1000000 - 1
                    } else {
                        0
                    }
                }).reduce(|acc, num|{acc + num}).unwrap();
                sum += self.new_columns.iter().map(|col|{
                    let ll = left.1.min(right.1);
                    let rr = left.1.max(right.1);
                    if ll < *col && *col < rr{
                        1000000 - 1
                    } else {
                        0
                    }
                }).reduce(|acc, num|{acc + num}).unwrap();
            });
        }
        println!("{}", sum);
    }
}


fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d11")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let map = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| Thing::from(&c.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let map = Map::new(map);
    map.solve_qn();

    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_seq() {}
}
