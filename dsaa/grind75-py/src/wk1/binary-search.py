from typing import List
class Solution:
    def search(self, nums: List[int], target: int) -> int:
        l = 0
        r = len(nums)
        while l < r:
            i = (l + r) // 2
            if nums[i] == target:
                return i
            elif nums[i] < target:
                l = i
            else:
                r = i
        return -1
