impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy_1 = -100000;
        let mut sell_2 = -100000;
        let mut buy_3 = -100000;
        let mut sell_4 = -100000;

        for &price in prices.iter() {
            sell_4 = sell_4.max(buy_3 + price);
            buy_3 = buy_3.max(sell_2 - price);
            sell_2 = sell_2.max(buy_1 + price);
            buy_1 = buy_1.max(0 - price);
        }


        buy_1.max(sell_2).max(buy_3).max(sell_4).max(0)
    }
}