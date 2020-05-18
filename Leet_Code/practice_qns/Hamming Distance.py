import math
class Solution:
    def hammingDistance(self, x, y):
        bin_x = self.decToBin(x)
        bin_y = self.decToBin(y)
        if(x<y):
            length = len(bin_y)
            while(len(bin_x) < length):
                bin_x = '0' + bin_x
        else:
            length = len(bin_x)
            while(len(bin_y) < length):
                bin_y = '0' + bin_y
        hammer_dist = 0
        for i in range(length):
            if(bin_x[i] != bin_y[i]):
                hammer_dist += 1
        return hammer_dist
        

    def decToBin(self, dec_num):
        bin_to_rtr = ''
        while(dec_num > 0):
            if(dec_num%2 == 0):
                bin_to_rtr = '0' + bin_to_rtr
            else:
                bin_to_rtr = '1' + bin_to_rtr
            dec_num = math.floor(dec_num/2)
        return bin_to_rtr
        


a = Solution()
a.hammingDistance(4, 14)