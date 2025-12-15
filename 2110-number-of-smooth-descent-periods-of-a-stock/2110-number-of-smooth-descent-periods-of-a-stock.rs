impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut left_id = 0;
        let mut result = 0;

        while left_id < prices.len() {
            let mut right_id = left_id + 1;

            while right_id < prices.len() && prices[right_id - 1] - 1 == prices[right_id] {
                right_id += 1;
            }
            result += (right_id - left_id) * (right_id - left_id + 1) / 2;


            left_id = right_id;
        }

        result as i64
    }
}