impl Solution {
    pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
        power.sort();
        
        let mut values: Vec<i32> = Vec::new();
        let mut gains: Vec<i64> = Vec::new();

        values.push(power[0]);
        gains.push(power[0] as i64);

        let mut prev_val: i32 = power[0];

        for &power_val in power.iter().skip(1) {
            if prev_val == power_val {
                *gains.last_mut().unwrap() += power_val as i64;
            } else {
                values.push(power_val);
                gains.push(power_val as i64);
                
                prev_val = power_val;
            }
        }

        let mut bests: Vec<i64> = vec![0; values.len()];

        bests[0] = gains[0];

        for i in 1 .. values.len() {
            let mut prev_best = 0;

            for j in (i.saturating_sub(3) .. i).rev() {
                if values[j] < values[i] - 2 {
                    prev_best = bests[j];
                    
                    break;
                }
            }

            bests[i] = bests[i - 1].max(gains[i] + prev_best);
        }


        *bests.last().unwrap()
    }
}