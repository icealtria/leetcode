impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut res = -1;
        let mut min_dis = i32::MAX;
        for (i, point) in points.iter().enumerate() {
            if point[0] == x || point[1] == y {
                let dis = (point[0] - x).abs() + (point[1] - y).abs();
                if dis < min_dis {
                    min_dis = dis;
                    res = i as i32;
                }
            }
        }
        res
    }
}
