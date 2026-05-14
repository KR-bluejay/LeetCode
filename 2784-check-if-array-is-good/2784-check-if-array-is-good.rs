use std::collections::HashMap;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let nums_len = nums.len() as i32;

        let mut num_map: HashMap<i32, u8> = HashMap::with_capacity(nums.len());

        for num in nums.into_iter() {
            num_map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }

        for id in 1 .. nums_len - 1  {
            if let Some(&count) = num_map.get(&id) && count == 1 {
                continue;
            } else {
                return false;
            }
        }

        *num_map.get(&(nums_len - 1)).unwrap_or(&0) == 2
    }
}