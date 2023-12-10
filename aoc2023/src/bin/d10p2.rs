use core::panic;
use queue::*;
use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Result},
    ops::{Add, IndexMut},
};

use itertools::izip;

/// Top, Bottom, Left Right
#[derive(Debug, Clone)]
enum Pipe {
    TopBottom,
    LeftRight,
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
    Ground,
    Starting,
}
impl Pipe {
    fn from_raw_pipe(raw_pipe: &str) -> Self {
        match raw_pipe {
            "|" => Self::TopBottom,
            "-" => Self::LeftRight,
            "L" => Self::TopRight,
            "J" => Self::TopLeft,
            "7" => Self::BottomLeft,
            "F" => Self::BottomRight,
            "." => Self::Ground,
            "S" => Self::Starting,
            _ => panic!("Unexpected raw_pipe: {}", raw_pipe),
        }
    }
    /// Given your current position, which direction u are going to check and the corrospoding pipe
    fn get_next_direction(
        relative_to_current_direction: &Direction,
        location_pipe: Self,
    ) -> Option<Direction> {
        match relative_to_current_direction {
            Direction::Top => match location_pipe {
                Self::TopBottom => Some(Direction::Top),
                Self::BottomLeft => Some(Direction::Left),
                Self::BottomRight => Some(Direction::Right),
                _ => None,
            },
            Direction::Bottom => match location_pipe {
                Self::TopBottom => Some(Direction::Bottom),
                Self::TopLeft => Some(Direction::Left),
                Self::TopRight => Some(Direction::Right),
                _ => None,
            },
            Direction::Left => match location_pipe {
                Self::LeftRight => Some(Direction::Left),
                Self::TopRight => Some(Direction::Top),
                Self::BottomRight => Some(Direction::Bottom),
                _ => None,
            },
            Direction::Right => match location_pipe {
                Self::LeftRight => Some(Direction::Right),
                Self::TopLeft => Some(Direction::Top),
                Self::BottomLeft => Some(Direction::Bottom),
                _ => None,
            },
            _ => panic!(
                "Tried to get next from None: {:?}",
                relative_to_current_direction
            ),
        }
    }
    // Given we are shotting a beam left to right, between the middle and bottom -0.75
    fn check_hit(pipe: Self) -> bool {
        matches!(pipe, Self::TopBottom | Self::BottomLeft | Self::BottomRight)
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    None,
}
impl Direction {
    fn iter() -> [Direction; 4] {
        [Self::Top, Self::Bottom, Self::Left, Self::Right]
    }
    fn to_index_pos(direction: &Self) -> (i64, i64) {
        match direction {
            Self::Top => (-1, 0),
            Self::Bottom => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
            _ => panic!("Wrong direction: {:?}", direction),
        }
    }
}
fn add_pos(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}
fn check_within_max_y_x(max_y: i64, max_x: i64, pos: (i64, i64)) -> bool {
    let (y, x) = pos;
    !(y < 0 || y >= max_y || x < 0 || x >= max_x)
}

fn largest_distance_from_s(map: Vec<Vec<Pipe>>) {
    let mut map = map;
    let max_y = map.len() as i64;
    let max_x = map[0].len() as i64;
    let mut s = (0, 0);
    for (y, row) in map.clone().iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if let Pipe::Starting = pipe {
                s = (y as i64, x as i64);
                break;
            }
        }
    }
    println!("{:?}", s);

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    // (index, distance_so_far)
    let mut queue = Vec::from([(s, Direction::iter().to_vec(), 0)]);
    let mut max_so_far = 0;
    while !queue.is_empty() {
        // println!("Queue: {:?}", queue);
        // println!("Visited: {:?}", visited);
        let (current_pos, possible_directions, acc) = queue.remove(0);
        max_so_far = max_so_far.max(acc);
        if visited.contains(&current_pos) {
            continue;
        }

        for direction in possible_directions {
            let possible_new_pos = add_pos(current_pos, Direction::to_index_pos(&direction));
            if !check_within_max_y_x(max_y, max_x, possible_new_pos) {
                continue;
            }
            let new_pos_pipe =
                &map.clone()[possible_new_pos.0 as usize][possible_new_pos.1 as usize];
            // println!("{:?}, {:?}", direction, new_pos_pipe);
            match Pipe::get_next_direction(&direction, new_pos_pipe.clone()) {
                Some(next_direction) => {
                    if visited.contains(&possible_new_pos) {
                        continue;
                    } else {
                        queue.push((possible_new_pos, vec![next_direction], acc + 1));
                    }
                }
                None => continue,
            }
        }
        visited.insert(current_pos);
    }
    println!("{}", max_so_far);
    let mut directions_s_can_go: (Direction, Direction) = (Direction::None, Direction::None);
    for direction in Direction::iter().clone() {
        let possible_new_pos = add_pos(s, Direction::to_index_pos(&direction));
        if !check_within_max_y_x(max_y, max_x, possible_new_pos) {
            continue;
        }
        let new_pos_pipe = &map.clone()[possible_new_pos.0 as usize][possible_new_pos.1 as usize];
        // println!("{:?}, {:?}", direction, new_pos_pipe);
        match Pipe::get_next_direction(&direction, new_pos_pipe.clone()) {
            // Some(_) => directions_s_can_go.push(direction),
            Some(_) => {
                if matches!(directions_s_can_go.0, Direction::None) {
                    directions_s_can_go.0 = direction.clone();
                } else {
                    directions_s_can_go.1 = direction.clone();
                }
            }
            None => continue,
        }
    }
    map[s.0 as usize][s.1 as usize] = match directions_s_can_go {
        (Direction::Top, Direction::Bottom) | (Direction::Bottom, Direction::Top) => {
            Pipe::TopBottom
        }
        (Direction::Right, Direction::Bottom) | (Direction::Bottom, Direction::Right) => {
            Pipe::BottomRight
        }
        (Direction::Left, Direction::Bottom) | (Direction::Bottom, Direction::Left) => {
            Pipe::BottomLeft
        }
        (Direction::Top, Direction::Left) | (Direction::Left, Direction::Top) => Pipe::TopLeft,
        (Direction::Top, Direction::Right) | (Direction::Right, Direction::Top) => Pipe::TopRight,
        _ => panic!("Something Wrong"),
    };
    println!("s_can_go: {:?}", directions_s_can_go);
    println!(
        "s_can_go: {:?}",
        match directions_s_can_go {
            (Direction::Top, Direction::Bottom) | (Direction::Bottom, Direction::Top) =>
                Pipe::TopBottom,
            (Direction::Right, Direction::Bottom) | (Direction::Bottom, Direction::Right) =>
                Pipe::BottomRight,
            (Direction::Left, Direction::Bottom) | (Direction::Bottom, Direction::Left) =>
                Pipe::BottomLeft,
            (Direction::Top, Direction::Left) | (Direction::Left, Direction::Top) => Pipe::TopLeft,
            (Direction::Top, Direction::Right) | (Direction::Right, Direction::Top) =>
                Pipe::TopRight,
            _ => panic!("Something Wrong"),
        }
    );

    let mut inside = 0;
    for (y, row) in map.clone().iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if !visited.contains(&(y as i64, x as i64)) {
                let y = y as i64;
                let mut intersection_count = 0;
                for mx in 0..(x as i64) {
                    let new_pos = (y, mx);
                    let new_pipe = &map[y as usize][mx as usize];
                    if visited.contains(&new_pos) && Pipe::check_hit(new_pipe.clone()) {
                        intersection_count += 1;
                    }
                }
                if intersection_count % 2 == 1 {
                    inside += 1;
                }
            }
        }
    }
    println!("inside: {}", inside);
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d10")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let map = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| Pipe::from_raw_pipe(&c.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:#?}", map);
    largest_distance_from_s(map);
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
