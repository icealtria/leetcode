impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let m = needle.len();
        let n = haystack.len();
        if m < 1 {return 0;}
        if m > n {return -1;}

        let needle = needle.as_bytes();
        let haystack = haystack.as_bytes();

        for window_start in 0..(n - m + 1) {
            for i in 0..m {
                if needle[i] != haystack[window_start + i] {
                    break;
                }
                if i == m - 1 {
                    return window_start as i32;
                }
            }
        }
        -1
    }
}
