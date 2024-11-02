from typing import Optional

from leetcode_struct import ListNode


class Solution:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        stack = []
        curr = head
        vHead = ListNode()
        while curr:
            while len(stack) and stack[-1].val < curr.val:
                stack.pop()
            stack.append(curr)
            curr = curr.next
        curr = vHead
        for it in stack:
            curr.next = it
            curr = curr.next
        return vHead.next

    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return head
        head.next = self.removeNodes(head.next)
        if head.next and head.val < head.next.val:
            return head.next
        return head


if __name__ == "__main__":
    s = Solution()
    arr = [5, 2, 13, 3, 8]

    def build():
        v = ListNode()
        curr = v
        for it in arr:
            curr.next = ListNode(it)
            curr = curr.next
        return v.next

    head = build()
    s.removeNodes(head)
