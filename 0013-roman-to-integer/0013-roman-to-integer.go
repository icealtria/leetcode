func romanToInt(s string) int {
	m := map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}
	res := 0
	prev := 0
	for i := len(s) - 1; i >= 0; i-- {
		curr := m[s[i]]
		if curr < prev {
			res -= curr
		} else {
			res += curr
		}
		prev = curr
	}

	return res
}
