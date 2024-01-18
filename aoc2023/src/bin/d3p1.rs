use std::{
    fmt::Debug,
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

#[derive(Debug)]
struct No {
    body: i64,
    y: i64,
    start: i64,
    end: i64,
}

impl No {
    fn get_is(&self) -> Vec<(i64, i64)> {
        (self.start..self.end)
            .map(|x| (self.y, x))
            .collect::<Vec<(i64, i64)>>()
    }
}
impl PartialEq for No {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
            && self.y == other.y
            && self.start == other.start
            && self.end == other.end
    }
}

fn parseNos(map: &str) -> Vec<No> {
    let mut nos = Vec::new();
    for (y, l) in map.split('\n').enumerate() {
        let mut currently_digit = false;
        let mut start = 0;
        let mut body = String::new();
        for (x, c) in l.chars().enumerate() {
            if c.is_ascii_digit() && !currently_digit {
                currently_digit = true;
                start = x;
            } else if !c.is_ascii_digit() && currently_digit {
                currently_digit = false;
                nos.push(No {
                    body: body.parse::<i64>().unwrap(),
                    y: y as i64,
                    start: start as i64,
                    end: x as i64,
                });
                body = String::new();
            }
            if c.is_ascii_digit() {
                body += &c.to_string();
            }
        }
        // Handle if final is on digit
        if currently_digit {
            nos.push(No {
                body: body.parse::<i64>().unwrap(),
                y: y as i64,
                start: start as i64,
                end: l.len() as i64,
            });
        }
    }
    nos
}

fn get_around_index(map: Vec<Vec<String>>, y: i64, x: i64)-> Vec<(i64, i64)>{
    let max_y = map.len() as i64;
    let max_x = map[0].len() as i64;

    let mut indexes = Vec::new();

    for (dy, dx) in izip!([-1, -1, -1, 0, 0, 1, 1, 1], [-1, 0, 1, -1, 1, -1, 0, 1]){
        let ny = y + dy;
        let nx = x + dx;
        if 0 <= ny && ny < max_y && 0 <= nx && nx < max_x{
            indexes.push((ny, nx));
        }
    }
    indexes
}

fn check_around_no(map: Vec<Vec<String>>, no: No)-> Option<i64>{
    for (y, x) in no.get_is(){
        let indexes = get_around_index(map.clone(), y, x);
        for index in indexes{
            let c = &map[index.0 as usize][index.1 as usize].chars().collect::<Vec<_>>()[0];
            if ['@', '#', '$', '%', '&', '*', '-', '+', '=', '/'].iter().any(|cc| cc == c){
                return Some(no.body);
            }
        }
    }
    None
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d3")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let map = input.split('\n').map(|l| {l.chars().map(|c| c.to_string()).collect::<Vec<_>>()}).collect::<Vec<_>>();

    let nos = parseNos(&input);
    for no in nos{
        if let Some(n) = check_around_no(map.clone(), no) {
            sum += n;
        }
    }

    println!("{:?}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nos_get_is() {
        let a = No {
            body: 467,
            y: 0,
            start: 0,
            end: 3,
        };
        assert_eq!(Vec::from([(0, 0), (0, 1), (0, 2)]), a.get_is());
    }

    #[test]
    fn test_parse_nos() {
        let a = parseNos("467..114..112\n...*......22.");
        assert_eq!(
            No {
                body: 467,
                y: 0,
                start: 0,
                end: 3
            },
            a[0]
        );
        assert_eq!(
            No {
                body: 112,
                y: 0,
                start: 10,
                end: 13
            },
            a[2]
        );
    }
}
