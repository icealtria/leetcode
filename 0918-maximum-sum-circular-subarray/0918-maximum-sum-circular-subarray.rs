impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut max, mut min, mut total) = (i32::MIN, i32::MAX, 0);
        let (mut curr_max, mut curr_min) = (0, 0);

        for &num in nums.iter() {
            curr_min = num.min(curr_min + num);
            min = min.min(curr_min);
            curr_max = num.max(curr_max + num);
            max = max.max(curr_max);
            total += num;
        }
        if max > 0 {
            max.max(total - min)
        } else {
            max
        }
    }
}