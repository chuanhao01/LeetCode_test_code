class Solution:
    def sortedSquares(self, A):
        negative_arr = []
        counter = 0
        while(counter < len(A)):
            if(A[counter] > -1):
                break
            else:
                negative_arr.append(-(A.pop(counter)))
                counter -= 1
            counter += 1
        if(len(A) == 0):
            negative_arr = negative_arr[::-1]
            for i in range(len(negative_arr)):
                negative_arr[i] = negative_arr[i]**2
            return negative_arr
        else:
            negative_length = 0
            postive_length = len(A) - 1
            while(postive_length > -1 and negative_length < len(negative_arr)):
                if(negative_arr[negative_length] > A[postive_length]):
                    A.insert((postive_length + 1), negative_arr[negative_length])
                    negative_length += 1
                elif(negative_arr[negative_length] == A[postive_length]):
                    A.insert((postive_length + 1), negative_arr[negative_length])
                    negative_length += 1
                else:
                    postive_length -= 1
                # print(A)
            if(postive_length == -1):
                while(negative_length < len(negative_arr)):
                    A.insert(0, negative_arr[negative_length])
                    negative_length += 1
        for i in range(len(A)):
            A[i] = A[i]**2
        return A

a = Solution()
a.sortedSquares([-1])

