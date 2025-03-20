use std::collections::BTreeSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut num_set: BTreeSet<i32> = nums.into_iter().collect();
        
        let mut cur_len = 0;
        let mut max_len = 1;
        let mut prev_num = i32::MAX;

        for num_item in num_set {
            if prev_num + 1 == num_item {
                cur_len += 1;
                max_len = max_len.max(cur_len);
                prev_num = num_item;
                continue;
            }

            prev_num = num_item;
            cur_len = 1;
        }

        max_len
    }
}