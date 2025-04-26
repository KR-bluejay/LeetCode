use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut valid_id = 0;
        let mut bound_count: i64 = 0;
        let mut min_id: i64 = -1;
        let mut max_id: i64 = -1;

        for (num_id, &num_item) in nums.iter().enumerate() {
            if num_item < min_k || max_k < num_item {
                min_id = -1;
                max_id = -1;

                valid_id = (num_id + 1) as i64;
                
                continue;
            }

            if num_item == min_k {
                min_id = num_id as i64;
            }
            if num_item == max_k {
                max_id = num_id as i64;
            }

            if min_id == -1 || max_id == -1 {
                continue;
            }

            let target_id = min_id.min(max_id);

            bound_count += (target_id - valid_id + 1) as i64; 
        }
        bound_count
    }
}