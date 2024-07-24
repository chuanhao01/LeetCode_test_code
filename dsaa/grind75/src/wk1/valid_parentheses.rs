struct Solution;

// https://leetcode.com/problems/valid-parentheses/

impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn check_corr(c: char, check: char) -> bool {
            match c {
                '(' => check == ')',
                '[' => check == ']',
                '{' => check == '}',
                _ => false,
            }
        }
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                }
                _ => {
                    if stack.is_empty(){
                        return false;
                    }
                    if !check_corr(stack[stack.len() - 1], c) {
                        return false;
                    } else {
                        stack.pop();
                    }
                }
            }
        }
        stack.is_empty()
    }
}
