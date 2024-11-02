from typing import List, Optional, Self


class ListNode:
    def __init__(self, val=0, next: Optional[Self] = None):
        self.val = val
        self.next = next

    @staticmethod
    def build(arr: List[int]):
        v = ListNode()
        curr = v
        for it in arr:
            curr.next = ListNode(it)
            curr = curr.next
        return v.next

    def __str__(self):
        curr = self
        result = "["
        seq = ""
        while curr:
            result += seq + str(curr.val)
            seq = ","
            curr = curr.next
        return result + "]"
