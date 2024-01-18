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

fn expand_universe(map: Vec<Vec<Thing>>) -> Vec<Vec<Thing>> {
    let mut new_map = map.clone();
    let max_y = map.len();
    let max_x = map[0].len();
    let mut rows_to_add = 0;
    let mut columns_to_add = 0;
    for (y, row) in map.iter().enumerate() {
        if row.iter().all(|thing| matches!(thing, Thing::Nothing)) {
            new_map.insert(y + rows_to_add, vec![Thing::Nothing; max_x].to_vec());
            rows_to_add += 1;
            // rows_to_add.push(y);
        }
    }
    for x in 0..max_x {
        let column = map.iter().map(|row| row[x].clone()).collect::<Vec<_>>();
        if column.iter().all(|thing| matches!(thing, Thing::Nothing)) {
            new_map.iter_mut().for_each(|row| row.insert(x + columns_to_add, Thing::Nothing));
            columns_to_add += 1;
            // columns_to_add.push(x);
        }
    }
    new_map
}
fn solve_qn(map: Vec<Vec<Thing>>){
    let map = expand_universe(map);
    let mut galaxies = Vec::new();
    for (y, row) in map.iter().enumerate(){
        for (x, thing) in row.iter().enumerate(){
            if let Thing::Galaxy = thing{
                galaxies.push((y, x));
            }
        }
    }
    let mut sum = 0;
    for i1 in 0..galaxies.len(){
        let left = galaxies[i1];
        (i1..galaxies.len()).for_each(|i2| {
            let right = galaxies[i2];
            sum += left.0.abs_diff(right.0) + left.1.abs_diff(right.1);
        });
    }
    println!("{}", sum);
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
    // println!("{:?}", map);
    solve_qn(map);

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
