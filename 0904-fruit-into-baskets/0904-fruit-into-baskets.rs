use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut prev = -1;
        let mut prev_length = 0;
        let mut prev_prev = -1;
        let mut cur_max = 0;
        let mut res = 0;
        for fruit in fruits {
            if fruit == prev {
                cur_max += 1;
                prev_length += 1;
            } else if fruit == prev_prev {
                std::mem::swap(&mut prev, &mut prev_prev);
                prev_length = 1;
                cur_max += 1
            } else {
                prev_prev = prev;
                prev = fruit;
                cur_max = prev_length + 1;
                prev_length = 1;
            }
            res = res.max(cur_max)
        }
        res
    }
}