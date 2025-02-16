// start 2215
// done 2227

// Not proud but it works

use std::{
    fs::File,
    io::{Read, Result},
};

// NOTES:
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d4")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        let line = l.chars().collect();
        map.push(line);
    }
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            sum += if dfs(&map, (y, x), 'A') { 1 } else { 0 };
        }
    }

    println!("sum: {}", sum);
    Ok(())
}

// Ok so brute force lol
fn dfs(map: &Vec<Vec<char>>, yx: (usize, usize), search_char: char) -> bool {
    // y, x is current pos to check
    let y = yx.0;
    let x = yx.1;
    if map[y][x] != search_char {
        return false;
    }

    // Checking digonals
    for (dydx1, dydx2) in [((-1, -1), (1, 1)), ((1, -1), (-1, 1))] {
        let nyx1 = (y as i32 + dydx1.0, x as i32 + dydx1.1);
        let nyx2 = (y as i32 + dydx2.0, x as i32 + dydx2.1);
        for (ny, nx) in [nyx1, nyx2] {
            if ny < 0 || ny >= map.len() as i32 {
                return false;
            }
            if nx < 0 || nx >= map[0].len() as i32 {
                return false;
            }
        }
        if map[nyx1.0 as usize][nyx1.1 as usize] == 'M' {
            if map[nyx2.0 as usize][nyx2.1 as usize] == 'S' {
            } else {
                return false;
            }
        } else if map[nyx1.0 as usize][nyx1.1 as usize] == 'S' {
            if map[nyx2.0 as usize][nyx2.1 as usize] == 'M' {
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
