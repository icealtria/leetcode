class Solution:
    def minimumFuelCost(self, roads: List[List[int]], seats: int) -> int:
        self.res = 0
        adj = defaultdict(list)
        for start, end in roads:
            adj[start].append(end)
            adj[end].append(start)

        def dfs(node, parent):
            accum = 1
            for n in adj[node]:
                if n == parent:
                    continue
                rep = dfs(n, node)
                self.res += (rep + seats - 1) // seats
                accum += rep
            return accum

        dfs(0, -1)
        return self.res