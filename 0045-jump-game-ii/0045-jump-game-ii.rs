impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut jumps = 0;
        let mut i = 0; // Current position
        let mut explored = 0; // The highest index we have explored
        while explored < n - 1 {
            // Choose where to jump based on how far the next position can take us
            let to_explore = (n-1).min(i + nums[i] as usize);
            i = (explored + 1..=to_explore)
                .max_by(|&j, &k| (j + nums[j] as usize).cmp(&(k + nums[k] as usize)))
                .unwrap();
            explored = to_explore;
            jumps += 1;
        }
        jumps
    }
}