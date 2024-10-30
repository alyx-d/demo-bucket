from typing import Optional

from leetcode.struct import ListNode


class Solution:
    def insertionSortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return head
        vHead = ListNode(next=head)
        lastSorted = head
        curr = lastSorted.next
        while curr:
            if curr.val >= lastSorted.val:
                lastSorted = lastSorted.next
            else:
                prev = vHead
                while prev.next.val <= curr.val:
                    prev = prev.next
                lastSorted.next = curr.next
                curr.next = prev.next
                prev.next = curr
            curr = lastSorted.next
        return vHead.next
