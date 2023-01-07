impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let (mut total, mut start, mut remain) = (0, 0, 0);
        for i in (0..len) {
            total = total + gas[i] - cost[i];
            remain = remain + gas[i] - cost[i];
            if remain < 0 {
                remain = 0;
                start = i + 1;
            }
        }
        if total < 0{
            -1
        }else{
            start as i32
        }
    }
}