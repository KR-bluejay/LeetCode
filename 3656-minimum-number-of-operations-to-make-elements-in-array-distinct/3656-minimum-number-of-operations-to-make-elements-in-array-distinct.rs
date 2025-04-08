use std::collections::{ HashMap };

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut op_count: usize = 0;
        let mut num_map: HashMap<i32, usize> = HashMap::new();

        for (num_id, &num_item) in nums.iter().enumerate() {
            if let Some(old_num_id) = num_map.insert(num_item, num_id) {
                op_count = op_count.max((old_num_id + 3) / 3);
            }
        }

        op_count as i32
    }
}