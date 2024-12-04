// start 2125
// done
use regex::Regex;
use std::{
    fs::File,
    io::{Read, Result},
};

// NOTES:
// XM
// MAS

// Don't think you will ever double count with DFS

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/temp")?;
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
    println!("{:?}", map);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            sum += if dfs(&map, y, x, 'X') { 1 } else { 0 };
            println!("{}, {}: {}, {}", y, x, map[y][x], sum);
        }
    }

    println!("sum: {}", sum);
    Ok(())
}

fn dfs(map: &Vec<Vec<char>>, y: usize, x: usize, search_char: char) -> bool {
    // y, x is current pos to check

    // exit conditions
    if map[y][x] != search_char {
        return false;
    }
    if search_char == 'S' && map[y][x] == search_char {
        return true;
    }
    let next_char = match search_char {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => panic!("oh no"),
    };
    // Iterate through all possible
    let mut s = false;
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
            s = s
                || dfs(
                    map,
                    (y as i32 + dy) as usize,
                    (x as i32 + dx) as usize,
                    next_char,
                );
        }
    }
    s
}
