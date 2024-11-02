from typing import List, Optional


from leetcode.leetcode_struct import ListNode


class Solution:
    def splitListToParts(
        self, head: Optional[ListNode], k: int
    ) -> List[Optional[ListNode]]:
        n = 0
        curr = head
        while curr:
            n += 1
            curr = curr.next
        p = n % k
        q = n // k
        i = 0
        curr = head
        parts = [None] * k
        while curr and i < k:
            parts[i] = curr
            partSize = q + (1 if i < p else 0)
            for _ in range(partSize - 1):
                curr = curr.next
            next = curr.next
            curr.next = None
            curr = next
            i += 1
        return parts
