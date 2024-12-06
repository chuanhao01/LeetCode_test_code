// start 2345
// end

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

// NOTES:
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d6")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut direction = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();

    let mut sum = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        let line = l.chars().collect();
        map.push(line);
    }
    println!("{:?}", map);

    let mut x = 0;
    let mut y = 0;
    let mut done = false;
    for (j, row) in map.clone().into_iter().enumerate() {
        for (i, c) in row.into_iter().enumerate() {
            if c == '^' {
                y = j as i32;
                x = i as i32;
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }
    let mut seen: Vec<((i32, i32), (i32, i32))> = Vec::new();
    let mut done = false;
    let mut cur = direction.next().unwrap();
    loop {
        seen.push(((y, x), cur.clone()));
        let mut ny = y + cur.0;
        let mut nx = x + cur.1;
        if ny < 0 || ny >= map.len() as i32 {
            break;
        }
        if nx < 0 || nx >= map[0].len() as i32 {
            break;
        }

        if map[ny as usize][nx as usize] == '#' {
            cur = direction.next().unwrap();
            ny = y + cur.0;
            nx = x + cur.1;
            if ny < 0 || ny >= map.len() as i32 {
                break;
            }
            if nx < 0 || nx >= map[0].len() as i32 {
                break;
            }
        }

        y = ny;
        x = nx;
    }
    // println!("{:?}", seen);
    let mut unique_locations: HashSet<(i32, i32)> = HashSet::new();
    for (l, d) in seen {
        unique_locations.insert(l);
    }
    sum = unique_locations.len();

    println!("sum: {}", sum);
    Ok(())
}
