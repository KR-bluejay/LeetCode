use std::collections::HashMap;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut num_map: HashMap<i32, usize> = HashMap::new();
        let mut prefix = 0;
        let mut max_len = 0;

        for (num_id, &num_item) in nums.iter().enumerate() {
            prefix += num_item;
            num_map.entry(prefix).or_insert(num_id);

            let need_val = prefix - k;
            
            if need_val == 0 {
                max_len = max_len.max(num_id + 1);
            } else if let Some(need_id) = num_map.get(&need_val) {
                max_len = max_len.max(num_id - need_id);
            }
        }
        max_len as i32
    }
}