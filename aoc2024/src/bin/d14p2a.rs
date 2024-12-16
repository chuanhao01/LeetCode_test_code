// start 2205
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
    let mut file_input = File::open("inputs/d14")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let seconds = 100;
    let mx = 101;
    let my = 103;
    let mut robots: Vec<(Vec<i64>, Vec<i64>)> = Vec::new();
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
        robots.push((p, v));
        // println!("{:?}, {:?}", p, v);
    }
    // println!("{:?}", quads);
    // sum = quads.0 * quads.1 * quads.2 * quads.3;

    // println!("{:?}", robots);
    for i in 0..100000 {
        let mut pos: Vec<Vec<i64>> = Vec::new();
        for ii in 0..robots.len() {
            let ff = &robots[ii];

            let final_x = ff.0[0] + ff.1[0] * i;
            let final_x = final_x % mx;
            let final_x = if final_x < 0 { final_x + mx } else { final_x };

            let final_y = ff.0[1] + ff.1[1] * i;
            let final_y = final_y % my;
            let final_y = if final_y < 0 { final_y + my } else { final_y };
            robots[ii].0 = vec![final_x, final_y];
            pos.push(vec![final_x, final_y]);
        }
        for rr in &robots {
            let r = &rr.0;
            for dd in [
                vec![(-1, 0), (-1, -1), (-1, 1)],
                vec![(1, 0), (1, -1), (1, 1)],
                vec![(0, 1), (-1, 1), (1, 1)],
                vec![(0, -1), (-1, -1), (1, -1)],
            ] {
                let mut np: Vec<Vec<i64>> = Vec::new();
                for d in dd {
                    np.push(vec![r[0] + d.1, r[1] + d.0]);
                }
                if np.iter().all(|cords| pos.contains(cords)) {
                    let mut map: Vec<Vec<char>> = vec![vec!['.'; mx as usize]; my as usize];
                    for r in &robots {
                        // println!("{:?}", r);
                        map[r.0[1] as usize][r.0[0] as usize] = 'x';
                    }
                    println!(
                        "{}",
                        map.iter()
                            .map(|l| l.iter().map(|c| c.to_string()).join(""))
                            .join("\n")
                    );
                    println!("{}", i);
                    println!("");
                }
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
