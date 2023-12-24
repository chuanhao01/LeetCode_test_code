use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    f64::INFINITY,
    fs::File,
    io::{Read, Result},
    mem::Discriminant,
    slice::Split,
    thread,
};

use itertools::{izip, Itertools};

enum Tile {
    Path,
    Forest,
    Slope(Direction),
}
impl Tile {
    fn new(raw_tile: &str) -> Self {
        match raw_tile {
            "." => Self::Path,
            "#" => Self::Forest,
            "^" => Self::Path,
            "<" => Self::Path,
            ">" => Self::Path,
            "v" => Self::Path,
            _ => panic!("Unknown raw_tile: {}", raw_tile),
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
    let builder = thread::Builder::new()
        .name("reductor".into())
        .stack_size(80 * 1024 * 1024); // 32MB of stack space

    let mut file_input = File::open("inputs/d23")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let map = input
        .split('\n')
        .map(|raw_row| {
            raw_row
                .chars()
                .map(|c| Tile::new(&c.to_string()))
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();
    let max_y = map.len() as i64;
    let max_x = map[0].len() as i64;

    let mut starting: (i64, i64) = (0, 0);
    let mut ending: (i64, i64) = (0, 0);
    for x in 0..max_x {
        if let Tile::Path = &map[0][x as usize] {
            starting = (0, x);
        }
        if let Tile::Path = &map[(max_y - 1) as usize][x as usize] {
            ending = (max_y - 1, x);
        }
    }

    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    let mut q: Vec<(i64, i64, i64)> = Vec::new();
    let mut longest = 0;
    fn dfs(
        loc: (i64, i64),
        map: &Vec<Vec<Tile>>,
        end: (i64, i64),
        seen: HashSet<(i64, i64)>,
    ) -> i64 {
        let max_y = map.len() as i64;
        let max_x = map[0].len() as i64;
        if loc == end {
            return 0;
        }
        let (y, x) = loc;
        let mut seen = seen;
        seen.insert(loc);
        let mut max_dist = 0;
        for possible_direction in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ] {
            let (cy, cx) = possible_direction.get_new_index();
            let (ny, nx) = (y + cy, x + cx);
            if ny < 0 || ny >= max_y || nx < 0 || nx >= max_x {
                // Skip if seen before or not possible
                continue;
            }
            if seen.contains(&(ny, nx)) || matches!(&map[ny as usize][nx as usize], Tile::Forest) {
                continue;
            }
            max_dist = max_dist.max(1 + dfs((ny, nx), map, end, seen.clone()))
        }
        println!("{:?}, {:?}, {}, {}", loc, end, seen.len(), max_dist);
        max_dist
    }
    let handler = builder
        .spawn(move || {
            // stack-intensive operations
            let mut sum = 0;
            sum = dfs(starting, &map, ending, HashSet::new());

            println!("sum: {}", sum);
        })
        .unwrap();

    handler.join().unwrap();
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
