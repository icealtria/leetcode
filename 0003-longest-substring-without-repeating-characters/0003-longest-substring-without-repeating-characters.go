func lengthOfLongestSubstring(s string) int {
	set := map[byte]bool{}
	l := 0
	res := 0

	for r := 0; r < len(s); r++ {
		for set[s[r]] {
			delete(set, s[l])
			l++
		}
		set[s[r]] = true
		res = max(res, r-l+1)
	}
	return res
}