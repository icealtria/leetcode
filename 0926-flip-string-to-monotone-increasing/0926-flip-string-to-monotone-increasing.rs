impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let (mut cnt1, mut cntf) = (0, 0);
        for x in s.chars() {
            if x == '1' {
                cnt1 += 1;
            } else {
                cntf += 1;
            }
            cntf = cntf.min(cnt1);
        }
        cntf
    }
}