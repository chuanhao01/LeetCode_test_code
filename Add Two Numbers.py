# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        self.l1Check = True
        self.l1_l = []
        temp = l1
        while (self.l1Check):
            self.l1_l.append(str(temp.val))
            if (temp.next == None):
                self.l1Check = False
            else:
                temp = temp.next
        self.l2Check = True
        self.l2_l = []
        temp = l2
        while(self.l2Check):
            self.l2_l.append(str(temp.val))
            if (temp.next == None):
                self.l2Check = False
            else:
                temp = temp.next
        self.l1_i = int("".join(self.l1_l))
        self.l2_i = int("".join(self.l2_l))
        self.rtr = self.l1_i + self.l2_i
        self.rtr = str(self.rtr)
        self.temp_l = []
        for i in range(len(self.rtr)):
            if i+1 < len(self.rtr) - 1:
                if i == 0:
                    a = ListNode(int(self.rtr[i]))
                else:
                    b = ListNode(int(self.rtr[i]))
                    b.next = a
                    a = b
                    if i == len(self.rtr) - 1:
                        return b



