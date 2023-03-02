impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        if n == 1 {
            return 1;
        }
        let (mut i, mut j) = (0, 0);

        while i < n {
            let mut count = 1;
            while i < n -1 && chars[i] == chars[i+1] {
                count += 1;
                i += 1;
            }
            chars[j] = chars[i];
            j += 1;
            if count > 1 {
                for c in count.to_string().chars() {
                    chars[j] = c;
                    j += 1;
                }
            }
            i += 1;            
        }
        j as i32
    }
}