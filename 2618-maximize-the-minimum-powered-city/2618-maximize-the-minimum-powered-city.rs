impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let r = r as usize;
        let k = k as i64;
        let mut left_power: i64 = i64::MAX;
        let mut right_power: i64 = k;
        let stations: Vec<i64> = stations.into_iter().map(|v| v as i64).collect();
        let mut total_power: i64 = 0;
        let mut result = i64::MAX;

        for id in 0 .. stations.len() {
            left_power = left_power.min(stations[id]);
            right_power += stations[id];

            if id <= r.min(stations.len() - 1) {
                total_power += stations[id];
            }
        }
        
        let mut add_powers: Vec<i64> = vec![0; stations.len()];
        let mut station_sums: Vec<i64> = Vec::with_capacity(stations.len());
        
        station_sums.push(total_power);

        for id in 1 .. stations.len() {
            if r < id {
                total_power -= stations[id - r - 1] as i64;
            }
            if id + r < stations.len() {
                total_power += stations[id + r] as i64;
            }

            station_sums.push(total_power);
        }

        while left_power <= right_power {
            let mid_power = left_power + (right_power - left_power) / 2;
            let mut extra: i64 = 0;
            let mut cur_add_power: i64 = 0;

            add_powers.fill(0);

            for id in 0 .. stations.len() {
                if id > 0 {
                    if r < id {
                        cur_add_power -= add_powers[id - r - 1];
                    }
        
                    if id + r < stations.len() {
                        cur_add_power += add_powers[id + r];
                    }
                }

                let expand_power = (mid_power - station_sums[id] - cur_add_power).max(0);

                if expand_power > 0 {
                    extra += expand_power;
                    cur_add_power += expand_power;
                    add_powers[(id + r).min(stations.len() - 1)] += expand_power;
    
                    if extra > k {
                        break;
                    }
                }
            }

            if extra <= k {
                result = mid_power;
                left_power = mid_power + 1;
            } else {
                right_power = mid_power - 1;
            }
        }

        

        result
    }
}