use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut m = i32::MAX;
        for &num in nums.iter() {
            if num % 2 == 0 {
                pq.push(num);
                m = m.min(num);
            } else {
                pq.push(num * 2);
                m = m.min(num * 2);
            }
        }

        let mut ans = i32::MAX;

        while !pq.is_empty() {
            let top = pq.pop().unwrap();
            ans = ans.min(top - m);
            if top % 2 != 0 {
                break;
            }
            m = m.min(top / 2);
            pq.push(top / 2);
        }
        ans
    }
}
