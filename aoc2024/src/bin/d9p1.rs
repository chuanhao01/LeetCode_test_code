// start 2000
// end 2115

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

// NOTES:
// Sohuld have just done the logic
// anyway a bunch of array manipulation
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
    let mut ffiles: Vec<(i64, i64)> = Vec::new();
    for (i, file) in files.iter().enumerate() {
        ffiles.push(((i) as i64, *file));
    }
    // println!("{:?}", ffiles);
    let mut ffinal: Vec<Vec<(i64, i64)>> = Vec::new();
    let mut i = 0;
    loop {
        if i >= free.len() {
            for ii in i..ffiles.len() {
                ffinal.push(Vec::from([ffiles[ii]]));
            }
            break;
        }
        ffinal.push(Vec::from([ffiles[i]]));
        let mut ffree = free[i];
        let mut current_free: Vec<(i64, i64)> = Vec::new();
        while ffree > 0 {
            let cbf = ffiles[ffiles.len() - 1];
            if ffree >= cbf.1 {
                ffree -= cbf.1;
                let bf = ffiles.remove(ffiles.len() - 1);
                free.remove(free.len() - 1);
                current_free.push(bf);
            } else {
                let cbf_i = ffiles.len() - 1;
                ffiles[cbf_i] = (cbf.0, cbf.1 - ffree);
                current_free.push((cbf.0, ffree));
                ffree = 0;
            }
        }
        ffinal.push(current_free);
        i += 1;
    }
    println!("{:?}", ffinal);
    let mut c: i64 = 0;
    for ff in ffinal {
        for fff in ff {
            for _ in 0..fff.1 {
                sum += c * fff.0;
                c += 1;
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
