from typing import Optional

from leetcode.struct import ListNode


class Solution:
    def mergeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        val = 0
        curr = head.next
        vHead = ListNode()
        vCurr = vHead
        while curr:
            val += curr.val
            if curr.val == 0:
                vCurr.next = ListNode(val)
                val = 0
                vCurr = vCurr.next
            curr = curr.next
        return vHead.next
