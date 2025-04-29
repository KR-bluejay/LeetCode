use std::collections::BTreeMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let num_max = *nums.iter().max().unwrap_or(&i32::MAX);
        let mut num_max_id_list: Vec<usize> = Vec::with_capacity(nums.len());
        let mut num_count = 0;

        for (num_id, &num_val) in nums.iter().enumerate() {
            if num_val == num_max {
                num_max_id_list.push(num_id);
            }

            if num_max_id_list.len() < k {
                continue;
            }
            num_count += num_max_id_list[num_max_id_list.len() - k] + 1;
        }
        num_count as i64


    }
}