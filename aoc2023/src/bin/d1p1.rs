use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d1")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    for l in input.split('\n') {
        let mut digits: Vec<char> = Vec::new();

        let l = l.to_string();
        for c in l.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            }
        }
        sum += format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<i64>()
            .unwrap();
    }
    println!("sum: {}", sum);
    Ok(())
}
