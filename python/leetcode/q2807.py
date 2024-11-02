from typing import Optional

from leetcode_struct import ListNode


class Solution:
    def insertGreatCommonDivisors(self, head: Optional[ListNode]) -> Optional[ListNode]:
        def gcd(a, b):
            while b:
                a, b = b, a % b
            return a

        if not head or not head.next:
            return head
        prev = head
        curr = prev.next
        while prev and curr:
            node = ListNode(gcd(prev.val, curr.val))
            print((prev.val, curr.val, node.val))
            prev.next = node
            node.next = curr
            prev = curr
            curr = curr.next
        return head


if __name__ == "__main__":
    s = Solution()
    head = ListNode.build([18, 6, 10, 3])
    print(head)
    result = s.insertGreatCommonDivisors(head)
    print(result)
