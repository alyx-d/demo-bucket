from math import inf
from typing import Optional

from leetcode.struct import ListNode


class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return head
        vHead = ListNode(inf, next=head)
        curr = vHead
        while curr and curr.next and curr.next.next:
            if curr.next.val == curr.next.next.val:
                val = curr.next.val
                while curr and curr.next and curr.next.val == val:
                    curr.next = curr.next.next
            else:
                curr = curr.next
        return vHead.next
