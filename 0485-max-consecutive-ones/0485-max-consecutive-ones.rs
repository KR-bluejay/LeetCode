use std::cmp;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut max_sum = 0;

        
        for right in 0 .. nums.len() {
            if nums[right] == 0 {
                left = right + 1;

                continue;
            }
            let cur_sum = right - left + 1;

            max_sum = cmp::max(cur_sum, max_sum);
        }

        max_sum as i32
    }
}