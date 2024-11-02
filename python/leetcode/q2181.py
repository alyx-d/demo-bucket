from typing import Optional

from leetcode_struct import ListNode


class Solution:
    def mergeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        val = 0
        vHead = ListNode()
        curr = head.next
        vCurr = vHead
        while curr:
            val += curr.val
            if curr.val == 0:
                vCurr.next = ListNode(val)
                val = 0
                vCurr = vCurr.next
            curr = curr.next
        return vHead.next
