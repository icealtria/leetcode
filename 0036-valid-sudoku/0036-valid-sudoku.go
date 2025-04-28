func isValidSudoku(board [][]byte) bool {
	for i := 0; i < 9; i++ {
		row := make(map[byte]bool)
		col := make(map[byte]bool)
		for j := 0; j < 9; j++ {
			if board[i][j] != '.' {
				if row[board[i][j]] {
					return false
				}
				row[board[i][j]] = true
			}
			if board[j][i] != '.' {
				if col[board[j][i]] {
					return false
				}
				col[board[j][i]] = true
			}
		}
	}
	for i := 0; i < 9; i += 3 {
		for j := 0; j < 9; j += 3 {
			box := make(map[byte]bool)
			for k := i; k < i+3; k++ {
				for l := j; l < j+3; l++ {
					if board[k][l] != '.' {
						if box[board[k][l]] {
							return false
						}
						box[board[k][l]] = true
					}
				}
			}
		}
	}
	return true
}