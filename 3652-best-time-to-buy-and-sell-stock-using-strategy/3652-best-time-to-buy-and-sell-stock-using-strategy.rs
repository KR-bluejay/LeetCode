impl Solution {
    pub fn max_profit(
        prices: Vec<i32>, 
        strategies: Vec<i32>, 
        k: i32
    ) -> i64 {
        let k = k as usize;

        let mut base_profit: Vec<i64> = Vec::with_capacity(prices.len() + 1);
        let mut holding_profit: Vec<i64> = vec![0; prices.len() + 1];
        let mut selling_profit: Vec<i64> = Vec::with_capacity(prices.len() + 1);

        selling_profit.push(0);
        base_profit.push(0);


        for id in 1 ..= prices.len()  {
            base_profit.push(base_profit[id - 1] + (prices[id - 1] * strategies[id - 1]) as i64);
            selling_profit.push(selling_profit[id - 1] + prices[id - 1] as i64);
        }


        let mut base: i64 = *base_profit.last().unwrap();
        let mut delta = 0;
        let mut prefix_len = 1;
        let mut prefix_dir = 0;


        for id in 1 .. strategies.len() {
            prefix_len += 1;

            if prefix_len < k {
                continue;
            }

            prefix_len = k;

            let first_id = (id + 1 - prefix_len);
            let first_id_2 = (id + 1 - (prefix_len / 2)) - 1;
            let second_id = (id + 1 - (prefix_len / 2));
            let second_id_2 = id;

            let delta_1 = holding_profit[first_id_2 + 1] - holding_profit[first_id];
            let delta_2 = selling_profit[second_id_2 + 1] - selling_profit[second_id];

            let base_delta = base_profit[id + 1] - base_profit[first_id];

            // println!("{} {}", selling_profit[second_id_2 + 1], selling_profit[second_id]);
            // println!("{id} {delta_1} {delta_2} {base_delta}");

            delta = delta.max(delta_1 + delta_2 - base_delta);

        }


        base + delta
    }
}