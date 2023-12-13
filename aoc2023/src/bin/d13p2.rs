use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

fn get_row_reflection_index(map: &str) -> Option<i64> {
    let rows = map.split('\n').collect::<Vec<_>>();
    for split in 1..rows.len() {
        let mut check_rows = Vec::new();
        for i in 0..split {
            let right = split * 2 - 1 - i;
            if right >= rows.len() {
                // Too far right
                continue;
            }
            check_rows.push((rows[i], rows[right]));
        }
        if check_rows.iter().fold(0, |acc, check_row| {
            acc + izip!(check_row.0.chars(), check_row.1.chars())
                .fold(0, |acc, (c1, c2)| acc + if c1 != c2 { 1 } else { 0 })
        }) == 1
        {
            return Some(split as i64);
        }
    }
    None
}

fn get_col_reflection_index(map: &str) -> Option<i64> {
    let rows = map
        .split('\n')
        .map(|row| row.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut cols = Vec::new();
    let max_y = rows.len();
    let max_x = rows[0].len();
    for x in 0..max_x {
        let mut col = String::new();
        for y in 0..max_y {
            col += &rows[y][x];
        }
        cols.push(col);
    }

    for split in 1..cols.len() {
        let mut check_rows = Vec::new();
        for i in 0..split {
            let right = split * 2 - 1 - i;
            if right >= cols.len() {
                // Too far right
                continue;
            }
            check_rows.push((&cols[i], &cols[right]));
        }
        if check_rows.iter().fold(0, |acc, check_row| {
            acc + izip!(check_row.0.chars(), check_row.1.chars())
                .fold(0, |acc, (c1, c2)| acc + if c1 != c2 { 1 } else { 0 })
        }) == 1
        {
            return Some(split as i64);
        }
    }
    None
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d13")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let maps = input.split("\n\n").collect::<Vec<_>>();
    sum += maps.iter().fold(0, |acc, map| {
        if let Some(col_index) = get_col_reflection_index(map) {
            acc + col_index
        } else {
            acc
        }
    });
    sum += 100
        * maps.iter().fold(0, |acc, map| {
            if let Some(row_index) = get_row_reflection_index(map) {
                acc + row_index
            } else {
                acc
            }
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
