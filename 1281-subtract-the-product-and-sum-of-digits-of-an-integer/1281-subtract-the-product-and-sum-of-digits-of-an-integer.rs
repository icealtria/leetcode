impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let sum:u32 = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).sum();
        let product = n
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .reduce(|a, b| a*b).unwrap();
          product as i32 - sum as i32
    }
}
