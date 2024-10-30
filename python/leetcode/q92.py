from typing import Optional

from struct import ListNode


class Solution:
    def reverseBetween(
        self, head: Optional[ListNode], left: int, right: int
    ) -> Optional[ListNode]:
        prev = head
        first = head
        last = head
        next = head
        for _ in range(left - 1):
            prev = first
            first = first.next
        last = first
        for _ in range(right - left):
            last = last.next
        next = last.next
        last.next = None
        self.reverseList(first)
        if prev != first:
            prev.next = last
        else:
            head = last
        if next:
            last.next = next
        return head

    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next:
            return head
        newHead = self.reverseList(head.next)
        head.next.next = head
        head.next = None
        return newHead


if __name__ == "__main__":
    head = ListNode.build([3, 5])
    s = Solution()
    result = s.reverseBetween(head, 1, 2)
    print(result)
