impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums:Vec<i32> = nums.iter().map(|x| x * x).collect();
        nums.sort();
        nums
    }
}
