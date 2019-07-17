class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        self.check = True
        self.counter = 0
        while(self.check):
            if (self.counter < len(nums)):
                if (nums[self.counter] == val):
