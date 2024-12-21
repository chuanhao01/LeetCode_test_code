// start
// end

// Create a depiler that works backwards using the prog as input?
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
    let mut file_input = File::open("inputs/temp")?;
    // let mut file_input = File::open("inputs/d17")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;

    let mut lines = input.lines();
    // let mut possible: HashSet<&str> = HashSet::new();
    let possible = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();
    println!("{:?}", possible);

    // skip empty lines
    lines.next();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let target = line;
        sum += if DD::run_dfs(target.chars().collect(), &possible) {
            1
        } else {
            0
        };
    }

    println!("sum: {}", sum);
    Ok(())
}

struct DD {
    dp: HashMap<Vec<char>, bool>,
}

impl DD {
    fn run_dfs(target: Vec<char>, possible: &HashSet<String>) -> bool {
        let mut dd = DD {
            dp: HashMap::default(),
        };
        dd.dfs(target, possible)
    }

    fn dfs(&mut self, target: Vec<char>, possible: &HashSet<String>) -> bool {
        let max_len = possible
            .iter()
            .map(|s| {
                let a = s.chars().collect::<Vec<char>>().len();
                a
            })
            .max()
            .unwrap();
        if self.dp.contains_key(&target) {
            return *self.dp.get(&target).unwrap();
        }
        if target.len() == 0 {
            return true;
        }
        if target.len() == 1 {
            return possible.contains(&target.clone().into_iter().join(""));
        }
        if target.len() <= max_len {
            if possible.contains(&target.clone().into_iter().join("")) {
                true
            } else {
                // keep searching
                let mut is_it = false;
                for i in 0..target.len() {
                    let left = target[0..i + 1].to_vec();
                    let right = target[i + 1..target.len()].to_vec();
                    println!("{:?}, {:?}, {:?}", target, left, right);
                    is_it = is_it || (self.dfs(left, possible) && self.dfs(right, possible));
                }
                self.dp.insert(target.clone(), is_it);
                is_it
            }
        } else {
            let mut is_it = false;
            for i in 0..max_len {
                let left = target[0..i + 1].to_vec();
                let right = target[i + 1..target.len()].to_vec();
                // println!("{:?}, {:?}, {:?}", target, left, right);
                is_it = is_it || (self.dfs(left, possible) && self.dfs(right, possible));
            }
            self.dp.insert(target.clone(), is_it);
            is_it
        }
    }
}
