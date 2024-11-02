from typing import Optional, Tuple

from leetcode.leetcode_struct import ListNode


def test():
    head = ListNode.build([1, 2, 3, 4, 5])
    s = Solution()
    (first, last) = s.reverse(head, head.next.next)
    print(first)


class Solution:
    def reverse(self, first: ListNode, last: ListNode) -> Tuple[ListNode, ListNode]:
        prev = None
        curr = first
        end = last.next
        while curr != end:
            next = curr.next
            curr.next = prev
            prev = curr
            curr = next
        return (last, first)

    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        if not head:
            return head
        vHead = ListNode(next=head)
        prev = vHead
        while head:
            last = prev
            for _ in range(k):
                last = last.next
                if not last:
                    return vHead.next
            next = last.next
            (first, last) = self.reverse(head, last)
            prev.next = first
            last.next = next
            prev = last
            head = last.next
        return vHead.next
