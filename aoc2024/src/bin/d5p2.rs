// start 0930
// end 1015

// Although a lot more thinking time the night before

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Result},
};

// NOTES:
fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d5")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut before_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut after_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut lines = input.split('\n');
    loop {
        let l = lines.next().unwrap();
        if l.is_empty() {
            break;
        }
        let mut nums = l.split("|");
        let before = nums.next().unwrap().parse().unwrap();
        let after = nums.next().unwrap().parse().unwrap();
        after_rules
            .entry(after)
            .and_modify(|nums| nums.push(before))
            .or_insert(vec![before]);
        before_rules
            .entry(before)
            .and_modify(|nums| nums.push(after))
            .or_insert(vec![after]);
    }
    loop {
        let l = lines.next().unwrap();
        if l.is_empty() {
            break;
        }
        let updates = l
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i32>>();
        let update_hm = updates
            .clone()
            .iter()
            .enumerate()
            .map(|(i, num)| (*num, i as i32))
            .collect::<HashMap<i32, i32>>();
        let mut safe = true;
        for (i, update) in updates.iter().enumerate() {
            if !safe {
                break;
            }
            if let Some(afters) = after_rules.get(&update) {
                for after in afters {
                    if let Some(ele_i) = update_hm.get(after) {
                        if i as i32 - ele_i < 0 {
                            safe = false;
                            break;
                        }
                    }
                }
            }
            if let Some(befores) = before_rules.get(&update) {
                for before in befores {
                    if let Some(ele_i) = update_hm.get(before) {
                        if i as i32 - ele_i > 0 {
                            safe = false;
                            break;
                        }
                    }
                }
            }
        }

        // println!("{:?}", updates);

        if !safe {
            let mut updates = updates;
            let mut new_updates: Vec<i32> = Vec::new();
            while !updates.is_empty() {
                for (i, update) in updates.clone().iter().enumerate() {
                    let other_updates = updates
                        .clone()
                        .into_iter()
                        .filter(|num| num != update)
                        .collect::<Vec<i32>>();
                    let mut possible = true;
                    if let Some(befores) = after_rules.get(update) {
                        for before in befores {
                            if other_updates.contains(before) {
                                // a before would come after, not possible skip
                                possible = false;
                                break;
                            }
                        }
                    }
                    if let Some(afters) = before_rules.get(update) {
                        for after in afters {
                            if new_updates.contains(after) {
                                // after is before, not possible skip
                                possible = false;
                                break;
                            }
                        }
                    }
                    if possible {
                        new_updates.push(updates.remove(i));
                        break;
                    }
                }
            }
            // println!("{:?}", new_updates);
            sum += new_updates[new_updates.len() / 2usize];
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
