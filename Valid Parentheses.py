class Solution:
    def isValid(self, s: str) -> bool:
        self.open_l = ["{", "[", "("]
        self.close_l = ["}", "]", ")"]
        self.check_so_far = True
        for i in range(len(s)):
            for i1 in range(3):
                if s[i] == self.open_l[i1]:
                    if i+1 < len(s):
                        if s[i+1] == self.close_l[i1]:
                            pass
                        elif 