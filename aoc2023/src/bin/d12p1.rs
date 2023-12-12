use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Result},
    ops::{Add, IndexMut},
};

use itertools::izip;

#[derive(Debug, Clone)]
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
    if spring_types.len() < information[0] as usize {
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
        c += count_poss(&spring_types[1..spring_types.len()], information);
    }
    if let SpringType::Damaged | SpringType::Unknown = spring_types[0] {
        c += if spring_types.len() > information[0] as usize {
            match spring_types[information[0] as usize] {
                SpringType::Unknown | SpringType::Working => count_poss(
                    &spring_types[information[0] as usize + 1..spring_types.len()],
                    &information[1..information.len()],
                ),
                SpringType::Damaged => 0,
            }
        } else {
            count_poss(
                &spring_types[information[0] as usize..spring_types.len()],
                &information[1..information.len()],
            )
        }
    }
    c
    // match spring_types[0] {
    //     SpringType::Working => count_poss(&spring_types[1..spring_types.len()], information),
    //     SpringType::Damaged => {
    //         if spring_types.len() > information[0] as usize {
    //             match spring_types[information[0] as usize] {
    //                 SpringType::Unknown | SpringType::Working => count_poss(
    //                     &spring_types[information[0] as usize + 1..spring_types.len()],
    //                     &information[1..information.len()],
    //                 ),
    //                 SpringType::Damaged => 0,
    //             }
    //         } else {
    //             count_poss(
    //                 &spring_types[information[0] as usize..spring_types.len()],
    //                 &information[1..information.len()],
    //             )
    //         }
    //     }
    //     SpringType::Unknown => {
    //         let working = count_poss(&spring_types[1..spring_types.len()], information);
    //         let damaged = if spring_types.len() > information[0] as usize {
    //             match spring_types[information[0] as usize] {
    //                 SpringType::Unknown | SpringType::Working => count_poss(
    //                     &spring_types[information[0] as usize + 1..spring_types.len()],
    //                     &information[1..information.len()],
    //                 ),
    //                 SpringType::Damaged => 0,
    //             }
    //         } else {
    //             count_poss(
    //                 &spring_types[information[0] as usize..spring_types.len()],
    //                 &information[1..information.len()],
    //             )
    //         };
    //         // println!("{}", working + damaged);
    //         working + damaged
    //     }
    // }
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
            let spring_types = row[0]
                .chars()
                .map(|c| SpringType::new(&c.to_string()))
                .collect::<Vec<_>>();
            let information = row[1]
                .split(',')
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (spring_types, information)
        })
        .collect::<Vec<_>>();
    sum = rows.iter().fold(0, |acc, (spring_types, information)| {
        println!("{}", count_poss(spring_types, information));
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
        let spring_types = "???.###."
            .chars()
            .map(|c| SpringType::new(&c.to_string()))
            .collect::<Vec<_>>();
        let information = vec![1, 1, 3];
        assert_eq!(count_poss(&spring_types, &information), 1);
    }
}
