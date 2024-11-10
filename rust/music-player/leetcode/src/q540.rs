use crate::Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] == nums[mid ^ 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return nums[left];
    }
}
