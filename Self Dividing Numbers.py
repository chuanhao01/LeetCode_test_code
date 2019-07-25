class Solution:
    def selfDividingNumbers(self, left, right):
        starting_num = left
        div_nums = []
        while(starting_num < right + 1):
            if(self.checkIfSelfNum(starting_num)):
                div_nums.append(starting_num)
            starting_num += 1
        return div_nums
    def checkIfSelfNum(self, num):
        for i in str(num):
            if(i == '0'):
                return False
            if(num%int(i) != 0):
                return False
        return True