from typing import List, Optional

from leetcode.struct import ListNode


class Solution:
    def modifiedList(
        self, nums: List[int], head: Optional[ListNode]
    ) -> Optional[ListNode]:
        numsSet = set(nums)
        vHead = ListNode(next=head)
        curr = head
        prev = vHead
        while curr:
            if curr.val in numsSet:
                prev.next = curr.next
            else:
                prev = curr
            curr = curr.next
        return vHead.next
