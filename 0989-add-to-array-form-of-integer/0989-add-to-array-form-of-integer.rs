impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut rem = 0;
        num.reverse();
        for n in num.iter_mut(){
            *n += k % 10 + rem;
            rem = *n / 10;
            *n %= 10;
            k /= 10;
        }
        while k > 0 {
            let val = k % 10 + rem;
            num.push(val % 10);
            rem = val/10;
            k /= 10;
        }
        
        if rem != 0 { num.push(rem); }
        num.reverse();
        num
    }
}