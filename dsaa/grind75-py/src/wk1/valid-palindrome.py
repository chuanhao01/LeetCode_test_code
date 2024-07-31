class Solution:
    def isPalindrome(self, s: str) -> bool:
        s = s.lower()
        s = "".join(c for c in s if c.isalnum())
        for i in range(len(s) // 2):
            r = len(s) - i - 1
            if s[i] != s[r]:
                return False
        return True
