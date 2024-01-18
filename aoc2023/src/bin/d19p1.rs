use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    f64::INFINITY,
    fs::File,
    io::{Read, Result},
    mem::Discriminant,
    slice::Split,
};

use itertools::{izip, Itertools};

#[derive(Debug)]
struct Part {
    category: PartCategory,
    amount: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum PartCategory {
    X,
    M,
    A,
    S,
}
impl PartCategory {
    fn new(raw_part: &str) -> Self {
        match raw_part {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Unkown raw_part: {}", raw_part),
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    GoTo(String),
    Accepted,
    Rejected,
}
impl Rule {
    fn new(raw_rule: &str) -> Self {
        match raw_rule {
            "R" => Self::Rejected,
            "A" => Self::Accepted,
            raw_rule => Self::GoTo(raw_rule.to_string()),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    category: PartCategory,
    condition: Ordering,
    amount: i64,
    rule: Rule,
}
impl Instruction {
    fn new(raw_instruction: &str) -> Self {
        let condition = if raw_instruction.contains('>') {
            Ordering::Greater
        } else {
            Ordering::Less
        };
        let raw_part_and_rest = match condition {
            Ordering::Greater => raw_instruction.split('>').collect::<Vec<_>>(),
            Ordering::Less => raw_instruction.split('<').collect::<Vec<_>>(),
            _ => panic!("Should not have equal"),
        };
        let part = PartCategory::new(raw_part_and_rest[0]);
        let rest = raw_part_and_rest[1];
        let raw_amount_and_rule = rest.split(':').collect::<Vec<_>>();
        let amount = raw_amount_and_rule[0].parse::<i64>().unwrap();
        let rule = Rule::new(raw_amount_and_rule[1]);
        Self {
            category: part,
            condition,
            amount,
            rule,
        }
    }
    fn run(&self, parts_count: &HashMap<PartCategory, i64>) -> Option<Rule> {
        let part_count = parts_count[&self.category];
        if self.condition != part_count.cmp(&self.amount) {
            return None;
        }
        Some(self.rule.clone())
    }
}

#[derive(Debug)]
enum PossibleInstruction {
    Instruction(Instruction),
    Rule(Rule),
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d19")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;
    let input = input.split("\n\n").collect::<Vec<_>>();
    let instructions = input[0]
        .split('\n')
        .map(|raw_row| {
            let raw_row = raw_row.split('{').collect::<Vec<_>>();
            let instruction_name = raw_row[0];
            let instructions = raw_row[1]
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|raw_instruction| {
                    if !raw_instruction.contains(':') {
                        return PossibleInstruction::Rule(Rule::new(raw_instruction));
                    }
                    PossibleInstruction::Instruction(Instruction::new(raw_instruction))
                })
                .collect::<Vec<PossibleInstruction>>();
            (instruction_name.to_string(), instructions)
        })
        .collect::<Vec<(String, Vec<PossibleInstruction>)>>();
    let parts_counts = input[1]
        .split('\n')
        .map(|raw_row| {
            let raw_row = raw_row.replace(['{', '}'], "");
            raw_row
                .split(',')
                .map(|raw_part| {
                    let split = raw_part.split('=');
                    let raw_part = split.collect::<Vec<_>>();
                    (
                        PartCategory::new(raw_part[0]),
                        raw_part[1].parse::<i64>().unwrap(),
                    )
                })
                .collect::<HashMap<PartCategory, i64>>()
        })
        .collect::<Vec<_>>();
    let first_instruction = "in";
    let instructions = instructions
        .into_iter()
        .collect::<HashMap<String, Vec<PossibleInstruction>>>();
    for parts_count in parts_counts {
        let mut instruction_set = &instructions[first_instruction];
        let mut current_index = 0;
        let mut current_instruction = &instruction_set[current_index];
        loop {
            match current_instruction {
                PossibleInstruction::Instruction(instruction) => {
                    match instruction.run(&parts_count) {
                        None => {
                            current_index += 1;
                            current_instruction = &instruction_set[current_index];
                        }
                        Some(rule) => match &rule {
                            Rule::Accepted => {
                                sum += parts_count.iter().fold(0, |acc, (_, count)| acc + *count);
                                break;
                            }
                            Rule::Rejected => {
                                break;
                            }
                            Rule::GoTo(new_instruction) => {
                                instruction_set = &instructions[new_instruction];
                                current_index = 0;
                                current_instruction = &instruction_set[current_index];
                            }
                        },
                    }
                }
                PossibleInstruction::Rule(rule) => match rule {
                    Rule::Accepted => {
                        sum += parts_count.iter().fold(0, |acc, (_, count)| acc + *count);
                        break;
                    }
                    Rule::Rejected => {
                        break;
                    }
                    Rule::GoTo(new_instruction) => {
                        instruction_set = &instructions[new_instruction];
                        current_index = 0;
                        current_instruction = &instruction_set[current_index];
                    }
                },
            };
        }
    }

    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_direction_get() {}
}
