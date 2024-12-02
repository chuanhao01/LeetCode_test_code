use std::{
    fs::File,
    io::{Read, Result},
};

// NOTES:
// Naive approach based on notes in d2p2
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d2")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut levels: Vec<Vec<i32>> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        let nums = l.split(" ").collect::<Vec<&str>>();
        let nums = nums
            .iter()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i32>>();
        println!("{:?}", nums);
        sum += if (0..nums.len())
            .map(|i| {
                check_levels(
                    nums.clone()
                        .iter()
                        .enumerate()
                        .filter(|(ii, _)| *ii != i)
                        .map(|(_, v)| *v)
                        .collect::<Vec<i32>>(),
                )
            })
            .any(|f| f)
        {
            1
        } else {
            0
        }
    }

    println!("sum: {}", sum);
    Ok(())
}

fn check_levels(levels: Vec<i32>) -> bool {
    let mut ascending = true;
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if diff == 0 {
            return false;
        }
        // Check on first
        if i == 1 {
            ascending = diff > 0
        }
        if ascending {
            if diff < 0 {
                return false;
            }
        } else {
            if diff > 0 {
                return false;
            }
        }

        if diff.abs() > 3 {
            return false;
        }
    }
    true
}
