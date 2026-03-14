impl Solution {
    fn get_costs(
        cache: &mut Vec<Vec<i32>>,
        coins: &Vec<i32>,
        left_id: usize, 
        right_id: usize
    ) -> i32 {
        if left_id > right_id 
        || left_id >= coins.len() 
        || right_id >= coins.len() {
            return 0;
        }

        if cache[left_id][right_id] > 0 {
            return cache[left_id][right_id];
        }


        for mid_id in left_id ..= right_id {
            let mut coin = *coins.get(left_id - 1).unwrap_or(&0)
                * coins[mid_id] 
                * *coins.get(right_id + 1).unwrap_or(&1);
            coin += Self::get_costs(cache, coins, left_id, mid_id - 1);
            coin += Self::get_costs(cache, coins, mid_id + 1, right_id);

            cache[left_id][right_id] = cache[left_id][right_id].max(coin);
        }

        cache[left_id][right_id]
    }
    pub fn max_coins(mut coins: Vec<i32>) -> i32 {
        coins.insert(0, 1);
        coins.insert(coins.len(), 1);

        let mut cache: Vec<Vec<i32>> = vec![vec![0; coins.len()]; coins.len()];
        
        Self::get_costs(&mut cache, &coins, 1, coins.len() - 2)
    }
}