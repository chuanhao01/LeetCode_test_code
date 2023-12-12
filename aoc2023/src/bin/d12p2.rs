use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
    ops::{Add, IndexMut},
};

use itertools::izip;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SpringType {
    Working,
    Damaged,
    Unknown,
}
impl SpringType {
    fn new(raw_spring_type: &str) -> Self {
        match raw_spring_type {
            "." => Self::Working,
            "#" => Self::Damaged,
            "?" => Self::Unknown,
            _ => panic!("Wrong raw_spring_type: {}", raw_spring_type),
        }
    }
}

fn count_poss(spring_types: &[SpringType], information: &[i64]) -> i64 {
    // println!("{:?}, {:?}", spring_types, information);
    let mut cache = HashMap::new();
    fn run(
        cache: &mut HashMap<(Vec<SpringType>, Vec<i64>), i64>,
        spring_types: &[SpringType],
        information: &[i64],
    ) -> i64 {
        if let Some(v) = cache.get(&(spring_types.to_vec(), information.to_vec())) {
            return *v;
        }
        if information.is_empty() {
            return if spring_types
                .iter()
                .any(|spring_type| matches!(spring_type, SpringType::Damaged))
            {
                0
            } else {
                1
            };
        }
        if spring_types.len() < information.iter().sum::<i64>() as usize {
            return 0;
        }
        if information.len() == 1 && information[0] as usize == spring_types.len() {
            return if spring_types
                .iter()
                .any(|spring_type| matches!(spring_type, SpringType::Working))
            {
                0
            } else {
                1
            };
        }
        let mut c = 0;
        if let SpringType::Working | SpringType::Unknown = spring_types[0] {
            c += run(cache, &spring_types[1..spring_types.len()], information);
        }
        if let SpringType::Damaged | SpringType::Unknown = spring_types[0] {
            c += if let SpringType::Damaged = spring_types[information[0] as usize] {
                0
            } else if run(
                cache,
                &spring_types[..information[0] as usize],
                &[information[0]],
            ) == 0
            {
                0
            } else {
                run(
                    cache,
                    &spring_types[information[0] as usize + 1..spring_types.len()],
                    &information[1..information.len()],
                )
            }
        }
        // println!("{}", c);
        cache.insert((spring_types.to_vec(), information.to_vec()), c);
        c
    }
    run(&mut cache, spring_types, information)
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d12")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;
    let rows = input
        .split('\n')
        .map(|row| {
            let row = row.split(' ').collect::<Vec<_>>();
            let spring_types = ((row[0].to_owned() + "?").repeat(4) + row[0])
                .chars()
                .map(|c| SpringType::new(&c.to_string()))
                .collect::<Vec<_>>();
            let information = ((row[1].to_owned() + ",").repeat(4) + row[1])
                .split(',')
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (spring_types, information)
        })
        .collect::<Vec<_>>();
    sum = rows.iter().fold(0, |acc, (spring_types, information)| {
        acc + count_poss(spring_types, information)
    });

    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_pos() {
        let spring_types = "???.###"
            .chars()
            .map(|c| SpringType::new(&c.to_string()))
            .collect::<Vec<_>>();
        let information = vec![1, 1, 3];
        assert_eq!(count_poss(&spring_types, &information), 1);
    }
}
