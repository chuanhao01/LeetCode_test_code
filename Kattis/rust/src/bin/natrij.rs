use std::{
    fmt::Display,
    io::{self, BufRead},
};

#[derive(PartialEq, Eq)]
struct Time {
    hour: u64,
    minute: u64,
    second: u64,
}
impl Time {
    fn new(s: String) -> Self {
        let s: Vec<u64> = s.split(":").map(|ss| ss.parse().unwrap()).collect();
        Self {
            hour: s[0],
            minute: s[1],
            second: s[2],
        }
    }
}
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hour == other.hour {
            if self.minute == other.minute {
                self.second.cmp(&other.second)
            } else {
                self.minute.cmp(&other.minute)
            }
        } else {
            self.hour.cmp(&other.hour)
        }
    }
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    }
}

// TODO: implement all the other methods
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let start = Time::new(lines.next().unwrap());
    let end = Time::new(lines.next().unwrap());
    if start > end {}
}
