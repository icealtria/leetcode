func wordPattern(pattern string, s string) bool {
	words := strings.Split(s, " ")
	if len(pattern) != len(words) {
		return false
	}
	m := make(map[byte]string)
	for i := 0; i < len(pattern); i++ {
		if _, ok := m[pattern[i]]; !ok {
			for _, v := range m {
				fmt.Println(v, words[i])
				if v == words[i] {
					return false
				}
			}
			m[pattern[i]] = words[i]
		} else {
			if m[pattern[i]] != words[i] {
				return false
			}
		}
	}
	return true
}
