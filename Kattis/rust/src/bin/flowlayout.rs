use std::io::{self, BufRead};

// TODO Some edge case idk
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap()).peekable();
    while lines.peek().unwrap() != "0" {
        let max_width: u64 = lines.next().unwrap().parse().unwrap();
        let mut boxes: Vec<(u64, u64)> = Vec::new();
        let mut cur_width = 0;
        let mut cur_max_height = 0;
        let mut c_max_width = 0;
        let mut c_height = 0;
        while lines.peek().unwrap() != "-1 -1" {
            let _box: Vec<u64> = lines
                .next()
                .unwrap()
                .split(' ')
                .map(|num| num.parse().unwrap())
                .collect();
            boxes.push((_box[0], _box[1]));
        }
        for (h, w) in boxes {
            if cur_width == 0 {
                cur_width = w;
                c_max_width = w;
                cur_max_height = h;
            } else {
                if cur_width + w <= max_width {
                    cur_width += w;
                    cur_max_height = cur_max_height.max(h);
                    c_max_width = c_max_width.max(cur_width);
                } else {
                    cur_width = w;
                    c_height += cur_max_height;
                    cur_max_height = h;
                }
            }
        }
        c_height += cur_max_height;
        c_max_width = c_max_width.max(cur_width);
        println!("{} x {}", c_height, c_max_width);
        // Skip -1 -1
        lines.next();
    }
    lines.next();
}
