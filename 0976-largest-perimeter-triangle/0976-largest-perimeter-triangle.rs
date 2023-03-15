impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by_key(|n| -n);
        nums.windows(3)
            .find(|w| w[0] < w[1] + w[2])
            .map(|w| w[0] + w[1] + w[2])
            .unwrap_or(0)
    }
}