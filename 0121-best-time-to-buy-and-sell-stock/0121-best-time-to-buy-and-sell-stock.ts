function maxProfit(prices: number[]): number {
    let n = prices.length;
    let minPrice = prices[0];
    let maxProfit = 0;
    for (let i = 1; i < n; i++) {
        if (prices[i] < minPrice) {
            minPrice = prices[i];
        } else if (prices[i] - minPrice > maxProfit) {
            maxProfit = prices[i] - minPrice;
        }
    }

    return maxProfit;
};