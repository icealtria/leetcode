class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        intervals.sort(key = lambda x: x[1])
        res = 0
        lastend = intervals[0][1]
        for start, end in intervals[1:]:
            if start >= lastend:
                lastend = end
            else:
                res += 1
        return res