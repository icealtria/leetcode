impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let n = nums.len();
        let mut left = -1;
        let (mut maxn, mut minn) = (-1, -1);
        let mut res:i64 = 0;
        for i in 0..n {
            let num = nums[i];
            if num >= min_k && num <= max_k{
                if num == min_k {minn = i as i32;}
                if num == max_k {maxn = i as i32;}
                res += 0.max(minn.min(maxn) - left) as i64;
            }
            else{
                left = i as i32;
                minn = -1;
                maxn = -1;
            }
        }
        res
    }
}
