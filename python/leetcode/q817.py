from typing import List, Optional

from leetcode.struct import ListNode


class Solution:
    def numComponents(self, head: Optional[ListNode], nums: List[int]) -> int:
        curr = head
        numsSet = set(nums)
        ans = 0
        while curr:
            if curr.val in numsSet:
                while curr and curr.val in numsSet:
                    curr = curr.next
                ans += 1
            else:
                curr = curr.next
        return ans

    def numComponents2(self, head: Optional[ListNode], nums: List[int]) -> int:
        curr = head
        ans = 0
        numsSet = set(nums)
        inSet = False
        while curr:
            if curr.val not in numsSet:
                inSet = False
            elif not inSet:
                inSet = True
                ans += 1
            curr = curr.next
        return ans
