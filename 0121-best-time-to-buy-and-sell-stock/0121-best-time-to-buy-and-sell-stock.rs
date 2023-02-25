impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut max_prices = vec![0; n];
        max_prices[n-1] = prices[n-1];
        for i in (0..n-1).rev() {
            max_prices[i] = max_prices[i+1].max(prices[i]); 
        }
        let mut max_profit = 0;
        for i in 0..n {
            max_profit = max_profit.max(max_prices[i] - prices[i]); 
        }
        max_profit
    }
}