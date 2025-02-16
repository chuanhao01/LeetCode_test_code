// start 1450
// end 1515
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

// NOTES:
// Should have, could have been faster
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d10")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0i64;
    let mut map: Vec<Vec<char>> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        map.push(l.chars().collect());
    }
    let mut starting_pos: Vec<(i64, i64)> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '0' {
                starting_pos.push((y as i64, x as i64));
            }
        }
    }
    for pos in starting_pos {
        sum += dfs(&map, pos, 0).len() as i64;
    }

    println!("sum: {}", sum);
    Ok(())
}

fn dfs(map: &Vec<Vec<char>>, location: (i64, i64), check_no: i64) -> HashSet<(i64, i64)> {
    // end cond
    if check_no == 9 {
        if map[location.0 as usize][location.1 as usize] == '9' {
            return HashSet::from([location]);
        } else {
            return HashSet::default();
        }
    }
    let c = map[location.0 as usize][location.1 as usize];
    if c == '.' || c.to_string() != check_no.to_string() {
        return HashSet::default();
    }
    let mut s: HashSet<(i64, i64)> = HashSet::new();
    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let ny = location.0 + d.0;
        let nx = location.1 + d.1;
        if ny < 0 || ny >= map.len() as i64 || nx < 0 || nx >= map[0].len() as i64 {
            continue;
        }
        let ns = dfs(map, (ny, nx), check_no + 1);
        s.extend(ns);
    }
    s
}
