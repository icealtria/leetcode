class Solution:
    def countSubTrees(self, n: int, edges: List[List[int]], labels: str) -> List[int]:
        def dfs(root, parent):
            count = Counter()
            for child in tree[root]:
                if child != parent:
                    count += dfs(child, root)
            count[labels[root]] += 1
            res[root] += count[labels[root]]
            return count
        
        tree = defaultdict(list)
        for x, y in edges:
            tree[x].append(y)
            tree[y].append(x)
        res = [0] * n
        dfs(0, None)

        return res