// start 2145
// end 2230
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
            map.push(l.chars().collect());
        } else {
            movements.append(&mut l.chars().collect());
        }
    }

    // println!("{:?}", map);
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
    for command in movements {
        let movement = direction_map.get(&command).unwrap();
        let new_robot_location = (robot_location.0 + movement.0, robot_location.1 + movement.1);
        if map[new_robot_location.0 as usize][new_robot_location.1 as usize] == 'O' {
            let mut new_boxes: Vec<((i64, i64), (i64, i64))> = Vec::new();
            let mut to_check = new_robot_location.clone();
            let mut wall = false;
            loop {
                let new_box_location = (to_check.0 + movement.0, to_check.1 + movement.1);
                if map[new_box_location.0 as usize][new_box_location.1 as usize] == '#' {
                    // Hit the wall
                    // dont move
                    wall = true;
                    break;
                }
                if map[new_box_location.0 as usize][new_box_location.1 as usize] == 'O'
                    || map[new_box_location.0 as usize][new_box_location.1 as usize] == '.'
                {
                    // another box
                    // println!("{:?}, {:?}", to_check, new_box_location);
                    new_boxes.push((to_check.clone(), new_box_location.clone()));
                    to_check = new_box_location.clone();
                }
                if map[new_box_location.0 as usize][new_box_location.1 as usize] == '.' {
                    // Hit the wall
                    // dont move
                    break;
                }
            }
            if wall {
                continue;
            }

            let mut replace = true;
            for (old_location, new_box_location) in new_boxes {
                map[new_box_location.0 as usize][new_box_location.1 as usize] = 'O';
                if replace {
                    map[old_location.0 as usize][old_location.1 as usize] = '.';
                    replace = false;
                }
            }
            robot_location = new_robot_location;
        } else if map[new_robot_location.0 as usize][new_robot_location.1 as usize] == '#' {
            // Hit the wall
            continue;
        } else if map[new_robot_location.0 as usize][new_robot_location.1 as usize] == '.' {
            robot_location = new_robot_location;
        } else {
            println!("Somehow i did not think of this")
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
            if map[y][x] == 'O' {
                sum += y * 100 + x;
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
