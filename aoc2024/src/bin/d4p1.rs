// start 2125
// done 2215
use std::{
    fs::File,
    io::{Read, Result},
};

// NOTES:
// XM
// MAS

// Don't think you will ever double count with DFS

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
            sum += dfs(&map, (y, x), None, 'X');
        }
    }

    println!("sum: {}", sum);
    Ok(())
}

// Ok so the search only allows for up, down, left, right, diagonal and backwards
// Pattern is that you must follow the path u take
fn dfs(
    map: &Vec<Vec<char>>,
    yx: (usize, usize),
    dydx: Option<(usize, usize)>,
    search_char: char,
) -> i32 {
    // y, x is current pos to check
    let y = yx.0;
    let x = yx.1;

    // exit conditions
    if map[y][x] != search_char {
        return 0;
    }
    if search_char == 'S' && map[y][x] == search_char {
        return 1;
    }
    let next_char = match search_char {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => panic!("oh no"),
    };
    // Iterate through all possible
    let mut s = 0;
    match dydx {
        Some((dy, dx)) => {
            // Continue searching in the same direction
            let ny = y as i32 + dy as i32;
            let nx = x as i32 + dx as i32;
            if ny < 0 || ny >= map.len() as i32 {
                return 0;
            }
            if nx < 0 || nx >= map[0].len() as i32 {
                return 0;
            }
            dfs(map, (ny as usize, nx as usize), dydx, next_char)
        }
        None => {
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if (dy, dx) == (0, 0) {
                        // Skip this one
                        continue;
                    }
                    if y as i32 + dy < 0 || y as i32 + dy >= map.len() as i32 {
                        continue;
                    }
                    if x as i32 + dx < 0 || x as i32 + dx >= map[0].len() as i32 {
                        continue;
                    }
                    s += dfs(
                        map,
                        ((y as i32 + dy) as usize, (x as i32 + dx) as usize),
                        Some((dy as usize, dx as usize)),
                        next_char,
                    );
                }
            }
            s
        }
    }
}
