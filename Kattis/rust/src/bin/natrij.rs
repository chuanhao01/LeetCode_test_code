use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    io::{self, BufRead},
    ops::{Add, Neg, Sub},
};

#[derive(PartialEq, Eq, Debug)]
struct Time {
    hour: i64,
    minute: i64,
    second: i64,
}
impl Time {
    fn new(s: String) -> Self {
        let s: Vec<i64> = s.split(':').map(|ss| ss.parse().unwrap()).collect();
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
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}
impl Add for Time {
    type Output = Time;
    fn add(self, rhs: Time) -> Self::Output {
        let mut output = Time {
            hour: self.hour + rhs.hour,
            minute: self.minute + rhs.minute,
            second: self.second + rhs.second,
        };
        if output.second >= 60 {
            output.minute += output.second / 60;
            output.second %= 60;
        } else if output.second < 0 {
            output.minute -= 1;
            output.second += 60;
        }
        if output.minute >= 60 {
            output.hour += output.minute / 60;
            output.minute %= 60;
        } else if output.minute < 0 {
            output.hour -= 1;
            output.minute += 60;
        }
        output
    }
}
impl Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl Neg for Time {
    type Output = Time;
    fn neg(self) -> Self::Output {
        Time {
            hour: -self.hour,
            minute: -self.minute,
            second: -self.second,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let start = Time::new(lines.next().unwrap());
    let end = Time::new(lines.next().unwrap());
    match start.cmp(&end) {
        Ordering::Greater => {
            println!("{}", Time::new(String::from("24:00:00")) - start + end);
        }
        Ordering::Less => {
            println!("{}", end - start);
        }
        Ordering::Equal => {
            println!("24:00:00");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_addition() {
        let times = vec![
            (
                Time::new(String::from("00:00:00")),
                Time::new(String::from("04:00:00")),
                Time::new(String::from("04:00:00")),
            ),
            (
                Time::new(String::from("24:00:00")),
                -Time::new(String::from("04:00:00")),
                Time::new(String::from("20:00:00")),
            ),
            (
                Time::new(String::from("02:00:00")),
                -Time::new(String::from("00:00:01")),
                Time::new(String::from("01:59:59")),
            ),
        ];
        for (a, b, output) in times {
            assert_eq!(a + b, output)
        }
    }
}
