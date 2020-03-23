class Solution:
    def findDuplicate(self, nums):
        calculated_sum = (len(nums) * (len(nums) - 1) / 2)
        actual_sum = 0
        for i in nums:
            actual_sum += i
        return int(actual_sum - calculated_sum)

a = Solution()
test = [1,3,4,2,2]
b = a.findDuplicate(test)
print(b)