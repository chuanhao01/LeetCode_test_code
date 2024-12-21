// start 2230
// end
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Result},
    vec,
};

use itertools::Itertools;
use queue::*;

struct Compiler {
    a: i128,
    b: i128,
    c: i128,
    out: Vec<i128>,
    i: usize,
}
impl Compiler {
    fn compile(a: i128, b: i128, c: i128, prog: &Vec<usize>) -> Vec<i128> {
        let mut compiler = Compiler {
            a,
            b,
            c,
            out: Vec::new(),
            i: 0,
        };
        while !compiler.is_at_end(prog) {
            compiler.consume(prog);
        }
        compiler.out.clone()
    }
    fn is_at_end(&mut self, prog: &Vec<usize>) -> bool {
        self.i >= prog.len()
    }
    fn consume(&mut self, prog: &Vec<usize>) {
        let opcode = prog[self.i];
        let operand = prog[self.i + 1];
        self.i += 2;
        // println!("{}", self.i);
        let combo = match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a.clone(),
            5 => self.b.clone(),
            6 => self.c.clone(),
            7 => -1,
            _ => panic!("oh oh"),
        };
        if combo < 0 {
            println!("nonono, {}: {}", self.i, combo);
        }
        match opcode {
            0 => {
                self.a = self.a / 2i128.pow(combo as u32);
            }
            1 => {
                self.b = self.b ^ operand as i128;
            }
            2 => self.b = combo % 8,
            3 => {
                if self.a == 0 {
                } else {
                    if operand < prog.len() {
                        self.i = operand;
                    }
                }
            }
            4 => {
                self.b = self.b ^ self.c;
            }
            5 => {
                self.out.push(combo % 8);
            }
            6 => {
                self.b = self.a / 2i128.pow(combo as u32);
            }
            7 => {
                self.c = self.a / 2i128.pow(combo as u32);
            }
            _ => panic!("Should not happen"),
        }
    }
}

// NOTES:
fn main() -> Result<()> {
    // let mut file_input = File::open("inputs/temp")?;
    let mut file_input = File::open("inputs/d17")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let mut sum = 0;
    let a = lines.next().unwrap();
    let a: i128 = a.split(": ").collect::<Vec<&str>>()[1].parse().unwrap();

    let b = lines.next().unwrap();
    let b: i128 = b.split(": ").collect::<Vec<&str>>()[1].parse().unwrap();

    let c = lines.next().unwrap();
    let c: i128 = c.split(": ").collect::<Vec<&str>>()[1].parse().unwrap();
    lines.next().unwrap();

    let prog = lines.next().unwrap();
    let prog: Vec<usize> = prog.split(": ").collect::<Vec<&str>>()[1]
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    // println!("{}, {}, {}", a, b, c);
    // println!("{:?}", prog);

    let out = Compiler::compile(a, b, c, &prog);
    println!("{:?}", out.iter().map(|num| num.to_string()).join(","));

    println!("sum: {}", sum);
    Ok(())
}
