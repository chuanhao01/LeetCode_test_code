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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Part {
    min: i64,
    max: i64,
}
impl Part {
    fn extend(&self, other: &Self) -> Self {
        Self {
            min: i64::min(self.min, other.min),
            max: i64::max(self.max, other.max),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
        .collect::<HashMap<String, Vec<PossibleInstruction>>>();
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
    fn get_all_possible_parts(
        current_index: i64,
        instruction_set: String,
        instructions: &HashMap<String, Vec<PossibleInstruction>>,
        part: HashMap<PartCategory, Part>,
        acc: HashSet<Vec<Part>>,
    ) -> HashSet<Vec<Part>> {
        let current_instruction = &instructions[&instruction_set][current_index as usize];
        let mut acc = acc;
        match current_instruction {
            PossibleInstruction::Instruction(instruction) => {
                // Assuming Less is the ordering, part_1 follows the rule, while part_2 goes onto the next instruction
                let mut part_1 = part.clone();
                part_1.get_mut(&instruction.category).unwrap().max = instruction.amount - 1;
                let mut part_2 = part.clone();
                part_2.get_mut(&instruction.category).unwrap().min = instruction.amount + 1;
                if instruction.condition == Ordering::Greater {
                    // Swap them
                    (part_2, part_1) = (part_1, part_2);
                }
                let mut possible_parts: HashSet<Vec<Part>> = HashSet::new();
                if part_1.iter().all(|(_, part)| part.min < part.max) {
                    let outcome = match &instruction.rule {
                        Rule::Accepted => HashSet::from([part_1
                            .into_iter()
                            .map(|part| part.1)
                            .collect::<Vec<_>>()]),
                        Rule::Rejected => HashSet::new(),
                        Rule::GoTo(new_instruction) => get_all_possible_parts(
                            0,
                            new_instruction.to_string(),
                            instructions,
                            part_1,
                            acc.clone(),
                        ),
                    };
                    possible_parts.extend(outcome);
                }
                if part_2.iter().all(|(_, part)| part.min < part.max) {
                    possible_parts.extend(get_all_possible_parts(
                        current_index + 1,
                        instruction_set,
                        instructions,
                        part,
                        acc.clone(),
                    ));
                }
                acc.extend(possible_parts);
                acc
            }
            PossibleInstruction::Rule(rule) => match rule {
                Rule::Accepted => {
                    acc.extend(HashSet::from([part
                        .into_iter()
                        .map(|part| part.1)
                        .collect::<Vec<_>>()]));
                    acc
                }
                Rule::Rejected => HashSet::new(),
                Rule::GoTo(new_instruction) => {
                    get_all_possible_parts(0, new_instruction.to_string(), instructions, part, acc)
                }
            },
        }
    }
    let mut possible_parts = get_all_possible_parts(
        0,
        String::from("in"),
        &instructions,
        HashMap::from([
            (PartCategory::A, Part { min: 1, max: 4000 }),
            (PartCategory::M, Part { min: 1, max: 4000 }),
            (PartCategory::S, Part { min: 1, max: 4000 }),
            (PartCategory::X, Part { min: 1, max: 4000 }),
        ]),
        HashSet::new(),
    );
    println!("{:?}", possible_parts);
    // let possible_parts = possible_parts.into_iter().reduce(|acc, parts| {
    //     izip!(acc, parts)
    //         .map(|(acc_part, part)| acc_part.extend(&part))
    //         .collect::<Vec<_>>()
    // });
    sum = possible_parts.iter().fold(0, |acc, possible_part| {
        acc + possible_part
            .iter()
            .fold(1, |accc, part| accc * (part.max - part.min + 1))
    });
    // sum = possible_parts.len();

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
