// start 2205
// end
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
    vec,
};

use itertools::Itertools;
use queue::*;

// NOTES:
fn main() -> Result<()> {
    // let mut file_input = File::open("inputs/temp")?;
    let mut file_input = File::open("inputs/d14")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0f64;
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
    let mut sss: HashSet<i64> = HashSet::new();
    let mut min_so_far = 103i64.pow(4);
    for i in 0..10000 {
        let mut sum: i64 = 0;
        let mut quads = (0, 0, 0, 0);
        let mut new_robots: Vec<(i64, i64)> = Vec::new();
        for ii in 0..robots.len() {
            let ff = &robots[ii];

            let final_x = ff.0[0] + ff.1[0] * i;
            let final_x = final_x % mx;
            let final_x = if final_x < 0 { final_x + mx } else { final_x };

            let final_y = ff.0[1] + ff.1[1] * i;
            let final_y = final_y % my;
            let final_y = if final_y < 0 { final_y + my } else { final_y };
            new_robots.push((final_x, final_y));
        }
        let to_check: HashSet<(i64, i64)> = new_robots.clone().into_iter().collect();

        if to_check.len() == new_robots.len() {
            let mut map: Vec<Vec<char>> = vec![vec!['.'; mx as usize]; my as usize];
            for r in &new_robots {
                // println!("{:?}", r);
                map[r.1 as usize][r.0 as usize] = 'x';
            }
            println!(
                "{}",
                map.iter()
                    .map(|l| l.iter().map(|c| c.to_string()).join(""))
                    .join("\n")
            );
            println!("{}", i);
            println!("{:?}", quads);
            println!("");
        }
        // let mut checked: HashSet<(i64, i64)> = HashSet::new();
        // let mut links_seen: HashSet<i64> = HashSet::new();
        // while !&to_check.is_empty() {
        //     let ele = to_check.clone();
        //     let ele = ele.iter().next().unwrap();
        //     to_check.remove(ele);
        //     let mut search_space: Vec<(i64, i64)> = vec![ele.clone()];
        //     let mut links = 0;
        //     while !search_space.is_empty() {
        //         let ss = search_space.remove(0);
        //         links += 1;
        //         for d in [
        //             (-1, 0),
        //             (1, 0),
        //             (0, -1),
        //             (0, 1),
        //             (-1, -1),
        //             (-1, 1),
        //             (1, -1),
        //             (1, 1),
        //         ] {
        //             let nx = ss.0 + d.0;
        //             let ny = ss.1 + d.1;
        //             if to_check.contains(&(nx, ny)) {
        //                 to_check.remove(&(nx, ny));
        //                 search_space.push((ny, nx));
        //             }
        //         }
        //     }
        //     links_seen.insert(links);
        // }
        // let mut links_seen = links_seen.clone().into_iter().collect::<Vec<i64>>();
        // links_seen.sort();
        // let abc = links_seen[links_seen.len() - 1] + links_seen[links_seen.len() - 2];
        // sss.insert(abc);

        // if sum == 105151032 {
        //     let mut map: Vec<Vec<char>> = vec![vec!['.'; mx as usize]; my as usize];
        //     for r in &robots {
        //         // println!("{:?}", r);
        //         map[r.0[1] as usize][r.0[0] as usize] = 'x';
        //     }
        //     println!(
        //         "{}",
        //         map.iter()
        //             .map(|l| l.iter().map(|c| c.to_string()).join(""))
        //             .join("\n")
        //     );
        //     println!("{}", i);
        //     println!("{:?}", quads);
        //     println!("");
        // }
    }
    let mut sss = sss;
    // sss.sort();
    // println!("{:?}", sss[2]);

    for i in 0..10000 {
        let mut sum: i64 = 0;
        let mut quads = (0, 0, 0, 0);
        for ii in 0..robots.len() {
            let ff = &robots[ii];

            let final_x = ff.0[0] + ff.1[0] * i;
            let final_x = final_x % mx;
            let final_x = if final_x < 0 { final_x + mx } else { final_x };

            let final_y = ff.0[1] + ff.1[1] * i;
            let final_y = final_y % my;
            let final_y = if final_y < 0 { final_y + my } else { final_y };
            robots[ii].0 = vec![final_x, final_y];
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
        sum = quads.0 * quads.1 * quads.2 * quads.3;

        if sum == 105151032 {
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
            println!("{:?}", quads);
            println!("");
        }
    }

    // for i in 0..100000 {
    //     let mut sum: i64 = 0;
    //     let mut quads = (0, 0, 0, 0);
    //     for ii in 0..robots.len() {
    //         let ff = &robots[ii];

    //         let final_x = ff.0[0] + ff.1[0] * i;
    //         let final_x = final_x % mx;
    //         let final_x = if final_x < 0 { final_x + mx } else { final_x };

    //         let final_y = ff.0[1] + ff.1[1] * i;
    //         let final_y = final_y % my;
    //         let final_y = if final_y < 0 { final_y + my } else { final_y };
    //         if final_x < mx / 2 && final_y < my / 2 {
    //             quads.0 += 1;
    //         } else if final_x > mx / 2 && final_y < my / 2 {
    //             quads.1 += 1;
    //         } else if final_x < mx / 2 && final_y > my / 2 {
    //             quads.2 += 1;
    //         } else if final_x > mx / 2 && final_y > my / 2 {
    //             quads.3 += 1;
    //         }
    //     }
    //     sum = quads.0 * quads.1 * quads.2 * quads.3;
    //     if sum <= min_so_far * 2 {
    //         let mut map: Vec<Vec<char>> = vec![vec!['.'; mx as usize]; my as usize];
    //         for r in &robots {
    //             // println!("{:?}", r);
    //             map[r.0[1] as usize][r.0[0] as usize] = 'x';
    //         }
    //         println!(
    //             "{}",
    //             map.iter()
    //                 .map(|l| l.iter().map(|c| c.to_string()).join(""))
    //                 .join("\n")
    //         );
    //         println!("{}", i);
    //         println!("");
    //     }
    // }

    println!("sum: {}", sum);
    Ok(())
}
