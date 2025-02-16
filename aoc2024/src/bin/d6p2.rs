// start 0000
// end

// Eh, silly bug with skipping steps before
// Fixed it, but can't be bothered to optimize further

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
    // println!("{:?}", map);

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
            ny = y;
            nx = x;
        }

        y = ny;
        x = nx;
    }
    let mut unique_wall_pos: HashSet<(i32, i32)> = HashSet::new();
    let mut possible_wall_pos: HashSet<(i32, i32)> = HashSet::new();
    let mut cc = 0;
    for (l, d) in seen {
        let ny = l.0 + d.0;
        let nx = l.1 + d.1;
        // Skip if wall OOR
        if ny < 0 || ny >= map.len() as i32 {
            continue;
        }
        if nx < 0 || nx >= map[0].len() as i32 {
            continue;
        }
        if unique_wall_pos.contains(&(ny, nx)) {
            continue;
        }
        let mut nmap = map.clone();
        nmap[ny as usize][nx as usize] = '#';
        // if nmap[ny as usize][nx as usize] != '#' && cc < 10 {
        //     cc += 1;
        //     nmap[ny as usize][nx as usize] = '#';
        //     println!(
        //         "{}",
        //         nmap.iter()
        //             .map(|l| l
        //                 .iter()
        //                 .map(|c| c.to_string())
        //                 .collect::<Vec<String>>()
        //                 .join(""))
        //             .collect::<Vec<String>>()
        //             .join("\n")
        //     );
        //     println!()
        // }
        if check_loop(&nmap) {
            unique_wall_pos.insert((ny, nx));
        };
    }
    sum = unique_wall_pos.len();
    // println!("{:?}", possible_wall_pos.len());

    println!("sum: {}", sum);
    Ok(())
}

fn check_loop(map: &Vec<Vec<char>>) -> bool {
    let mut direction = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();

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
    let mut cur = direction.next().unwrap();
    loop {
        if seen.contains(&((y, x), cur.clone())) {
            return true;
        }
        seen.push(((y, x), cur.clone()));
        let mut ny = y + cur.0;
        let mut nx = x + cur.1;
        if ny < 0 || ny >= map.len() as i32 {
            return false;
        }
        if nx < 0 || nx >= map[0].len() as i32 {
            return false;
        }

        if map[ny as usize][nx as usize] == '#' {
            cur = direction.next().unwrap();
            ny = y;
            nx = x;
        }

        y = ny;
        x = nx;
    }
}
