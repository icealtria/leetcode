use std::cmp;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut high, mut low) = (nums.len(), 0);
        let mut idx;
        
        if target > nums[high - 1] {
            return high as i32;
        }
        if target < nums[0] {
            return 0;
        }
        
        while low <= high {
            idx = (low + high) / 2;
            match nums[idx].cmp(&target) {
                cmp::Ordering::Equal => return idx as i32,
                cmp::Ordering::Less => low = idx + 1,
                cmp::Ordering::Greater => high = idx - 1,
            }
        }
        low as i32
    }
}