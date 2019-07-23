class Solution:
    def sortArrayByParity(self, A):
        new_arr = []
        total_length = len(A) - 1
        while(total_length > -1):
            if(A[total_length]%2 == 0):
                new_arr.append(A.pop(total_length))
            total_length -= 1
        new_arr += A
        return new_arr