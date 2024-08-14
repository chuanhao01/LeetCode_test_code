# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        seen = {}
        if head is None:
            return False
        while head.next is not None:
            if head.val in seen:
                if seen[head.val] == head.next.val and head.val != head.next.val:
                    return True
            seen[head.val] = head.next.val
            head = head.next
        return False
