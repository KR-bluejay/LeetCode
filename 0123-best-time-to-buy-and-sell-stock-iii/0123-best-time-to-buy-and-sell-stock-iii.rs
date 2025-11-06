impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy_1 = i32::MIN;
        let mut sell_2 = i32::MIN / 2;
        let mut buy_3 = i32::MIN / 2;
        let mut sell_4 = i32::MIN / 2;

        for &price in prices.iter() {
            let cur_buy_1 = buy_1.max(0 - price);
            let cur_sell_2 = sell_2.max(buy_1 + price);
            let cur_buy_3 = buy_3.max(sell_2 - price);
            let cur_sell_4 = sell_4.max(buy_3 + price);

            buy_1 = cur_buy_1;
            sell_2 = cur_sell_2;
            buy_3 = cur_buy_3;
            sell_4 = cur_sell_4;
        }


        buy_1.max(sell_2).max(buy_3).max(sell_4).max(0)
    }
}