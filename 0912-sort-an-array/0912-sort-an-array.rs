impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        fn merge(nums: &[i32]) -> Vec<i32> {
            let n = nums.len();
            if n < 2 {
                return nums.to_vec();
            }
            let mid = n / 2;
            let left = merge(&nums[..mid]);
            let right = merge(&nums[mid..]);
            let mut left_it = left.into_iter().peekable();
            let mut right_it = right.into_iter().peekable();
            let mut rez = vec![0; n];
            let mut rez_it = rez.iter_mut();
            while let (Some(&l), Some(&r)) = (left_it.peek(), right_it.peek()) {
                *rez_it.next().unwrap() = if l < r {
                    left_it.next();
                    l
                } else {
                    right_it.next();
                    r
                };
            }
            for l in left_it {
                *rez_it.next().unwrap() = l;
            }
            for r in right_it {
                *rez_it.next().unwrap() = r;
            }
            rez
        }
        merge(&nums)
    }
}

