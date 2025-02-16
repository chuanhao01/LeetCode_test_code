// start 1930
// end 2020
use std::{
    arch::x86_64::_CMP_TRUE_UQ,
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;
use queue::*;

// NOTES:
fn main() -> Result<()> {
    // let mut file_input = File::open("inputs/temp")?;
    let mut file_input = File::open("inputs/d12")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut possible_c: HashSet<char> = HashSet::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        let l: Vec<char> = l.chars().collect();
        map.push(l.clone());
        for c in l {
            possible_c.insert(c);
        }
    }
    println!("{:?}", map);
    println!("{:?}", possible_c);

    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    // Find a value to start on not in seen
    while seen.len() < map.len() * map[0].len() {
        let mut y = 0i64;
        let mut x = 0i64;
        for yy in 0..map.len() {
            for xx in 0..map[0].len() {
                if !seen.contains(&(yy as i64, xx as i64)) {
                    y = yy as i64;
                    x = xx as i64;
                    break;
                }
            }
        }
        // Find all connected
        let c = map[y as usize][x as usize];
        let mut q: Queue<(i64, i64)> = Queue::from(vec![(y, x)]);
        let mut tiles: HashSet<(i64, i64)> = HashSet::new();
        while !q.is_empty() {
            // println!("{}, {}", c, q.len());
            let current = q.dequeue().unwrap();
            if map[current.0 as usize][current.1 as usize] != c || seen.contains(&current) {
                // Skip since not the same
                continue;
            }
            // if the same
            // Add it to seen
            seen.insert((current.0, current.1));
            tiles.insert((current.0, current.1));

            for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let ny = current.0 + d.0;
                let nx = current.1 + d.1;
                if ny < 0 || ny >= map.len() as i64 || nx < 0 || nx >= map[0].len() as i64 {
                    // Skip cause oor
                    continue;
                }

                if !seen.contains(&(ny, nx)) && map[ny as usize][nx as usize] == c {
                    q.queue((ny, nx)).unwrap();
                }
            }
        }
        // println!("{}, {:?}", c, tiles);
        let area = tiles.len();
        let mut sides = 0;
        let mut interior = 0;
        let mut exterior = 0;
        for tile in &tiles {
            // check exterior
            // What the only check 2 sides
            for ds in [
                // [(-1, 0), (0, 1), (-1, 1)],
                // [(-1, 0), (0, -1), (-1, -1)],
                // [(0, -1), (1, 0), (1, -1)],
                // [(1, 0), (0, 1), (1, 1)],
                [(-1, 0), (0, 1)],
                [(-1, 0), (0, -1)],
                [(0, -1), (1, 0)],
                [(1, 0), (0, 1)],
            ] {
                exterior += if ds.iter().all(|d| {
                    let ny = tile.0 + d.0;
                    let nx = tile.1 + d.1;
                    if ny < 0 || ny >= map.len() as i64 || nx < 0 || nx >= map[0].len() as i64 {
                        // Skip cause oor
                        true
                    } else {
                        if map[ny as usize][nx as usize] == c {
                            false
                        } else {
                            true
                        }
                    }
                }) {
                    1
                } else {
                    0
                };
            }
            // interior
            for ds in [
                ((-1, 0), (0, 1), (-1, 1)),
                ((-1, 0), (0, -1), (-1, -1)),
                ((0, -1), (1, 0), (1, -1)),
                ((1, 0), (0, 1), (1, 1)),
            ] {
                interior += if [ds.0, ds.1].iter().all(|d| {
                    let ny = tile.0 + d.0;
                    let nx = tile.1 + d.1;
                    if ny < 0 || ny >= map.len() as i64 || nx < 0 || nx >= map[0].len() as i64 {
                        // Skip cause oor
                        false
                    } else {
                        if map[ny as usize][nx as usize] == c {
                            true
                        } else {
                            false
                        }
                    }
                }) && [ds.2].iter().all(|d| {
                    let ny = tile.0 + d.0;
                    let nx = tile.1 + d.1;
                    if ny < 0 || ny >= map.len() as i64 || nx < 0 || nx >= map[0].len() as i64 {
                        // Skip cause oor
                        true
                    } else {
                        if map[ny as usize][nx as usize] == c {
                            false
                        } else {
                            true
                        }
                    }
                }) {
                    1
                } else {
                    0
                };
            }
        }
        println!(
            "{}, {}, {}, {}",
            c,
            exterior,
            interior,
            area * (interior + exterior)
        );
        sum += area * (interior + exterior);
    }

    println!("sum: {}", sum);
    Ok(())
}

// for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
//     let ny = tile.0 + d.0;
//     let nx = tile.1 + d.1;
//     if ny <0 || ny >= map.len() as i64 || nx < 0 || nx > map[0].len() as i64{
//         // Skip cause oor
//         continue;
//     }
// }
