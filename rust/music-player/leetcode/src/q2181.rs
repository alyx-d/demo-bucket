use crate::{ListNode, Solution};

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v_head = Some(Box::new(ListNode::new(0)));
        let mut curr = head.as_ref();
        let mut v_curr = v_head.as_mut();
        while curr.is_some() {
            if curr.as_ref().unwrap().val != 0 {
                v_curr.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(curr.as_ref().unwrap().val)));
                v_curr = v_curr.unwrap().next.as_mut();
                while curr.as_ref().unwrap().next.is_some()
                    && curr.as_ref().unwrap().next.as_ref().unwrap().val != 0
                {
                    v_curr.as_mut().unwrap().val +=
                        curr.as_ref().unwrap().next.as_ref().unwrap().val;
                    curr = curr.as_ref().unwrap().next.as_ref();
                }
            }
            curr = curr.as_ref().unwrap().next.as_ref();
        }
        v_head.unwrap().next
    }
}
