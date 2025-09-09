use std::collections::HashMap;

impl Solution {
    pub fn people_aware_of_secret(mut n: i32, mut delay: i32, mut forget: i32) -> i32 {
        let mut secret_aware_map: HashMap<i32, i32> = HashMap::with_capacity(n as usize);
        let mut total_aware_count: i32 = 1;
        let mut total_delay_count: i32 = 1;

        const MODULO_NUM: i32 = 1_000_000_007;

        secret_aware_map.insert(1, 1);

        for day_id in 1 + delay ..= n {
            if let Some(day_forget_count) = secret_aware_map.get(&(day_id - forget)) {
                total_aware_count -= day_forget_count;
            }

            if let Some(day_delay_count) = secret_aware_map.get(&(day_id - delay)) {
                total_delay_count -= day_delay_count;
            }


            let today_count = (((total_aware_count - total_delay_count) % MODULO_NUM) + MODULO_NUM) % MODULO_NUM;
            
            secret_aware_map.insert(day_id, today_count);

            total_aware_count += today_count;
            total_delay_count += today_count;

            total_aware_count = ((total_aware_count % MODULO_NUM) + MODULO_NUM) % MODULO_NUM;
            total_delay_count = ((total_delay_count % MODULO_NUM) + MODULO_NUM) % MODULO_NUM;
        }


        total_aware_count
    }
}