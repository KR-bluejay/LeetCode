use std::collections::HashMap;

impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut num_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut num_count_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());


        let mut max_key = i32::MAX;
        let mut max_count = 0;

        for num in nums {
            let key = (num % space);
            let num_count = *num_count_map.entry(key).and_modify(|v| *v += 1).or_insert(1);
            let num_key = *num_map.entry(key).and_modify(|v| {
                if *v > num {
                    *v = num;
                }
            }).or_insert(num);

            if (num_count > max_count) || (max_count == num_count && num_key < max_key) {
                max_key = num_key;
                max_count = num_count;
            }
        }

        max_key
    }
}