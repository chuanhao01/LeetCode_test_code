class Solution:
    def isValid(self, s):
        stack_check = []
        map_par = {
            ')' : '(',
            ']' : '[',
            '}' : '{'
        }
        for i in s:
            if(i == '(' or i == '[' or i == '{'):
                stack_check.append(i)
            else:
                if(i == ')' or i == ']' or i == '}'):
                    if(len(stack_check) == 0):
                        return False
                    top = stack_check.pop()
                    if(map_par[i] != top):
                        return False
        return len(stack_check) == 0
