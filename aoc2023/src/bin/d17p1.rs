/// Thoughts
/// Still need to brush up on algos and other data structs
/// Priority Queue in rust has more specific implementation that you need to take note of
/// Also dijkstra's algorithm works here cause you are treating the map as a graph
/// Since we only care about reaching the goal in the shortest path
/// Needed help on this qns
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    f64::INFINITY,
    fs::File,
    io::{Read, Result},
    mem::Discriminant,
    slice::Split,
};

use itertools::izip;
use priority_queue::{DoublePriorityQueue, PriorityQueue};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
impl Direction {
    fn is_opposite(&self, other: &Self) -> bool {
        match self {
            Self::Up => {
                matches!(other, &Self::Down)
            }
            Self::Down => {
                matches!(other, &Self::Up)
            }
            Self::Left => {
                matches!(other, &Self::Right)
            }
            Self::Right => {
                matches!(other, &Self::Left)
            }
            // Always false, since you cannot be opposite of Direction::None
            Self::None => false,
        }
    }
    fn get_new_index(&self) -> (i64, i64) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
            Self::None => panic!("Should never call get_new_index with None direction"),
        }
    }
}

fn get_lowest(map: Vec<Vec<u32>>) {
    let max_y = map.len();
    let max_x = map[0].len();
    let mut seen: HashSet<((i64, i64), Direction, i64)> = HashSet::new();
    let mut q: DoublePriorityQueue<((i64, i64), Direction, i64), i64> = DoublePriorityQueue::new();
    q.push(((0, 0), Direction::None, 0), 0);
    while !q.is_empty() {
        // println!("{:?}", q);
        // if q.len() > 10 {
        //     break;
        // }
        let (((y, x), direction, straight), heat) = q.pop_min().unwrap();
        if y as usize == max_y - 1 && x as usize == max_x - 1 && straight < 3 {
            // println!("seen: {:?}", seen);
            println!("heat: {}", heat);
            break;
        }
        if seen.contains(&((y, x), direction.clone(), straight)) {
            continue;
        }
        seen.insert(((y, x), direction.clone(), straight));
        // Handling case where you can continue in the same direction
        for possible_direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let (cy, cx) = possible_direction.get_new_index();
            let (ny, nx) = (y + cy, x + cx);
            if ny < 0
                || ny as usize >= max_y
                || nx < 0
                || nx as usize >= max_x
                || direction.is_opposite(&possible_direction)
            {
                // Skip if OOB or opposite direction, cannot go backwards
                continue;
            }
            // println!("({}, {}), direction: {:?}", ny, nx, possible_direction);
            if possible_direction == direction {
                // If you are going straight, try and continue going straight
                if straight < 3 {
                    // Was stuck here, because I assumed the push would override/add a new entry into the queue
                    q.push_decrease(
                        ((ny, nx), possible_direction.clone(), straight + 1),
                        heat + map[ny as usize][nx as usize] as i64,
                    );
                }
            } else {
                // Other Direction
                q.push_decrease(
                    ((ny, nx), possible_direction.clone(), 1),
                    heat + map[ny as usize][nx as usize] as i64,
                );
            }
        }
    }
}

fn main() -> Result<()> {
    // let mut file_input = File::open("inputs/d17")?;
    let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;
    let map = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    get_lowest(map);

    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_direction_get() {
        let d = Direction::Up;
        assert!(d.is_opposite(&Direction::Down));
        assert!(!d.is_opposite(&Direction::Left));
        assert!(Direction::Down.is_opposite(&Direction::Up));
        assert!(!Direction::Down.is_opposite(&Direction::Left));
        assert!(Direction::Left.is_opposite(&Direction::Right));
        assert!(!Direction::Left.is_opposite(&Direction::Up));
        assert!(Direction::Right.is_opposite(&Direction::Left));
        assert!(!Direction::Right.is_opposite(&Direction::Up));
    }
}
