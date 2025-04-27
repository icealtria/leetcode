func reverseWords(s string) string {
	arr := strings.Fields(s)
	slices.Reverse(arr)
	return strings.Join(arr, " ")
}
