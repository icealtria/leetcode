class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        ans = []
        for i in range(len(intervals)):
            start = intervals[i][0]
            end = intervals[i][1]
            if newInterval[1] < start:
                ans.append(newInterval)
                return ans + intervals[i:]
            elif newInterval[0] > end:
                ans.append(intervals[i])
            else:
                newInterval = [min(newInterval[0], start),
                               max(newInterval[1], end)]
        ans.append(newInterval)
        return ans