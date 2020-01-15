class Solution:
    def longestCommonPrefix(self, strs):
        if strs == []:
            return ""
        self.lengthOfShortestWord = len(strs[0])
        self.prefix_l = []
        for i in strs:
            if len(i) < self.lengthOfShortestWord:
                self.lengthOfShortestWord = len(i)
        for i in range(self.lengthOfShortestWord):
            self.check = []
            for i1 in strs:
                self.check.append(i1[i])
            if len(set(self.check)) == 1:
                self.prefix_l.append(self.check[0])
            else:
                break
        rtr_s = "".join(self.prefix_l)
        return rtr_s