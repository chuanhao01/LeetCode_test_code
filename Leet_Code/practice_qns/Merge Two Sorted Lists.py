# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        self.checkMerge = True
        self.l1check = False
        self.l2check = False
        self.list = []
        self.templ1 = l1
        self.templ2 = l2
        while(self.checkMerge):
            if (self.templ1 == None):
                self.l1check = True
            if self.templ2 == None:
                self.l2check = True
            if (self.l1check == False):
                self.list.append(self.templ1.val)
            if (self.l2check == False):
                self.list.append(self.templ2.val)
            if self.l1check == False:
                if (self.templ1.next != None):
                    self.templ1 = self.templ1.next
                elif (self.templ1.next == None):
                    self.l1check = True
            if self.l2check == False:
                if (self.templ2.next != None):
                    self.templ2 = self.templ2.next
                elif (self.templ2.next == None):
                    self.l2check = True
            if (self.l1check and self.l2check):
                self.checkMerge = False
        if (self.list == []):
            return None
        if (len(self.list) == 1):
            return ListNode(self.list[0])
        self.list.sort()
        for i in range(len(self.list)):
            if i == 0:
                self.tempLL = ListNode(self.list[len(self.list) - 1])
            else:
                self.tempLL1 = ListNode(self.list[len(self.list) - 1 - i])
                self.tempLL1.next = self.tempLL
                self.tempLL = self.tempLL1
        return self.tempLL1