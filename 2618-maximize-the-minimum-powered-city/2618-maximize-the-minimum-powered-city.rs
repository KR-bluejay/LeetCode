impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let r = r as usize;
        let k = k as i64;
        let n = stations.len();
        
        let stations: Vec<i64> = stations.into_iter().map(|v| v as i64).collect();
        
        let mut left_power = i64::MAX;
        let mut right_power = k;
        let mut total_power = 0i64;
        
        let r_bound = r.min(n - 1);
        for id in 0..n {
            left_power = left_power.min(stations[id]);
            right_power += stations[id];
            if id <= r_bound {
                total_power += stations[id];
            }
        }
        
        let mut station_sums = vec![0i64; n];
        station_sums[0] = total_power;
        
        for id in 1..n {
            if id > r {
                total_power -= stations[id - r - 1];
            }
            if id + r < n {
                total_power += stations[id + r];
            }
            station_sums[id] = total_power;
        }
        
        let mut result = left_power;
        let mut add_powers = vec![0i64; n];
        
        while left_power <= right_power {
            let mid_power = left_power + (right_power - left_power) / 2;
            
            add_powers.fill(0);
            let mut cur_add_power = 0i64;
            let mut extra = 0i64;
            let mut possible = true;
            
            for id in 0..n {
                if id > 0 {
                    if id > r {
                        cur_add_power -= add_powers[id - r - 1];
                    }
                    if id + r < n {
                        cur_add_power += add_powers[id + r];
                    }
                }
                
                let actual_power = station_sums[id] + cur_add_power;
                
                if actual_power < mid_power {
                    let expand_power = mid_power - actual_power;
                    extra += expand_power;
                    
                    if extra > k {
                        possible = false;
                        break;
                    }
                    
                    cur_add_power += expand_power;
                    let install_pos = (id + r).min(n - 1);
                    add_powers[install_pos] += expand_power;
                }
            }
            
            if possible {
                result = mid_power;
                left_power = mid_power + 1;
            } else {
                right_power = mid_power - 1;
            }
        }
        
        result
    }
}