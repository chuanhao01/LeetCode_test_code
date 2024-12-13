// start 1910
// end
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;
use queue::*;

// NOTES:
fn main() -> Result<()> {
    // let mut file_input = File::open("inputs/temp")?;
    let mut file_input = File::open("inputs/d13")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut possible_c: HashSet<char> = HashSet::new();
    let mut lines = input.split('\n').collect::<Vec<&str>>();

    for i in 0..lines.len() / 4 {
        let a_s = lines[i * 4];
        let a_s = a_s.split(": ").collect::<Vec<&str>>()[1];
        let a_s = a_s.split(", ").collect::<Vec<&str>>();
        let a_s = a_s
            .iter()
            .map(|s| {
                let s = s.chars().collect::<Vec<char>>();
                let s = &s[2..s.len()].iter().map(|c| c.to_string()).join("");
                s.to_owned().parse().unwrap()
            })
            .collect::<Vec<i64>>();
        // println!("{:?}", a_s);

        let b_s = lines[i * 4 + 1];
        let b_s = b_s.split(": ").collect::<Vec<&str>>()[1];
        let b_s = b_s.split(", ").collect::<Vec<&str>>();
        let b_s = b_s
            .iter()
            .map(|s| {
                let s = s.chars().collect::<Vec<char>>();
                let s = &s[2..s.len()].iter().map(|c| c.to_string()).join("");
                s.to_owned().parse().unwrap()
            })
            .collect::<Vec<i64>>();
        // println!("{:?}", b_s);

        let prize_s = lines[i * 4 + 2];
        let prize_s = prize_s.split(": ").collect::<Vec<&str>>()[1];
        let prize_s = prize_s.split(", ").collect::<Vec<&str>>();
        let prize_s = prize_s
            .iter()
            .map(|s| {
                let s = s.chars().collect::<Vec<char>>();
                let s = &s[2..s.len()].iter().map(|c| c.to_string()).join("");
                s.to_owned().parse().unwrap()
            })
            .collect::<Vec<i64>>();
        // println!("{:?}", prize_s);
        let mut smallest_token_so_far = 1000;
        for a in 0..100 {
            for b in 0..100 {
                if a * a_s[0] + b * b_s[0] == prize_s[0] && a * a_s[1] + b * b_s[1] == prize_s[1] {
                    smallest_token_so_far = smallest_token_so_far.min(a * 3 + b);
                }
            }
        }
        if smallest_token_so_far == 1000 {
            // Skip
        } else {
            sum += smallest_token_so_far;
        }
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
