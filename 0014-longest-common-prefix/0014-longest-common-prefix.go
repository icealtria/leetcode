func longestCommonPrefix(strs []string) string {
	n := len(strs)
	if n == 0 {
		return ""
	}
	sort.Strings(strs)
	for i := range strs[0] {
		if strs[0][i] != strs[n-1][i] {
			return strs[0][:i]
		}
	}
	return strs[0]
}