use crate::{ListNode, Solution};

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let v_head = Some(Box::new(ListNode::new(0)));
        let mut curr = head.as_ref();
        let mut v_curr = v_head.clone();
        while curr.is_some() {
            if curr.as_ref().unwrap().val != 0 {
                v_curr.clone().unwrap().next = curr.cloned();
                while curr.as_ref().unwrap().next.is_some() {
                    v_curr.clone().unwrap().val +=
                        curr.as_ref().unwrap().next.as_ref().unwrap().val;
                    curr = curr.as_ref().unwrap().next.as_ref().unwrap().next.as_ref();
                }
                v_curr = v_curr.clone().unwrap().next;
            }
            curr = curr.as_ref().unwrap().next.as_ref();
        }
        v_head.unwrap().next
    }
}
