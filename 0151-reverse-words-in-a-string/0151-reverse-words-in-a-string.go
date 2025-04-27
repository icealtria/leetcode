func reverseWords(s string) string {
	s = strings.Trim(s, " ")
	arr := strings.Fields(s)
	slices.Reverse(arr)
	return strings.Join(arr, " ")
}
