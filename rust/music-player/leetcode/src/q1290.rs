use crate::{ListNode, Solution};

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        if head.is_none() {
            return 0;
        }
        let mut len = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }
        let mut ans = 0;
        curr = &head;
        while let Some(node) = curr {
            ans += node.val * 2_i32.pow(len - 1);
            curr = &node.next;
            len -= 1;
        }
        ans
    }

    pub fn get_decimal_value2(head: Option<Box<ListNode>>) -> i32 {
        let mut ans = 0;
        let mut curr = head;
        while let Some(node) = curr {
            ans = (ans << 1) + node.val;
            curr = node.next;
        }
        ans
    }
}
