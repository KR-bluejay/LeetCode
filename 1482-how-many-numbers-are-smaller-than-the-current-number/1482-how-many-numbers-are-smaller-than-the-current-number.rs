use std::collections::HashMap;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums: Vec<(i32, i32)> = nums.iter()
            .enumerate()
            .map(|(id, val)| (*val, id as i32))
            .collect();
        sorted_nums.sort();
        
        let mut ranks: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        // let mut results: Vec<i32> = Vec::with_capacity(nums.len());
        let mut prev_value = -1;
        let mut prev_rank = 0;

        for (id, (cur_value, cur_id)) in sorted_nums.into_iter().enumerate() {
            if prev_value != cur_value {
                prev_value = cur_value;
                prev_rank = id;
            }
            ranks.insert(cur_value, prev_rank as i32);
        }

        nums.iter().map(|v| *ranks.get(v).unwrap()).collect()
    }
}