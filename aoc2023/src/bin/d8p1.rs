use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

#[derive(Debug)]
struct Next {
    left: String,
    right: String,
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d8")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let input = input.split("\n\n").collect::<Vec<_>>();
    let seq = input[0].chars().map(|c| c.to_string()).collect::<Vec<_>>();
    // println!("{:?}", seq);
    let raw_mappings = input[1];
    let mut map: HashMap<String, Next> = HashMap::new();
    for raw_mapping in raw_mappings.split('\n') {
        let mapping = raw_mapping.split(" = ").collect::<Vec<_>>();
        let from = mapping[0];
        let raw_locations = mapping[1].replace(['(', ')'], "");
        let locations = raw_locations.split(", ").collect::<Vec<_>>();
        // println!("from: {}, locations: {:?}", from, locations);
        map.insert(
            from.to_string(),
            Next {
                left: locations[0].to_string(),
                right: locations[1].to_string(),
            },
        );
    }
    let mut current = String::from("AAA");
    let mut index = 0;
    while current != "ZZZ" {
        let direction = &seq[index];
        let mapping = &map[&current];
        current = if direction == "L" {
            mapping.left.clone()
        } else {
            mapping.right.clone()
        };
        index = if index < seq.len() - 1 { index + 1 } else { 0 };
        sum += 1;
    }

    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
}
