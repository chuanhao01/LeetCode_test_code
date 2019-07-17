class Solution:
    def numJewelsInStones(self, J, S):
        counter_jewels = 0
        jewels = 0
        new_arr = []
        for i in S:
            new_arr.append(i)
        S = new_arr
        while(jewels<len(J)):
            stone = len(S) - 1
            while(stone > -1):
                if(S[stone] == J[jewels]):
                    counter_jewels += 1
                    S.pop(stone)
                stone -= 1
            jewels += 1
        return counter_jewels