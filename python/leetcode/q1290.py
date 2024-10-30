from typing import Optional

from leetcode.struct import ListNode


class Solution:
    def getDecimalValue(self, head: Optional[ListNode]) -> int:
        arr = []
        curr = head
        while curr:
            arr.append(curr.val)
            curr = curr.next
        ans = 0
        n = len(arr)
        for i in range(n):
            ans += 2 ** (n - 1 - i) * arr[i]
        return ans
