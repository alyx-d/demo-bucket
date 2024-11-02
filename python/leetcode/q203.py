from typing import Optional

from leetcode.leetcode_struct import ListNode


class Solution:
    def removeElements(self, head: Optional[ListNode], val: int) -> Optional[ListNode]:
        vHead = ListNode(next=head)
        curr = head
        prev = vHead
        while curr:
            if curr.val == val:
                prev.next = curr.next
            else:
                prev = curr
            curr = curr.next
        return vHead.next

    def removeElements2(self, head: Optional[ListNode], val: int) -> Optional[ListNode]:
        if head == None:
            return None
        head.next = self.removeElements2(head.next, val)
        return head.next if head.val == val else head
