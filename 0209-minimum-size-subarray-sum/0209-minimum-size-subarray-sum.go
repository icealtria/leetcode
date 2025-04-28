func minSubArrayLen(target int, nums []int) int {
	res := math.MaxInt
	l := 0
	sum := 0
	for r := range nums {
		sum += nums[r]
		for sum >= target {
			res = min(res, r-l+1)
			sum -= nums[l]
			l++
		}
	}
	if res == math.MaxInt {
		return 0
	}
	return res
}