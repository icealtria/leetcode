class Solution:
    def shortestAlternatingPaths(self, n: int, redEdges: List[List[int]], blueEdges: List[List[int]]) -> List[int]:
        graph = [[[], []] for i in range(n)]

        for start, end in redEdges: 
            graph[start][0].append(end)
        for start, end in blueEdges: 
            graph[start][1].append(end)
        
        result = [[0, 0]] + [[n * 2, n * 2] for i in range(n - 1)]
        
        # Initialize the bfs queue with starting node 0 and color 0 and 1
        bfs = [[0, 0], [0, 1]]
        
        # Start the BFS traversal
        for node, color in bfs:
            for next_node in graph[node][color]:
                if result[next_node][color] == n * 2:
                    result[next_node][color] = result[node][1 - color] + 1
                    bfs.append([next_node, 1 - color])
        
        # Return the minimum distance between node 0 and all other nodes, 
        # or -1 if not possible
        return [x if x < n * 2 else -1 for x in map(min, result)]