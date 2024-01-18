use std::{
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

#[derive(Debug)]
struct Seq {
    body: Vec<i64>,
    next: Option<Box<Seq>>,
}
impl Seq {
    fn new(body: Vec<i64>) -> Self {
        let mut check_seq = Vec::new();
        for i in 0..body.len() - 1 {
            check_seq.push(body[i + 1] - body[i]);
        }
        Self {
            body: body.clone(),
            next: if body.iter().all(|num| *num == 0) {
                None
            } else {
                Some(Box::new(Self::new(check_seq)))
            },
        }
    }
    fn get_next_value(&self) -> i64 {
        match &self.next {
            Some(next) => self.body.last().unwrap() + next.get_next_value(),
            None => 0,
        }
    }
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d9")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let seqs = input
        .split('\n')
        .map(|raw_seq| {
            Seq::new(
                raw_seq
                    .split(' ')
                    .map(|raw_num| raw_num.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    sum = seqs.iter().fold(0, |acc, seq| acc + seq.get_next_value());

    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_seq() {
        let a = Seq::new(vec![
            4, 5, 3, -2, -10, -21, -35, -52, -72, -95, -121, -150, -182, -217, -255, -296, -340,
            -387, -437, -490, -546,
        ]);
        // println!("{:?}", a);
        assert_eq!(a.get_next_value(), 18);
    }
}
