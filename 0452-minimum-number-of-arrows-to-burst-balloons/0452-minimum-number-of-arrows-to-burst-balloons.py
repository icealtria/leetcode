class Solution:
    def findMinArrowShots(self, points: List[List[int]]) -> int:
        points.sort(key = lambda x:x[1])
        res = 1
        lastend = points[0][1]
        for start, end in points:
            if start > lastend:
                res += 1
                lastend = end
        return res