use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d1")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;

    input = input.replace("one", "o1e");
    input = input.replace("two", "t2o");
    input = input.replace("three", "t3e");
    input = input.replace("four", "f4r");
    input = input.replace("five", "f5e");
    input = input.replace("six", "s6x");
    input = input.replace("seven", "s7n");
    input = input.replace("eight", "e8t");
    input = input.replace("nine", "n9e");
    for l in input.split('\n') {
        let mut digits: Vec<char> = Vec::new();

        let l = l.to_string();
        for c in l.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            }
        }
        // println!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        sum += format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<i64>()
            .unwrap();
    }
    println!("sum: {}", sum);
    Ok(())
}
