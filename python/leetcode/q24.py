from typing import Optional

from leetcode.struct import ListNode


class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next:
            return head
        vHead = ListNode(next=head)
        prev = vHead
        curr = head
        next = head.next
        while curr and next:
            node = next.next
            prev.next = next
            next.next = curr
            curr.next = node
            prev = curr
            curr = node
            if node:
                next = node.next
        return vHead.next

    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next:
            return head
        newHead = head.next
        head.next = self.swapPairs(newHead.next)
        newHead.next = head
        return newHead
