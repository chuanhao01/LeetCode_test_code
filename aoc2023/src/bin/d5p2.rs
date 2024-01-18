use std::{
    fs::File,
    io::{Read, Result},
};

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i64,
    length: i64,
}

fn ranges_consume_mapper(ranges: Vec<Range>, mapper: &Mapper) -> Vec<Range> {
    let mut converted_ranges: Vec<Range> = Vec::new();
    let mut queue_ranges = ranges;
    for map in &mapper.maps {
        // println!("{:?}", queue_ranges);
        let mut next_check_convert_ranges: Vec<Range> = Vec::new();
        for range in &queue_ranges {
            // 1st case larger
            if map.source + map.range <= range.start {
                next_check_convert_ranges.push(*range);
                continue;
            }
            // 2nd case smaller
            if range.start < map.source && range.start + range.length < map.source {
                next_check_convert_ranges.push(*range);
                continue;
            }

            let mut left = range.start;
            let right = range.start + range.length;
            // println!("l: {:?}, r: {:?}", left, right);
            if left < map.source {
                next_check_convert_ranges.push(Range {
                    start: left,
                    length: map.source - left,
                });
                left = map.source;
            }

            if left == right {
                converted_ranges.push(Range {
                    start: map.dest,
                    length: 1,
                });
                continue;
            }

            if right <= map.source + map.range {
                converted_ranges.push(Range {
                    start: left - map.source + map.dest,
                    length: right - left,
                });
                continue;
            }
            converted_ranges.push(Range {
                start: map.dest,
                length: map.range,
            });
            next_check_convert_ranges.push(Range {
                start: map.source + map.range,
                length: right - map.source - map.range,
            });
        }
        queue_ranges = next_check_convert_ranges;
    }
    converted_ranges.append(&mut queue_ranges);
    converted_ranges
}

#[derive(Default, Debug)]
struct Mapper {
    maps: Vec<Map>,
}
impl Mapper {
    fn convert(&self, input: i64) -> i64 {
        for map in &self.maps {
            if let Some(converted_input) = map.convert(input) {
                return converted_input;
            }
        }
        input
    }
}

#[derive(Debug)]
struct Map {
    dest: i64,
    source: i64,
    range: i64,
}
impl Map {
    fn convert(&self, input: i64) -> Option<i64> {
        if self.source <= input && input < self.source + self.range {
            return Some(self.dest + input - self.source);
        }
        None
    }
}

fn generate_mappers(mapper_inputs: &[&str]) -> Vec<Mapper> {
    mapper_inputs
        .iter()
        .map(|mapper_input| -> Mapper {
            let mapper_input = mapper_input.split('\n').collect::<Vec<_>>();
            let maps = mapper_input[1..mapper_input.len()]
                .iter()
                .map(|line_of_numbers| {
                    let numbers = line_of_numbers
                        .split(' ')
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                    Map {
                        dest: numbers[0],
                        source: numbers[1],
                        range: numbers[2],
                    }
                })
                .collect::<Vec<_>>();
            Mapper { maps }
        })
        .collect::<Vec<_>>()
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d5")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let input = input.split("\n\n").collect::<Vec<_>>();
    let seeds = input[0].split(": ").collect::<Vec<_>>()[1]
        .split(' ')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mappers = generate_mappers(&input[1..input.len()]);

    let mut sum = 0;
    let mut inital_seeds: Vec<Range> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let length = seeds[i + 1];
        inital_seeds.push(Range { start, length });
    }
    // println!("{:?}", inital_seeds);
    // println!("{:?}", mappers);
    let final_range = mappers.iter().fold(inital_seeds, |acc, mapper| {
        let new_range = ranges_consume_mapper(acc, mapper);
        // println!("{:?}", new_range);
        new_range
    });
    // println!("{:?}", final_range);
    sum = final_range.iter().map(|range| {range.start}).min().unwrap();
    println!("sum: {}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranges_consume_mapper() {
        let inital_seed = vec![Range {
            start: 1,
            length: 10,
        }];
        let mapper = Mapper {
            maps: vec![
                Map {
                    dest: 100,
                    source: 2,
                    range: 3,
                },
                Map {
                    dest: 103,
                    source: 5,
                    range: 2,
                },
            ],
        };
        let out = ranges_consume_mapper(inital_seed, &mapper);
        println!("{:?}", out);
    }
}
