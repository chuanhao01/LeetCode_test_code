// start 2000
// end

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

// NOTES:
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d9")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0i64;
    let mut files: Vec<i64> = Vec::new();
    let mut free: Vec<i64> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }

        let mut file = true;
        for c in l.chars() {
            let c = c.to_string().parse().unwrap();
            if file {
                files.push(c);
                file = false;
            } else {
                free.push(c);
                file = true;
            }
        }
    }
    let free = free.into_iter().filter(|f| *f != 0).collect::<Vec<i64>>();
    let free_space: i64 = free[0..free.len() - 1]
        .to_owned()
        .into_iter()
        .reduce(|acc, num| acc + num)
        .unwrap();
    files.reverse();
    let mut files_in_place: Vec<Vec<(i64, i64)>> = Vec::new();
    let mut files_from_back: Vec<Vec<(i64, i64)>> = Vec::new();
    let mut back_files: Vec<(i64, i64)> = Vec::new();
    let mut front_files: Vec<(i64, i64)> = Vec::new();
    let mut running_count = 0i64;
    for (i, file) in files.iter().enumerate() {
        // from the back
        if running_count >= free_space {
            front_files.push(((files.len() - i - 1) as i64, *file));
        } else {
            running_count += file;
            back_files.push(((files.len() - i - 1) as i64, *file));
        }
    }
    front_files.reverse();
    // println!("{:?}", front_files);
    for ff in free[0..free.len() - 1].to_owned() {
        let mut ff = ff;
        let mut current_free: Vec<(i64, i64)> = Vec::new();
        while ff > 0 {
            let cbf = back_files[0];
            if ff >= cbf.1 {
                ff -= cbf.1;
                let bf = back_files.remove(0);
                current_free.push(bf);
            } else {
                let bf = back_files[0].clone();
                back_files[0] = (bf.0, bf.1 - ff);
                current_free.push((bf.0, ff));
                ff = 0;
            }
        }
        files_from_back.push(current_free);
    }
    if !back_files.is_empty() {
        files_from_back.push(Vec::from([back_files[0]]));
    }
    // println!("{:?}", free);
    // println!("{:?}", files_from_back);
    let mut c: i64 = 0;
    for i in 0..front_files.len() {
        let ff = front_files[i];
        for _ in 0..ff.1 {
            sum += c * ff.0;
            c += 1;
        }
        let fs = &files_from_back[i];
        for fss in fs {
            for _ in 0..fss.1 {
                sum += c * fss.0;
                c += 1;
            }
        }
    }
    if files_from_back.len() > front_files.len() {
        for i in front_files.len()..files_from_back.len() {
            let fs = &files_from_back[i];
            for fss in fs {
                for _ in 0..fss.1 {
                    sum += c * fss.0;
                    c += 1;
                }
            }
        }
    }

    // Block
    // [[(id, length)*]]
    // Files in place
    // [(0,3)]
    // Free space
    // [[(31,2), (10,2)]]
    // Alternate between the 2

    // println!("{}", free_space);
    // println!("{:?}", files);
    // println!("{:?}", free);

    println!("sum: {}", sum);
    Ok(())
}
