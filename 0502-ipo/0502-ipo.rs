use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;
        let mut projects = Vec::new();
        for (&p, &c) in profits.iter().zip(capital.iter()) {
            projects.push((p, c));
        }
        projects.sort_by(|a, b| a.1.cmp(&b.1));

        let mut heaq: BinaryHeap<i32> = BinaryHeap::new();
        let mut curr = 0;
        for _ in 0..k {

            while let Some((p, _)) = projects.get(curr).filter(|(p, c)| c <= &w) {
                heaq.push(*p);
                curr += 1;
            }

            if let Some(p) = heaq.pop() {
                w += p;
            }
        }
        w
    }
}
