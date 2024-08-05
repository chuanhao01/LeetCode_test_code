class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        s_counts = {}
        for c in s:
            if c in s_counts:
                s_counts[c]  += 1
            else :
                s_counts[c] = 1
        t_counts = {}
        for c in t:
            if c in t_counts:
                t_counts[c]  += 1
            else :
                t_counts[c] = 1
        return t_counts == s_counts

        

        