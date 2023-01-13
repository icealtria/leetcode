class Solution:
    def longestPath(self, parent: List[int], s: str) -> int:
        def dfs(node):
            res = 1
            for child in d[node]:
               length  = dfs(child)
               if s[child] != s[node]:
                   self.ans = max(self.ans, res + length)
                   res = max(res, length + 1)
            return res
        
        self.ans = 1
        d = defaultdict(list)
        for i, k in enumerate(parent[1:], 1):
            d[k].append(i)
        dfs(0)
        return self.ans
