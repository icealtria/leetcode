func lengthOfLastWord(s string) int {
	trimed := strings.Trim(s, " ")
	arr := strings.Split(trimed, " ")
	return len(arr[len(arr)-1])
}