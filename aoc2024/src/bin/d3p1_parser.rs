// start 21:15
use std::{
    fs::File,
    io::{Read, Result},
};

// NOTES:
// Naive approach based on notes in d2p2
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut line: String = String::new();
    for l in input.split('\n') {
        if l.is_empty() {
            continue;
        }
        line = l.to_string();
    }
    sum = Parser::parse(line);

    println!("sum: {}", sum);
    Ok(())
}

struct Parser {
    start: usize,
    cur: usize,
    sum: usize,
}

impl Parser {
    fn new() -> Self {
        Self {
            start: 0,
            cur: 0,
            sum: 0,
        }
    }
    fn parse(line: String) -> usize {
        let line: Vec<char> = line.chars().collect();
        0
    }
    fn advance(&mut self, line: &Vec<char>) -> Option<char> {
        self.cur += 1;
        if self.cur - 1 >= line.len() {
            return None;
        }
        Some(line[self.cur - 1])
    }
    fn peek(&self, line: &Vec<char>) -> char {
        line[self.cur]
    }
    fn skip(&mut self, line: &Vec<char>) {
        // skip everything that is not m
        while self.cur <= line.len() {
            let c = self.peek(line);
            if c == 'm' {
                return;
            }
            self.advance(line);
        }
    }
    fn consume_token(&mut self, line: &Vec<char>) {}
    fn consume_mul(&mut self, line: &Vec<char>) {}
    fn next_token(&mut self, line: &Vec<char>) -> Option<String> {
        self.skip(line);
        // Reached the end
        if self.cur >= line.len() {
            return None;
        }
        // Consume and return a token
        self.start = self.cur;
        self.consume_token(line);
        Some(String::from("wow"))
    }
}
