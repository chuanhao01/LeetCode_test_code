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

enum MapItem {
    Starting,
    Plot,
    Rock,
}
impl MapItem {
    fn new(raw_map_item: &str) -> Self {
        match raw_map_item {
            "S" => Self::Starting,
            "." => Self::Plot,
            "#" => Self::Rock,
            _ => panic!("Unknown"),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn get_new_index(&self) -> (i64, i64) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d21")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let map = input
        .split("\n")
        .map(|raw_row| {
            raw_row
                .chars()
                .map(|c| MapItem::new(&c.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (max_y, max_x) = (map.len(), map[0].len());
    // y, x, step_count
    let mut steps_q: HashSet<(i64, i64)> = HashSet::new();
    let mut cur_step = 0;
    for (y, row) in map.iter().enumerate(){
        for (x, item) in row.iter().enumerate(){
            if let MapItem::Starting = item{
                steps_q.insert((y as i64, x as i64));
                break;
            }
        }
    }
    // Find all locations he can start from
    while cur_step < 64{
        let mut next_steps: HashSet<(i64, i64)> = HashSet::new();
        for steps in steps_q{
            let (y, x) = steps;
            for possible_direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right]{
                let (cy, cx) = possible_direction.get_new_index();
                let (ny, nx) = (y + cy, x + cx);
                if ny < 0 || ny >= max_y as i64 || nx < 0 || nx >= max_x as i64{
                    continue;
                }
                if let MapItem::Rock = map[ny as usize][nx as usize]{
                    continue;
                }
                next_steps.insert((ny, nx));
            }
        }
        steps_q = next_steps;
        cur_step += 1;
    }
    println!("{:?}", steps_q.len());


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
