use std::collections::{BTreeMap, VecDeque};

struct MKAverage {
    m: usize,
    k: usize,
    deque: VecDeque<i32>,
    sort: BTreeMap<i32, i32>,
    sum: i32,
}


/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        MKAverage {
            m: m as usize,
            k: k as usize,
            deque: VecDeque::new(),
            sort: BTreeMap::new(),
            sum: 0,
        }
    }

    fn add_element(&mut self, num: i32) {
        self.deque.push_back(num);
        *self.sort.entry(num).or_insert(0) += 1;
        self.sum += num;
        if self.deque.len() > self.m {
            let tmp = self.deque.pop_front().unwrap();
            if let Some(v) = self.sort.get_mut(&tmp) {
                if *v == 1 {
                    self.sort.remove(&tmp);
                } else { *v -= 1; }
            }

            self.sum -= tmp
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.deque.len() < self.m { return -1; }

        let mut k_sum: i32 = 0;

        let mut k_times = self.k;
        'label: for (num, count) in self.sort.iter().take(self.k) {
            for _ in 0..*count {
                k_sum += num;
                k_times -= 1;
                if k_times < 1 { break 'label; }
            }
        }

        k_times = self.k;
        'label2: for (num, count) in self.sort.iter().rev().take(self.k) {
            for _ in 0..*count {
                k_sum += num;
                k_times -= 1;
                if k_times < 1 { break 'label2; }
            }
        }


        (self.sum - k_sum) / (self.m - 2 * self.k) as i32
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */