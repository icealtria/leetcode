class Solution:
    def minTime(self, n: int, edges: List[List[int]], hasApple: List[bool]) -> int:
        tree = defaultdict(list)
        visited = [False for _ in range(n)]
        for x, y in edges:
            tree[x].append(y)
            tree[y].append(x)

        def dfs(root):
            if visited[root]:
                return 0
            visited[root] = True
            time = 0
            for child in tree[root]:
                time += dfs(child)
            if time or hasApple[root]:
                time += 2
            return time
        return max(dfs(0)-2, 0)