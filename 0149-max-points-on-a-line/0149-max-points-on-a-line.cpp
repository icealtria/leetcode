class Solution {
public:
    int maxPoints(vector<vector<int>>& points) {
        int max_points = 0;
        for (int i = 0; i < points.size(); i++) {
            int same_x_points = 1;
            unordered_map<double, int> slope_count;
            for (int j = 0; j < points.size(); j++) {
                if (i == j) continue;
                vector<int> point1 = points[i];
                vector<int> point2 = points[j];
                if (point1[0] == point2[0]) {
                    same_x_points++;
                } else {
                    double slope = (double)(point1[1] - point2[1]) / (double)(point1[0] - point2[0]);
                    slope_count[slope]++;
                }
            }
            max_points = max(max_points, same_x_points);
            for (auto count : slope_count) {
                max_points = max(max_points, count.second + 1);
            }
        }
        return max_points;
    }
};