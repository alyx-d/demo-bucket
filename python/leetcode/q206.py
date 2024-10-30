from typing import Optional

from leetcode.struct import ListNode


class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next:
            return head
        newHead = self.reverseList(head.next)
        head.next.next = head
        head.next = None
        return newHead

    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return head
        prev = None
        curr = head
        while curr:
            next = curr.next
            curr.next = prev
            prev = curr
            curr = next
        return prev
