# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        self.checkMerge = True
        self.list = []
        self.templ1 = l1
        self.templ2 = l2
        while(self.checkMerge):
            try:
                self.list.append(self.templ1.val)
            except:
                pass
            try:
                self.list.append(self.templ2.val)
            except:
                pass
            if (self.templ1.next == None and self.templ2.next == None):
                self.checkMerge = False
            self.templ1 = self.templ1.next
            self.templ2 = self.templ2.next
        print(self.list)