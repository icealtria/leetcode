func countSubarrays(nums []int) int {
	res := 0
	for i := 1; i < len(nums)-1; i++ {
		if nums[i] == (nums[i-1]+nums[i+1])*2 {
			res++
		}
	}
	return res
}