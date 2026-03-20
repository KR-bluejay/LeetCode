impl Solution {
    pub fn min_number_of_seconds(
        mountain_height: i32, 
        worker_times: Vec<i32>
    ) -> i64 {
        let mountain_height = mountain_height as i64;
        let worker_max_time = *worker_times.iter().max().unwrap() as i64;
        
        let mut left = 0;

        // worker_max_time * (1 + .. + mountain_height)
        // worker_max_time * [mountain_height * (mountain_height + 1) / 2]
        // worker_max_time * [(mountain_height.pow(2) * mounta_height) / 2]
        let mut right = worker_max_time * mountain_height * (mountain_height + 1) / 2;
        let mut result = 0;

        while left <= right {
            let mid = (left + right) / 2;
            let mut count = 0;

            for &worker_time in worker_times.iter() {
                let work = (mid / worker_time as i64) as f64;
                // mid = worker_time * k * (k + 1) / 2
                // mid / worker_time = k * (k + 1) / 2
                // 2 * mid / worker_time = k * (k + 1)
                // 2 * mid / worker_time = k.pow(2) + k
                // k.pow(2) + k - 2 * mid / worker_time = 0
                // [-1 + (1 + 8 * mid / worker_time).sqrt()] / -2
                let k = ((-1.0 + (1.0 + 8.0 * work).sqrt()) / 2.0) as i64;
                
                count += k;
            }

            if count >= mountain_height {
                result = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }


        result
    }
}