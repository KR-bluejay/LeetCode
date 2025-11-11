impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let max_zero = m as usize;
        let max_one = n as usize;
        
        let mut result = 0;

        let mut active: Vec<(usize, usize)> = Vec::with_capacity(strs.len());
        let mut cache: Vec<Vec<i32>> 
            = vec![vec![0; max_one + 1]; max_zero + 1];

        active.push((0, 0));

        for str_item in strs.into_iter() {
            let (zero_used, one_used) = str_item.clone().into_bytes()
                .into_iter()
                .fold((0, 0), |mut acc, val| {
                    match val {
                        b'0' => acc.0 += 1,
                        _ => acc.1 += 1,
                    }
                    acc
                });
            let mut id = active.len();
            
            while id > 0 {
                id -= 1;
                let (active_zero, active_one) = active[id];
                let next_zero = zero_used + active_zero;
                let next_one = one_used + active_one;

                if max_zero < next_zero 
                || max_one < next_one {
                    continue;
                }

                if cache[next_zero][next_one] == 0 {
                    active.push((next_zero, next_one));
                }
                cache[next_zero][next_one] = cache[next_zero][next_one]
                    .max(cache[active_zero][active_one] + 1);

                result = result.max(cache[next_zero][next_one]);
            }
            active.sort_unstable();
        }
        result
    }
}