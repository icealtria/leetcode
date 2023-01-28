class SummaryRanges:

    def __init__(self):
        self.s = set()

    def addNum(self, value: int) -> None:
        self.s.add(value)

    def getIntervals(self) -> list[list[int]]:
        nums = sorted(list(self.s))
        intervals = []
        start = end = nums[0]
        for n in nums[1:]:
            if n == end + 1:
                end = n
            else:
                intervals.append([start, end])
                start = end = n
        intervals.append([start, end])
        return intervals

# Your SummaryRanges object will be instantiated and called as such:
# obj = SummaryRanges()
# obj.addNum(value)
# param_2 = obj.getIntervals()