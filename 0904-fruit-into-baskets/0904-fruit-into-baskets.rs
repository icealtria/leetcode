use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let (mut i, mut j) = (0, 0);

        while j < fruits.len() {
            *mp.entry(fruits[j]).or_insert(0) += 1;

            if mp.len() > 2 {
                *mp.get_mut(&fruits[i]).unwrap() -= 1;

                if *mp.get(&fruits[i]).unwrap() == 0 {
                    mp.remove(&fruits[i]);
                }

                i += 1;
            } else {
                result = result.max(mp.values().sum());
            }
            j += 1;
        }
        result
    }
}