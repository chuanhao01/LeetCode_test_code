use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n: u64 = lines.next().unwrap().parse().unwrap();
    let mut max_fun = -1;
    let mut max_name: String = String::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let parsed_line: Vec<&str> = line.split(' ').collect();
        let name = parsed_line[0].to_owned();
        let fun: i64 = parsed_line[1].to_owned().parse().unwrap();
        if fun > max_fun {
            max_fun = fun;
            max_name = name.clone();
        }
    }
    println!("{}", max_name);
}
