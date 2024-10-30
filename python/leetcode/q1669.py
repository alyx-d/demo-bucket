from leetcode.struct import ListNode


class Solution:
    def mergeInBetween(
        self, list1: ListNode, a: int, b: int, list2: ListNode
    ) -> ListNode:
        prev = list1
        for i in range(a - 1):
            prev = prev.next
        curr = prev.next
        for i in range(b - a):
            curr = curr.next
        last = list2
        while last.next:
            last = last.next
        prev.next = list2
        last.next = curr.next
        return list1
