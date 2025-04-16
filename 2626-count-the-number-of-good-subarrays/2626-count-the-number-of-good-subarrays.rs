use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut num_pair_count = 0;
        let mut good_subarray_count = 0;

        let mut left_id: usize = 0;
        let mut num_freq_map: HashMap<i32, i32> = HashMap::new();
        
        for (right_id, &right_val) in nums.iter().enumerate() {
            num_pair_count += *num_freq_map.entry(right_val).or_insert(0);
            *num_freq_map.entry(right_val).or_insert(0) += 1;


            while num_pair_count >= k && left_id < right_id {
                let left_val = nums[left_id];

                num_freq_map.entry(left_val).and_modify(|v| *v -= 1);
                num_pair_count -= *num_freq_map.get(&left_val).unwrap();
                

                left_id += 1;
                good_subarray_count += (nums.len() - right_id) as i64;
            }
        }
        good_subarray_count
    }
}