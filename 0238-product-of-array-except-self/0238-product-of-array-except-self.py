class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)

        result = [1] * n
        
        for i in range(1, n):
            result[i] = result[i - 1] * nums[i - 1]
        
        right = 1

        for i in range(n - 1, -1, -1):
            result[i] *= right
            right *= nums[i]
        
        return result