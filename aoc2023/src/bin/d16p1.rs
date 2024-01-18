use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read, Result},
    slice::Split,
};

use itertools::izip;

/// Helpers for Objects
enum Mirror {
    Left,
    Right,
}
impl Mirror {
    fn reflect_direction(&self, direction: Direction) -> Direction {
        match self {
            Self::Left => match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
            },
            Self::Right => match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
            },
        }
    }
}

/// Helpers for Objects
enum Splitter {
    Horizontal,
    Vertrical,
}
impl Splitter {
    fn split_direction(&self, direction: Direction) -> Option<(Direction, Direction)> {
        match self {
            Self::Horizontal => match direction {
                Direction::Left | Direction::Right => None,
                Direction::Up | Direction::Down => Some((Direction::Left, Direction::Right)),
            },
            Self::Vertrical => match direction {
                Direction::Up | Direction::Down => None,
                Direction::Left | Direction::Right => Some((Direction::Up, Direction::Down)),
            },
        }
    }
}

enum Objects {
    Empty,
    Mirror(Mirror),
    Splitter(Splitter),
}
impl Objects {
    fn new(raw_object: &str) -> Self {
        match raw_object {
            "." => Self::Empty,
            "\\" => Self::Mirror(Mirror::Right),
            "/" => Self::Mirror(Mirror::Left),
            "-" => Self::Splitter(Splitter::Horizontal),
            "|" => Self::Splitter(Splitter::Vertrical),
            _ => panic!("Invalid raw_object: {}", raw_object),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn get_new_index(&self, index: (i64, i64)) -> (i64, i64) {
        match self {
            Self::Up => (index.0 - 1, index.1),
            Self::Down => (index.0 + 1, index.1),
            Self::Left => (index.0, index.1 - 1),
            Self::Right => (index.0, index.1 + 1),
        }
    }
}

struct Map {
    body: Vec<Vec<Objects>>,
}
impl Map {
    fn new(raw_map: &str) -> Self {
        let body = raw_map
            .to_string()
            .split('\n')
            .map(|row| {
                row.chars()
                    .map(|c| Objects::new(&c.to_string()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { body }
    }
    fn traverse_map(&mut self) -> usize {
        let mut taken: HashSet<(i64, i64)> = HashSet::new();
        let mut paths: HashSet<(Direction, (i64, i64))> = HashSet::new();
        let mut path_id = 0;
        let mut queue: Vec<(Direction, (i64, i64), i64)> =
            vec![(Direction::Right, (0, 0), path_id)];
        while !queue.is_empty() {
            let item = queue.remove(0);
            let (direction, map_location, current_path_id) = item;
            let max_y = self.body.len() as i64;
            let max_x = self.body[0].len() as i64;
            let (y, x) = map_location;
            if y < 0 || y >= max_y || x < 0 || x >= max_x {
                continue;
            }
            if paths.get(&(direction.clone(), map_location)).is_some() {
                continue;
            }
            taken.insert(map_location);
            paths.insert((direction.clone(), map_location));
            match &self.body[y as usize][x as usize] {
                Objects::Empty => queue.push((
                    direction.clone(),
                    direction.get_new_index(map_location),
                    current_path_id,
                )),
                Objects::Mirror(mirror) => {
                    let new_direction = mirror.reflect_direction(direction);
                    queue.push((
                        new_direction.clone(),
                        new_direction.get_new_index(map_location),
                        current_path_id,
                    ));
                }
                Objects::Splitter(splitter) => {
                    if let Some((dir1, dir2)) = splitter.split_direction(direction.clone()) {
                        queue.push((dir1.clone(), dir1.get_new_index(map_location), path_id + 1));
                        queue.push((dir2.clone(), dir2.get_new_index(map_location), path_id + 2));
                        path_id += 2;
                    } else {
                        queue.push((
                            direction.clone(),
                            direction.get_new_index(map_location),
                            current_path_id,
                        ))
                    }
                }
            }
        }
        taken.len()
    }
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d16")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;
    let mut map = Map::new(&input);
    sum = map.traverse_map();

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
