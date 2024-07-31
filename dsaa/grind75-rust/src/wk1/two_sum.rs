// https://leetcode.com/problems/two-sum/
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Assuming more than 2 elements
        let mut more_index: HashMap<i32, usize> = HashMap::new();
        more_index.insert(target - nums[0], 0);
        for i in 1..nums.len() {
            let num = nums[i];
            match more_index.get(&num) {
                Some(&ii) => return vec![ii as i32, i as i32],
                None => {
                    more_index.insert(target - num, i);
                }
            }
        }
        // Should never hit
        vec![0, 1]
    }
}
