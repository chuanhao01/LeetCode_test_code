use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

// NOTES:
// Using swaping to solve
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
                if ffiles[i].0 == -1 {
                    // Skip
                    i += 1;
                    continue;
                } else {
                    ffinal.push(Vec::from([ffiles[ii]]));
                }
            }
            break;
        }
        if i >= ffiles.len() {
            break;
        }
        if ffiles[i].0 == -1 {
            // Skip
            i += 1;
            continue;
        }
        ffinal.push(vec![ffiles[i]]);
        let mut ffree = free[i];
        let mut current_free: Vec<(i64, i64)> = Vec::new();
        while ffree > 0 {
            for ii in 0..ffiles.len() - i - 1 {
                let cbf = ffiles[ffiles.len() - 1 - ii];
                // Skip moved
                if cbf.0 == -1 {
                    continue;
                }
                // Can fit the whole chunk
                if ffree >= cbf.1 {
                    // Becase swap
                    ffree -= cbf.1;
                    let cl = ffiles.len();
                    ffiles[cl - 1 - ii] = (-1, cbf.1);
                    current_free.push(cbf);
                    // Remove the free if end
                } else {
                    let cl = ffiles.len();
                    ffiles[cl - 1 - ii] = (cbf.0, cbf.1 - ffree);
                    current_free.push((cbf.0, ffree));
                    ffree = 0;
                }
            }
            // finished
            break;
        }
        ffinal.push(current_free);
        i += 1;
    }

    // println!("{:?}", ffinal);
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
