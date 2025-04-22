func trap(height []int) int {
	n := len(height)
	if n < 3 {
		return 0
	}

	left := 0
	right := n - 1
	leftMax := 0
	rightMax := 0
	res := 0

	for left < right {
		if height[left] < height[right] {
			if height[left] >= leftMax {
				leftMax = height[left]
			} else {
				res += leftMax - height[left]
			}
			left++
		} else {
			if height[right] >= rightMax {
				rightMax = height[right]
			} else {
				res += rightMax - height[right]
			}
			right--
		}
	}
	return res
}