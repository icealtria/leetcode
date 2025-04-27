func isPalindrome(s string) bool {
	var cleaned strings.Builder
	for _, char := range strings.ToLower(s) {
		if unicode.IsLetter(char) || unicode.IsDigit(char) {
			cleaned.WriteRune(char)
		}
	}
	new := cleaned.String()
	return reverse(new) == new
}

func reverse(s string) string {
	arr := strings.Split(s, "")
	slices.Reverse(arr)
	return strings.Join(arr, "")
}