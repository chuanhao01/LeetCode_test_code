class Solution:
    def isValid(self, s):
        para_dict = {
        '{' : '}',
        '(' : ')',
        '[' : ']',
        }
        if(s == ''):
            return True
        i = 0
        if(len(s) == 1):
            return False
        while(i<len(s)):
            for key in para_dict:
                if(s[i] == para_dict[key]):
                    return False
            temp = s[i]
            if(s[i+1] != para_dict[temp]):
                return False
            else:
                if(i+2 == len(s)):
                    break
                else:
                    i += 2
        return True


def isValid(s):
    para_dict = {
        '{' : '}',
        '(' : ')',
        '[' : ']',

    }
    if(s == ''):
        return True
    i = 0
    while(i<len(s)):
        temp = s[i]
        if(s[i+1] != para_dict[temp]):
            return False
        else:
            if(i+2 == len(s)):
                break
            else:
                i += 2
    return True

print(isValid('()[]()'))