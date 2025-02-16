use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d1")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut left_v: HashMap<i32, i32> = HashMap::new();
    let mut right_v: HashMap<i32, i32> = HashMap::new();
    for l in input.split('\n') {
        let nums = l.split("   ").collect::<Vec<&str>>();
        left_v
            .entry(nums[0].parse().unwrap())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        right_v
            .entry(nums[1].parse().unwrap())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    for k in left_v.keys() {
        if let Some(v) = right_v.get(k) {
            sum += k * left_v[k] * v;
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
