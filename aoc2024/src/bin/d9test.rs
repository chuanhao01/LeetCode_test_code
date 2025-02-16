// start 2115
// end

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

// NOTES:
fn main() {
    let a = "00992111777.44.333....5555.6666.....8888..";
    let mut sum: i64 = 0;
    for (i, c) in a.chars().enumerate() {
        if c != '.' {
            let n: i64 = c.to_string().parse().unwrap();
            sum += i as i64 * n;
        }
    }
    println!("{}", sum);
}
