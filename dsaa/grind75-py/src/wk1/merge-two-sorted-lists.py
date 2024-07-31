# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        head = None
        cur = None
        while list1 is not None or list2 is not None:
            if list1 is None:
                if cur is None:
                    return list2
                else:
                    cur.next = list2
                    return head
            if list2 is None:
                if cur is None:
                    return list1
                else:
                    cur.next = list1
                    return head
            if list1.val < list2.val:
                if cur is None:
                    head = list1
                    cur = list1
                else:
                    cur.next = list1
                    cur = cur.next
                list1 = list1.next
            else:
                if cur is None:
                    head = list2
                    cur = list2
                else:
                    cur.next = list2
                    cur = cur.next
                list2 = list2.next
        return head
