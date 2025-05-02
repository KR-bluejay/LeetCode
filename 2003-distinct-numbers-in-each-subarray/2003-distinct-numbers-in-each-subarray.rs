use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn distinct_numbers(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut num_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        let mut results: Vec<i32> = Vec::with_capacity(nums.len() - k + 1);
        for i in 0 .. k - 1 {
            num_map.entry(nums[i])
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        for i in k - 1 .. nums.len() {

            num_map.entry(nums[i])
                .and_modify(|v| *v += 1)
                .or_insert(1);

            results.push(num_map.len() as i32);

            let num_count = num_map.entry(nums[i + 1 - k])
                .and_modify(|v| *v -= 1);

            if *num_map.get(&nums[i + 1 - k]).unwrap() == 0 {
                num_map.remove(&nums[i + 1 - k]);
            }
        }
        results
    }
}