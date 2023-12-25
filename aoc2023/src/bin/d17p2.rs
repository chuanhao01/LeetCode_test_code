/// Thoughts
/// Still need to brush up on algos and other data structs
/// Priority Queue in rust has more specific implementation that you need to take note of
/// Also dijkstra's algorithm works here cause you are treating the map as a graph
/// Since we only care about reaching the goal in the shortest path
/// Needed help on this qns
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    f64::INFINITY,
    fs::File,
    io::{Read, Result},
    mem::Discriminant,
    slice::Split,
};

use itertools::izip;
use priority_queue::{DoublePriorityQueue, PriorityQueue};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

fn get_lowest(map: Vec<Vec<i64>>) {
    let max_y = map.len();
    let max_x = map[0].len();
    let mut seen: HashSet<(i64, i64, i64, i64, i64)> = HashSet::new();
    let mut q: BinaryHeap<(Reverse<i64>, i64, i64, i64, i64, i64)> = BinaryHeap::new();
    q.push((Reverse(0), 0, 0, 0, 0, 0));
    while let Some(item) = q.pop() {
        // println!("{:?}", q);
        let (Reverse(heat), y, x, dy, dx, n) = item;
        if y == max_y as i64 - 1 && x == max_x as i64 - 1 && 3 < n {
            println!("heat: {}", heat);
            break;
        }
        if seen.contains(&(y, x, dy, dx, n)) {
            continue;
        }
        seen.insert((y, x, dy, dx, n));
        if (dy, dx) != (0, 0) && n < 10 {
            let (ny, nx) = (y + dy, x + dx);
            if 0 <= ny && ny < max_y as i64 && 0 <= nx && nx < max_x as i64 {
                q.push((
                    Reverse(heat + map[ny as usize][nx as usize]),
                    ny,
                    nx,
                    dy,
                    dx,
                    n + 1,
                ));
            }
        }
        if (dy, dx) == (0, 0) || (3 < n && n < 11) {
            for possible_direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if possible_direction != (dy, dx) && possible_direction != (-dy, -dx) {
                    let (ny, nx) = (y + possible_direction.0, x + possible_direction.1);
                    if 0 <= ny && ny < max_y as i64 && 0 <= nx && nx < max_x as i64 {
                        q.push((
                            Reverse(heat + map[ny as usize][nx as usize]),
                            ny,
                            nx,
                            possible_direction.0,
                            possible_direction.1,
                            1,
                        ));
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d17")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;
    let map = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
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
