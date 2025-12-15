impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut result = 1_i64;
        let mut descent_period = 1_i64;

        for id in 1 .. prices.len() {
            descent_period = (((prices[id - 1] - 1 == prices[id]) as i64 * -1) & descent_period) + 1;
            result += descent_period;
        }

        result
    }
}