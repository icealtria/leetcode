use std::collections::HashSet;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut groups: Vec<HashSet<String>> = vec![HashSet::new(); 26];
        let mut pairs: i64 = 0;

        ideas.into_iter().for_each(|idea| {
            groups[(idea.as_bytes()[0] - b'a') as usize].insert(idea[1..].to_string());
        });

        for i in 0..25 {
            for j in (i + 1)..26 {
                let dup = groups[i].iter().filter(|&s| groups[j].contains(s)).count();
                pairs += 2 * ((groups[i].len() - dup) * (groups[j].len() - dup)) as i64;
            }
        }
        pairs
    }
}