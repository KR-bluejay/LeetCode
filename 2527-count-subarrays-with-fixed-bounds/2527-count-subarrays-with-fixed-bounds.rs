use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut k_map: HashMap<i32, usize> = HashMap::new();
        let mut valid_id = 0;
        let mut bound_count: i64 = 0;

        for (num_id, &num_item) in nums.iter().enumerate() {
            if num_item < min_k || max_k < num_item {
                valid_id = num_id + 1;
                k_map.clear();
                continue;
            }

            if num_item == min_k {
                k_map.insert(min_k, num_id);
            } else if num_item == max_k {
                k_map.insert(max_k, num_id);
            }

            if !k_map.contains_key(&min_k) || !k_map.contains_key(&max_k) {
                continue;
            }

            let min_k_id = k_map.get(&min_k).unwrap();
            let max_k_id = k_map.get(&max_k).unwrap();
            let target_id = min_k_id.min(max_k_id);

            bound_count += (target_id - valid_id + 1) as i64; 
        }
        bound_count
    }
}