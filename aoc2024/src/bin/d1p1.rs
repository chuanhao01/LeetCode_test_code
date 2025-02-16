use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d1")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut left_v: Vec<i32> = Vec::default();
    let mut right_v: Vec<i32> = Vec::default();
    for l in input.split('\n') {
        let nums = l.split("   ").collect::<Vec<&str>>();
        left_v.push(nums[0].parse().unwrap());
        right_v.push(nums[1].parse().unwrap());
    }

    left_v.sort();
    right_v.sort();

    for i in 0..left_v.len() {
        let l = left_v[i];
        let r = right_v[i];
        sum += (r - l).abs();
        // if r < l {
        //     println!("oh no, {} {} {}", i, l, r);
        // }
    }

    println!("sum: {}", sum);
    Ok(())
}
