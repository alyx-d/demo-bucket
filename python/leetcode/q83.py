from typing import Optional
from leetcode.struct import ListNode


class Solution:
    def deleteDulplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next:
            return head
        curr = head
        while curr and curr.next:
            if curr.val == curr.next.val:
                val = curr.val
                while curr and curr.next and curr.next.val == val:
                    curr.next = curr.next.next
            else:
                curr = curr.next
        return head

    def deleteDuplicates2(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return head
        curr = head
        while curr.next:
            if curr.val == curr.next.val:
                curr.next = curr.next.next
            else:
                curr = curr.next
        return head
