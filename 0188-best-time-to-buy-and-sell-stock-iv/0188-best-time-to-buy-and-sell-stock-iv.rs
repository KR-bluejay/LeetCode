impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut cache: Vec<i32> = vec![-10000; k * 2];
        let mut max_price = -10000;

        for price in prices {
            for cache_id in (1 .. cache.len()).rev() {
                let cur_price = if cache_id % 2 == 0 {
                    cache[cache_id - 1] - price
                } else {
                    cache[cache_id - 1] + price
                };
                max_price = max_price.max(cur_price);

                cache[cache_id] = cache[cache_id].max(cur_price);
            }
            cache[0] = cache[0].max(-price);
        }

        max_price.max(0)
    }
}