use std::{
    fs::File,
    io::{Read, Result},
};

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
        // println!("{:?}", nums);
        sum += if check_levels(nums, true) { 1 } else { 0 };
    }

    println!("sum: {}", sum);
    Ok(())
}

fn check_levels(levels: Vec<i32>, first: bool) -> bool {
    // Brain:
    // In hindsight, I could also just check the list excluding 1 element at a time
    let mut ascending = true;
    // NOTES: Main diff is that we do not know to remove the left or right
    // So go down both paths and check
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if diff == 0 {
            if first {
                let mut l = levels.clone();
                let mut r = levels.clone();
                l.remove(i - 1);
                r.remove(i);
                // println!("l: {:?}, {}", l, check_levels(l.clone(), false));
                // println!("r: {:?}, {}", r, check_levels(r.clone(), false));
                return check_levels(l, false) || check_levels(r, false);
            }
            return false;
        }
        // Check on first
        if i == 1 {
            ascending = diff > 0
        }
        // NOTES:
        // thought is the edge case of 10 13 12 11 10 9
        // since after the first, we say the list is ascending
        // but we do not know if the first number flips the list
        // So we need to include a check for that as well
        if ascending {
            if diff < 0 {
                if first {
                    let mut l = levels.clone();
                    let mut r = levels.clone();
                    l.remove(i - 1);
                    r.remove(i);
                    // println!("l: {:?}, {}", l, check_levels(l.clone(), false));
                    // println!("r: {:?}, {}", r, check_levels(r.clone(), false));
                    if i == 2 {
                        return check_levels(l, false)
                            || check_levels(r, false)
                            || check_levels(
                                levels
                                    .clone()
                                    .iter()
                                    .enumerate()
                                    .filter(|ii| ii.0 != 0)
                                    .map(|ii| *ii.1)
                                    .collect::<Vec<i32>>(),
                                false,
                            );
                    }
                    return check_levels(l, false) || check_levels(r, false);
                }
                return false;
            }
        } else {
            if diff > 0 {
                if first {
                    let mut l = levels.clone();
                    let mut r = levels.clone();
                    l.remove(i - 1);
                    r.remove(i);
                    // println!("l: {:?}, {}", l, check_levels(l.clone(), false));
                    // println!("r: {:?}, {}", r, check_levels(r.clone(), false));
                    if i == 2 {
                        return check_levels(l, false)
                            || check_levels(r, false)
                            || check_levels(
                                levels
                                    .clone()
                                    .iter()
                                    .enumerate()
                                    .filter(|ii| ii.0 != 0)
                                    .map(|ii| *ii.1)
                                    .collect::<Vec<i32>>(),
                                false,
                            );
                    }
                    return check_levels(l, false) || check_levels(r, false);
                }
                return false;
            }
        }

        if diff.abs() > 3 {
            if first {
                let mut l = levels.clone();
                let mut r = levels.clone();
                l.remove(i - 1);
                r.remove(i);
                println!("l: {:?}, {}", l, check_levels(l.clone(), false));
                println!("r: {:?}, {}", r, check_levels(r.clone(), false));
                return check_levels(l, false) || check_levels(r, false);
            }
            return false;
        }
    }
    true
}
