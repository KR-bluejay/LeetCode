impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let mut cache = vec![vec![i32::MIN / 2; 3]; coins[0].len() + 1];

        cache[1][0] = 0;
        cache[1][1] = 0;
        cache[1][2] = 0;

        for row_id in 0 .. coins.len() {
            for col_id in 1 ..= coins[0].len() {
                let cur_coin = coins[row_id][col_id - 1];

                cache[col_id][2] = *[
                    cache[col_id - 1][2] + cur_coin, 
                    cache[col_id][2] + cur_coin, 
                    cache[col_id - 1][1], 
                    cache[col_id][1], 
                ].iter().max().unwrap();
                
                cache[col_id][1] = *[
                    cache[col_id - 1][1] + cur_coin, 
                    cache[col_id][1] + cur_coin, 
                    cache[col_id - 1][0], 
                    cache[col_id][0], 
                ].iter().max().unwrap();
                
                cache[col_id][0] = *[
                    cache[col_id - 1][0] + cur_coin, 
                    cache[col_id][0] + cur_coin, 
                ].iter().max().unwrap();
            }
        }

        *cache.last().unwrap().iter().max().unwrap()
    }
}