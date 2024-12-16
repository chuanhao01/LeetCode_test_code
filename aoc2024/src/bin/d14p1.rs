// start 2145
// end 2205
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
    let mut file_input = File::open("inputs/d14")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let seconds = 100;
    let mx = 101;
    let my = 103;
    let mut quads = (0, 0, 0, 0);
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        let mut l = l.split(" ");
        let p = l.next().unwrap();
        let p = p.split("=").collect::<Vec<&str>>()[1];
        let p = p
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i64>>();

        let v = l.next().unwrap();
        let v = v.split("=").collect::<Vec<&str>>()[1];
        let v = v
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i64>>();
        // println!("{:?}, {:?}", p, v);

        let final_x = p[0] + v[0] * seconds;
        let final_x = final_x % mx;
        let final_x = if final_x < 0 { final_x + mx } else { final_x };

        let final_y = p[1] + v[1] * seconds;
        let final_y = final_y % my;
        let final_y = if final_y < 0 { final_y + my } else { final_y };
        println!("{}, {}", final_x, final_y);
        if final_x < mx / 2 && final_y < my / 2 {
            quads.0 += 1;
        } else if final_x > mx / 2 && final_y < my / 2 {
            quads.1 += 1;
        } else if final_x < mx / 2 && final_y > my / 2 {
            quads.2 += 1;
        } else if final_x > mx / 2 && final_y > my / 2 {
            quads.3 += 1;
        }
    }
    println!("{:?}", quads);
    sum = quads.0 * quads.1 * quads.2 * quads.3;

    println!("sum: {}", sum);
    Ok(())
}
