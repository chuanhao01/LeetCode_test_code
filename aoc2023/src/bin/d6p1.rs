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
    let times = input[0].split(' ').map(|s|{s.parse::<i64>().unwrap()}).collect::<Vec<_>>();
    let distances = input[1].split(' ').map(|s|{s.parse::<i64>().unwrap()}).collect::<Vec<_>>();

    let mut ways_to_beat_record: Vec<i64> = Vec::new();
    for (time, distance) in izip!(times, distances){
        let mut ways = 0;
        for l in 0..time{
            let r = time - l;
            if l*r > distance {
                ways += 1;
            }
        }
        ways_to_beat_record.push(ways);
    }

    let mut sum = 0;
    sum = ways_to_beat_record.into_iter().reduce(|acc, n|{acc * n}).unwrap();
    println!("sum: {}", sum);
    Ok(())
}
