class Solution:
    def minMaxGame(self, nums: List[int]) -> int:
        if len(nums) == 1:
            return nums[0]
        status = 0
        new_nums = []
        for x, y in zip(*[iter(nums)]*2):
            if status == 0:
                new_nums.append(min(x, y))
            else:
                new_nums.append(max(x, y))
            status = 1 - status
        return self.minMaxGame(new_nums)