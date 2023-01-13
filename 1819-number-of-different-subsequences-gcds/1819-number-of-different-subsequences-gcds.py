class Solution:
    def countDifferentSubsequenceGCDs(self, nums: List[int]) -> int:
        T, nums, ans = max(nums) + 1, set(nums), 0
        for x in range(1, T):
            g = 0
            for y in range(x, T, x):
                if y in nums:
                    g = gcd(g, y)
                if g == x:
                    break
            if g == x:
                ans += 1
        return ans