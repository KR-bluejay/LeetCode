impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut tx_hold: Vec<i32> = vec![0; prices.len()];
        let mut tx_sell: Vec<i32> = vec![0; prices.len()];

        tx_hold[0] = prices[0] * -1;
        tx_sell[0] = 0;

        for i in 1 .. prices.len() {
            tx_hold[i] = tx_hold[i - 1]
                .max(tx_sell[i - 1] - prices[i]);
            tx_sell[i] = tx_sell[i - 1]
                .max(tx_hold[i - 1] + prices[i] - fee);
        }

        *tx_sell.last().unwrap()
    }
}