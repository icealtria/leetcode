class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        max_points = 0
        for i in range(len(points)):
            same_x_points = 1
            slope_count = {}
            for j in range(len(points)):
                if i == j:
                    continue
                point1 = points[i]
                point2 = points[j]
                if point1[0] == point2[0]:
                    same_x_points += 1
                else:
                    slope = (point1[1] - point2[1]) / (point1[0] - point2[0])
                    if slope not in slope_count:
                        slope_count[slope] = 2
                    else:
                        slope_count[slope] += 1
            max_points = max(max_points, same_x_points)
            for count in slope_count.values():
                max_points = max(max_points, count)
        return max_points