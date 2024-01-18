use std::{
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d6")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let input = input.split('\n').collect::<Vec<_>>();
    let times = input[0].replace(' ', "").parse::<i64>().unwrap();
    let distances = input[1].replace(' ', "").parse::<i64>().unwrap();
    println!("t: {}, d: {}", times, distances);

    let mut sum = 0;
    for l in 0..times{
        let r = times - l;
        if l*r > distances {
            sum += 1;
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
