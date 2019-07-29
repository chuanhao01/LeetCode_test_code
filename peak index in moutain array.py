class Solution:
    def peakIndexInMountainArray(self, A):
        index_to_rtr = 0
        temp = 0
        for i in range(len(A)):
            if(A[i] > temp):
                index_to_rtr = i
                temp = A[i]
        return index_to_rtr
        