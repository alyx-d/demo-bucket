from itertools import pairwise
from math import inf
from typing import List, Optional
from leetcode.leetcode_struct import ListNode


class Soltuion:
    def nodeBetweenCriticalPoints(self, head: Optional[ListNode]) -> List[int]:
        arr = []
        curr = head
        while curr:
            arr.append(curr.val)
            curr = curr.next
        n = len(arr)
        if n <= 3:
            return [-1, -1]
        minDistance = inf
        maxDistance = -1
        idx = []
        for i in range(1, n - 1):
            if arr[i] > arr[i - 1] and arr[i] > arr[i + 1]:
                idx.append(i)
            if arr[i] < arr[i - 1] and arr[i] < arr[i + 1]:
                idx.append(i)
        if len(idx) <= 1:
            return [-1, -1]
        maxDistance = idx[-1] - idx[0]
        for a, b in pairwise(idx):
            minDistance = min(minDistance, b - a)
        if minDistance == inf or maxDistance == -1:
            return [-1, -1]
        return [minDistance, maxDistance]
