impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut buy_at_price = i32::MAX;
        let mut max_profit = 0;
        for i in 0..n {
            buy_at_price = buy_at_price.min(prices[i]);
            max_profit = max_profit.max(prices[i] - buy_at_price);
        }
        max_profit
    }
}
