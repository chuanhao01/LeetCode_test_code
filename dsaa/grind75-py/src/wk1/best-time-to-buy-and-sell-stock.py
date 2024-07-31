class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        diff_so_far = 0
        l = prices[0]
        for i in range(1, len(prices)):
            r = prices[i]
            if r - l < 0:
                l = r
            else:
                diff_so_far = max(r - l, diff_so_far)
        return diff_so_far
