func spiralOrder(matrix [][]int) []int {
	direct := [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	m, n := len(matrix), len(matrix[0])
	res := make([]int, 0, m*n)
	visited := make([][]bool, m)
	for i := 0; i < m; i++ {
		visited[i] = make([]bool, n)
	}
	x, y, d := 0, 0, 0
	for len(res) < m*n {
		res = append(res, matrix[x][y])
		visited[x][y] = true
		nx, ny := x+direct[d][0], y+direct[d][1]
		if nx < 0 || nx >= m || ny < 0 || ny >= n || visited[nx][ny] {
			d = (d + 1) % 4
			nx, ny = x+direct[d][0], y+direct[d][1]
		}
		x, y = nx, ny
	}
	return res
}