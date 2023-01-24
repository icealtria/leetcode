class Solution:
    def snakesAndLadders(self, board: List[List[int]]) -> int:
        destination = len(board)**2
        board.reverse()
        for i in range(1, len(board), 2): board[i].reverse()
        cells = [None]+list(chain(*board))

        queue, seen, ct = deque([1]), {1}, 0

        while queue:
            lenQ = len(queue)
            for _ in range(lenQ):
                cur = queue.popleft()
                if cur == destination:
                    return ct
                for i in range(cur+1, min(cur+7, destination+1)):
                    nxt = cells[i] if cells[i]+1 else i
                    if nxt in seen: continue
                    seen.add(nxt)
                    queue.append(nxt)
            ct += 1

        return -1