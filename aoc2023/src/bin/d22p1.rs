use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    f64::INFINITY,
    fs::File,
    io::{Read, Result},
    mem::Discriminant,
    slice::Split,
};

use itertools::{izip, Itertools};

// Include min and max
#[derive(Debug)]
struct Interval {
    min: i64,
    max: i64,
}
impl Interval {
    fn new(a: i64, b: i64) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }
}

#[derive(Debug)]
struct Brick {
    x: Interval,
    y: Interval,
    z: Interval,
}
impl Brick {
    fn new(left: Vec<i64>, right: Vec<i64>) -> Self {
        Self {
            x: Interval::new(left[0], right[0]),
            y: Interval::new(left[1], right[1]),
            z: Interval::new(left[2], right[2]),
        }
    }
    /// If other is under self
    fn under(&self, other: &Self) -> bool {
        other.z.max < self.z.min
    }
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d22")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;
    let bricks = input
        .split('\n')
        .map(|raw_row| {
            let mut raw_row = raw_row.split('~');
            let left = raw_row
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let right = raw_row
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            Brick::new(left, right)
        })
        .collect::<Vec<Brick>>();

    println!("{:?}", bricks);
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
