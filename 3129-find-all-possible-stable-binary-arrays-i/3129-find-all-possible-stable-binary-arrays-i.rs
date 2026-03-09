impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const MODULO: u32 = 1_000_000_007;

        let (zero, one, limit) = (zero as usize, one as usize, limit as usize);
        let mut cache: Vec<Vec<Vec<u32>>> 
            = vec![vec![vec![0; 2]; one + 1]; zero + 1];

        for zero_id in 1 ..= zero.min(limit) {
            cache[zero_id][0][0] = 1;
        }

        for one_id in 1 ..= one.min(limit) {
            cache[0][one_id][1] = 1;
        }

        for zero_id in 1 ..= zero {
            for one_id in 1 ..= one {
                cache[zero_id][one_id][0] 
                    = cache[zero_id - 1][one_id][0] 
                    + cache[zero_id - 1][one_id][1] 
                    + MODULO;

                cache[zero_id][one_id][0] -= if zero_id > limit {
                    cache[zero_id - limit - 1][one_id][1] 
                } else {
                    0
                };
                cache[zero_id][one_id][0] %= MODULO;

                
                cache[zero_id][one_id][1] 
                    = cache[zero_id][one_id - 1][0] 
                    + cache[zero_id][one_id - 1][1] 
                    + MODULO;
                
                cache[zero_id][one_id][1] -= if one_id > limit {
                    cache[zero_id][one_id - limit - 1][0]
                } else {
                    0
                };
                cache[zero_id][one_id][1] %= MODULO;
            }
        }

        ((cache[zero][one][0] + cache[zero][one][1]) % MODULO) as i32
    }
}