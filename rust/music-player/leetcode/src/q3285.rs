use crate::Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut count = 0;
        let chars = s.chars().into_iter().collect::<Vec<char>>();
        for i in 0..s.len() {
            let mut count_0 = 0;
            let mut count_1 = 0;
            for j in i..s.len() {
                match chars[j] {
                    '0' => count_0 += 1,
                    '1' => count_1 += 1,
                    _ => break,
                }
                if count_0 <= k || count_1 <= k {
                    count += 1;
                } else {
                    break;
                }
            }
        }
        count
    }
}
