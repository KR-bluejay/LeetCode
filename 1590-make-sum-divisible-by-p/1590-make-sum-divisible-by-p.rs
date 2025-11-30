use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut target = 0;
        let mut is_enough = false;
        
        for &num in nums.iter() {
            target += num;

            is_enough |= (target >= p);
            target %= p;
        }

        if target == 0 {
            return 0;
        } else if !is_enough {
            return -1;
        }
        let mut num_sum = 0;
        let mut num_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut result = i32::MAX;

        num_map.insert(0, -1);

        for (num_id, &num_val) in nums.iter().enumerate() {
            if num_val == target {
                return 1;
            }

            num_sum = (num_sum + num_val) % p;
            let needed_num = (num_sum + p - target) % p;

            if let Some(id) = num_map.get(&needed_num) {
                result = result.min(num_id as i32 - id);
            }
            num_map.insert(num_sum, num_id as i32);
        }

        if result == i32::MAX || result == nums.len() as i32 {
            -1
        } else {
            result
        }
    }
}