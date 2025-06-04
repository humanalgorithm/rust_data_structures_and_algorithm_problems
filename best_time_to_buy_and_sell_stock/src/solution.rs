use std::cmp;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut profit = 0;
    for i in 0..prices.len() {
        min_price = cmp::min(prices[i], min_price);
        profit = cmp::max(prices[i] - min_price, profit);
    }
    return profit;
}
