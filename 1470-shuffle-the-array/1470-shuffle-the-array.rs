impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        nums[..n as usize]
        .iter()
        .zip(nums[n as usize..].iter())
        .flat_map(|(&x, &y)| vec![x, y])
        .collect()
    }
}