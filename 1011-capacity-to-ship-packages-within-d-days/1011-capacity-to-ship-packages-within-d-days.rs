impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let (mut l, mut r) = (0, 0);
        for w in weights.iter() {
            l = l.max(*w);
            r += w;
        }

        while l < r {
            let mut mid = (l + r) / 2;
            match Self::feasible(&weights, days, mid) {
                true => r = mid,
                false => l =  mid + 1,
            }
        }
        l
    }
    
    pub fn feasible(weights: &Vec<i32>, days: i32, capacity: i32) -> bool {
        let mut t = 0;
        let mut d = 1;
        for x in weights {
            t += x;
            if t > capacity {
                t = *x;
                d += 1;
            }
        }
        d <= days
    }
}