use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d4")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    for line in input.split("\n") {
        let line = line.split(": ").collect::<Vec<_>>()[1];
        let winning_nos = line.split(" | ").collect::<Vec<_>>()[0];
        let nos = line.split(" | ").collect::<Vec<_>>()[1];

        let winning_nos = winning_nos
            .chars()
            .enumerate()
            .fold(Vec::new(), |mut acc: Vec<String>, (i, c)| {
                let im = i % 3;
                match im {
                    0 => {
                        acc.push(c.to_string());
                    }
                    1 => {
                        if let Some(last) = acc.last_mut() {
                            *last += &c.to_string();
                        }
                    }
                    2 => {}
                    _ => panic!("Should not have i%3 > 3"),
                };
                acc
            })
            .iter()
            .map(|no| no.replace(' ', ""))
            .map(|no| no.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let nos = nos
            .chars()
            .enumerate()
            .fold(Vec::new(), |mut acc: Vec<String>, (i, c)| {
                let im = i % 3;
                match im {
                    0 => {
                        acc.push(c.to_string());
                    }
                    1 => {
                        if let Some(last) = acc.last_mut() {
                            *last += &c.to_string();
                        }
                    }
                    2 => {}
                    _ => panic!("Should not have i%3 > 3"),
                };
                acc
            })
            .iter()
            .map(|no| no.replace(' ', ""))
            .map(|no| no.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        sum += nos.iter().fold(0, |acc, no| {
            if winning_nos.iter().any(|winning_no| winning_no == no) {
                if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            } else {
                acc
            }
        });
    }

    println!("sum: {}", sum);
    Ok(())
}
