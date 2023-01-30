impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split('|')
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .flat_map(|(_, s)| s.chars())
            .filter(|&c| c == '*')
            .count() as _
    }
}