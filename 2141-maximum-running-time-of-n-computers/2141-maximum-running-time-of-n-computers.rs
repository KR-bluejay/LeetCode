impl Solution {
    pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
        let compute_count = n as i64;

        let mut left_battery: i64 = 0;
        let mut right_battery = batteries.iter().fold(0, |acc, &battery| acc + battery as i64) / compute_count;

        while left_battery < right_battery {
            let mid_battery = left_battery + (right_battery - left_battery + 1) / 2;
            let mut total = batteries.iter()
                .fold(0, |acc, &battery| acc + mid_battery.min(battery as i64));

            if mid_battery * compute_count <= total {
                left_battery = mid_battery;
            } else {
                right_battery = mid_battery - 1;
            }
        }

        left_battery
    }
}