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
        let mut prize_s = prize_s
            .iter()
            .map(|s| {
                let s = s.chars().collect::<Vec<char>>();
                let s = &s[2..s.len()].iter().map(|c| c.to_string()).join("");
                s.to_owned().parse().unwrap()
            })
            .collect::<Vec<i64>>();
        prize_s[0] += 10000000000000;
        prize_s[1] += 10000000000000;
        println!("{:?}", prize_s);
        let af = (prize_s[0] as f64 * b_s[1] as f64 - prize_s[1] as f64 * b_s[0] as f64)
            / (a_s[0] as f64 * b_s[1] as f64 - b_s[0] as f64 * a_s[1] as f64);
        let bf = (prize_s[0] as f64 * a_s[1] as f64 - prize_s[1] as f64 * a_s[0] as f64)
            / (a_s[1] as f64 * b_s[0] as f64 - a_s[0] as f64 * b_s[1] as f64);
        println!("{}, {}", af, bf);
        if af == af.round() && bf == bf.round() {
            sum += af.round() as i64 * 3 + bf.round() as i64;
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
