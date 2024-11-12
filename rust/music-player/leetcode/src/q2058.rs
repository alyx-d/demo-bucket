use std::cmp::min;

use crate::{ListNode, Solution};

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = vec![-1; 2];
        let mut pd = vec![0; 0];
        let mut prev = head.as_ref();
        let mut curr = head.as_ref().unwrap().next.as_ref();
        let mut next = curr.as_ref().unwrap().next.as_ref();
        let mut idx = 1;
        while next.is_some() {
            if prev.as_ref().unwrap().val < curr.as_ref().unwrap().val
                && curr.as_ref().unwrap().val > next.as_ref().unwrap().val
                || prev.as_ref().unwrap().val > curr.as_ref().unwrap().val
                    && curr.as_ref().unwrap().val < next.as_ref().unwrap().val
            {
                if pd.len() < 2 {
                    pd.push(idx);
                    if pd.len() == 2 {
                        ans[0] = pd[1] - pd[0];
                        ans[1] = ans[0];
                    }
                } else {
                    ans[0] = min(ans[0], idx - pd[1]);
                    ans[1] = idx - pd[0];
                    pd[1] = idx;
                }
            }
            prev = curr;
            curr = next;
            next = next.as_ref().unwrap().next.as_ref();
            idx += 1;
        }
        ans
    }
}
