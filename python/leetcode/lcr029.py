from typing import Optional

from leetcode.struct import ListNode


class Solution:
    def insert(self, head: Optional[ListNode], insertVal: int) -> Optional[ListNode]:
        if not head:
            head = ListNode(insertVal)
            head.next = head
            return head
        prev = head
        curr = head.next
        while curr != head:
            if prev.val <= insertVal <= curr.val:
                break
            if prev.val > curr.val:
                if insertVal > prev.val or insertVal < curr.val:
                    break
            prev = prev.next
            curr = curr.next
        newNode = ListNode(insertVal)
        prev.next = newNode
        newNode.next = curr
        return head
