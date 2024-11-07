use std::cmp::min;

use crate::Solution;

impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        if min(x, y / 4) % 2 == 1 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}
