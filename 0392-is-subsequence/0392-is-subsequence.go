func isSubsequence(s string, t string) bool {
	idx, jdx := 0, 0
	for idx < len(s) && jdx < len(t) {
		if s[idx] == t[jdx] {
			idx++
			jdx++
		} else {
			jdx++
		}
	}
	return idx == len(s)
}