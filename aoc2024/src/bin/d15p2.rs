// start 2230
// end
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
    vec,
};

use itertools::Itertools;
use queue::*;

// NOTES:
fn main() -> Result<()> {
    // let mut file_input = File::open("inputs/temp")?;
    let mut file_input = File::open("inputs/d15")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let direction_map =
        HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);
    let next_box_map = HashMap::from([('[', (0, 1)), (']', (0, -1))]);

    let mut sum = 0;
    let mut empty_count = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty() {
            empty_count += 1;
            continue;
        }
        if empty_count == 0 {
            let mut newl: Vec<char> = Vec::new();
            for c in l.chars() {
                if c == '#' {
                    newl.push('#');
                    newl.push('#');
                } else if c == '.' {
                    newl.push('.');
                    newl.push('.');
                } else if c == '@' {
                    newl.push('@');
                    newl.push('.');
                } else if c == 'O' {
                    newl.push('[');
                    newl.push(']');
                }
            }
            map.push(newl);
        } else {
            movements.append(&mut l.chars().collect());
        }
    }

    println!("{:?}", map);
    // println!("{:?}", movements);

    let mut robot_location: (i64, i64) = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                robot_location = (y as i64, x as i64);
                map[y][x] = '.';
                break;
            }
        }
    }
    println!(
        "{}",
        map.iter()
            .map(|vc| vc.iter().map(|c| c.to_string()).join(""))
            .join("\n")
    );
    println!();

    for instruction in movements {
        let direction = direction_map[&instruction];
        let new_robot_pos = (
            robot_location.0 + direction.0,
            robot_location.1 + direction.1,
        );
        let c = map[new_robot_pos.0 as usize][new_robot_pos.1 as usize];
        if c == '.' {
            robot_location = new_robot_pos;
        } else if c == '#' {
            continue;
        } else if c == '[' || c == ']' {
            let mut to_check: Vec<(i64, i64)> = Vec::from([new_robot_pos]);
            let mut boxes: Vec<(i64, i64)> = Vec::new();
            let mut wall = false;
            while !to_check.is_empty() {
                let check = to_check.remove(0);
                if boxes.contains(&check) {
                    // Skip
                    continue;
                }
                let cc = map[check.0 as usize][check.1 as usize];
                if cc == '[' || cc == ']' {
                    boxes.push(check.clone());
                    let bbox_d = next_box_map[&cc];
                    boxes.push((check.0 + bbox_d.0, check.1 + bbox_d.1));
                    to_check.push((check.0 + direction.0, check.1 + direction.1));
                    to_check.push((
                        check.0 + bbox_d.0 + direction.0,
                        check.1 + bbox_d.1 + direction.1,
                    ));
                } else if cc == '#' {
                    wall = true;
                    break;
                } else if cc == '.' {
                    continue;
                }
            }
            if wall {
                continue;
            }

            let mut new_boxes: Vec<(i64, i64, char)> = Vec::new();
            for bbox in boxes {
                let c = map[bbox.0 as usize][bbox.1 as usize];
                new_boxes.push((bbox.0 + direction.0, bbox.1 + direction.1, c));
                map[bbox.0 as usize][bbox.1 as usize] = '.';
            }
            for bbox in new_boxes {
                map[bbox.0 as usize][bbox.1 as usize] = bbox.2;
            }
            robot_location = new_robot_pos;
        }
    }

    println!(
        "{}",
        map.iter()
            .map(|vc| vc.iter().map(|c| c.to_string()).join(""))
            .join("\n")
    );

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '[' {
                sum += y * 100 + x;
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
