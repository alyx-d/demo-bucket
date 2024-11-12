use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&(target - nums[i])) {
                ans.push(i as i32);
                ans.push(*map.get(&(target - nums[i])).unwrap());
                break;
            } else {
                map.insert(nums[i], i as i32);
            }
        }
        ans
    }
}
