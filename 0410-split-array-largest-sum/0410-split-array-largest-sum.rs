impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let (mut l, mut r) = (0, 0);
        for x in nums.iter() {
            l = l.max(*x);
            r += x;
        }
        
        while l < r {
            let mid = (l + r) / 2;
            let (mut tot, mut cnt) = (0, 1);
            for num in nums.iter() {
                tot += num;
                if tot > mid {
                    tot = *num;
                    cnt += 1;
                }
            }
            if cnt > m {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        r
    }
}
