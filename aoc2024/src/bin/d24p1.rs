// start
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
    let mut file_input = File::open("inputs/temp")?;
    // let mut file_input = File::open("inputs/d25")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let mut sum = 0;
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut l = line.split("-");
        let left = l.next().unwrap().to_owned();
        let right = l.next().unwrap().to_owned();
        connections
            .entry(left)
            .and_modify(|v| v.push(right.clone()))
            .or_insert(vec![right]);
    }
    println!("{:?}", connections);
    let alt =
    for (k, v) in connections{

    }


    println!("sum: {}", sum);
    Ok(())
}
