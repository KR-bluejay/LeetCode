use std::collections::HashMap;
use std::cmp::Ordering;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut num_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut max_freq: i32 = 0;
        let mut max_count: i32 = 0;

        for num in &nums {
            let num_freq = *num_map.entry(*num)
                .and_modify(|v| *v += 1)
                .or_insert(1);

            match num_freq.cmp(&max_freq) {
                Ordering::Equal => {
                    max_count += 1;
                },
                Ordering::Greater => {
                    max_freq = num_freq;
                    max_count = 1;
                },
                _ => {

                },
            }
        }

        max_count * max_freq
    }
}