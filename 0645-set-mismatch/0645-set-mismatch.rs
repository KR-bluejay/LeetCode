use std::collections::BTreeMap;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut num_counts: Vec<i32> = vec![0; 10000];
        let mut num_results: Vec<i32> = vec![0; 2];

        for &num in nums.iter() {
            num_counts[num as usize - 1] += 1;
        }

        for (num_id, &num_count) in num_counts.iter().enumerate() {
            if num_results[0] != 0 && num_results[1] != 0 {
                break;
            }


            if num_count == 0  {
                num_results[1] = num_id as i32 + 1;
            } else if num_count > 1 {
                num_results[0] = num_id as i32 + 1;
            }
        }

        num_results
    }
}