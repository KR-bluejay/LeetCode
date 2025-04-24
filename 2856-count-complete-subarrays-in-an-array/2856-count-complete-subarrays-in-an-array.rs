use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<_> = nums.iter().collect();
        let num_dist_count = num_set.len();
        
        let mut right_id: usize = 0;
        let mut complete_count: usize = 0;
        let mut num_count_map: HashMap<i32, i32> = HashMap::with_capacity(num_dist_count);

        for left_id in 0 .. (nums.len() - num_dist_count + 1) {
            if 0 < left_id {
                let prev_num_key = nums[left_id - 1];
                let mut num_value = num_count_map.get_mut(&prev_num_key).unwrap();

                if *num_value <= 1 {
                    num_count_map.remove(&prev_num_key);
                } else {
                    *num_value -= 1;
                }
            }

            while right_id < nums.len() 
            && num_count_map.len() < num_dist_count {
                num_count_map.entry(nums[right_id])
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                right_id += 1;
            }
            if num_count_map.len() == num_dist_count {
                complete_count += nums.len() - right_id + 1;
            }
        }

        complete_count as i32
    }
}