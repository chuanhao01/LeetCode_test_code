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
            self.l1_l.append(temp.val)
            if (temp.next == None):
                self.l1Check = False
            else:
                temp = temp.next
        self.l2Check = True
        self.l2_l = []
        temp = l2
        while(self.l2Check):
            self.l2_l.append(temp.val)
            if (temp.next == None):
                self.l2Check = False
            else:
                temp = temp.next
        temp = []
        for i in range(len(self.l1_l)):
            temp.append(str(self.l1_l[len(self.l1_l) - 1 - i]))
        self.l1_l = temp
        temp = []
        for i in range(len(self.l2_l)):
            temp.append(str(self.l2_l[len(self.l2_l) - 1 - i]))
        self.l2_l = temp
        self.l1_l = int("".join(self.l1_l))
        self.l2_l = int("".join(self.l2_l))
        self.rtr = self.l1_l + self.l2_l
        self.rtr = str(self.rtr)
        a = self.getLinkedList(self.rtr)
        return a

    def getLinkedList(self, item):
        for i in range(len(item)):
            if i == 0:
                self.temp = ListNode(int(item[i]))
            else:
                a = ListNode(int(item[i]))
                a.next = self.temp
                self.temp = a
        return self.temp






