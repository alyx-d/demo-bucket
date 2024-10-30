from typing import Optional

from leetcode.struct import ListNode


class Solution:
    """
    用后面的值覆盖前面的值
    node不能是最后一个节点
    """

    def deleteNode(self, node: Optional[ListNode]) -> None:
        node.val = node.next.val
        node.next = node.next.next
