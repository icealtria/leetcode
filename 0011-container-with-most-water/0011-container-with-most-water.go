func maxArea(height []int) int {
	max, l, r := 0, 0, len(height)-1
	for l < r {
		area := (r - l) * min(height[l], height[r])
		if area > max {
			max = area
		}
		if height[l] < height[r] {
			l++
		} else {
			r--
		}
	}
	return max
}