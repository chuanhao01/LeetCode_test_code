use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    f64::INFINITY,
    fs::File,
    io::{Read, Result},
    mem::Discriminant,
    slice::Split,
    task::Context,
    thread,
};

use itertools::{izip, Itertools};

#[derive(Debug)]
struct Hailstone {
    m: f64,
    c: f64,
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}
impl Hailstone {
    fn new(raw_hailstone: &str) -> Self {
        let mut raw_hailstone = raw_hailstone.split(" @ ");
        let ps = raw_hailstone
            .next()
            .unwrap()
            .split(", ")
            .map(|raw_num| raw_num.replace(' ', "").parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        let vs = raw_hailstone
            .next()
            .unwrap()
            .split(", ")
            .map(|raw_num| raw_num.replace(' ', "").parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        let m = vs[1] / vs[0];
        Self {
            m,
            c: ps[1] - m * ps[0],
            px: ps[0],
            py: ps[1],
            pz: ps[2],
            vx: vs[0],
            vy: vs[1],
            vz: vs[2],
        }
    }
    /// Returns the t value if they intersect
    fn check_intersect(&self, other: &Self) -> Option<f64> {
        if self.m == other.m {
            return None;
        }
        let x = (other.c - self.c) / (self.m - other.m);
        let t = (x - self.px) / self.vx;
        Some(t)
    }
    fn get_x_y(&self, t: f64) -> (f64, f64) {
        (self.px + self.vx * t, self.py + self.vy * t)
    }
}

fn main() -> Result<()> {
    // const lower: f64 = 200000000000000_f64;
    // const higher: f64 = 400000000000000_f64;

    // const lower: f64 = 7.0;
    // const higher: f64 = 27.0;

    // let mut file_input = File::open("inputs/d24")?;
    let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let hailstones = input.split('\n').map(Hailstone::new).collect::<Vec<_>>();
    println!(
        "{:?}",
        hailstones
            .iter()
            .map(|hailstone| { (hailstone.m, hailstone.c) })
            .collect::<Vec<_>>()
    );
    for i in 0..hailstones.len() {
        let cur = &hailstones[i];
        for j in i + 1..hailstones.len() {
            let other = &hailstones[j];
            if let (Some(cur_t), Some(other_t)) =
                (cur.check_intersect(other), other.check_intersect(cur))
            {
                // if cur_t < 0.0 || other_t < 0.0 {
                //     continue;
                // }
                let (x, y) = cur.get_x_y(cur_t);
                println!(
                    "{}, {}: ({}, {}), cur_t: {}, other_t: {}",
                    i, j, x, y, cur_t, other_t
                );
            } else {
                println!("{}, {}: Parallel", i, j);
            }
        }
    }
    println!("sum = {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_direction_get() {}
}
