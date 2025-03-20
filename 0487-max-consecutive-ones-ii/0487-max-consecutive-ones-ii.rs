use std::cmp;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut left_index: i32 = 0;
        let mut flip_index: i32 = -1;

        let mut max_sum: i32 = 0;

        for right_index  in 0 .. nums.len() as i32 {
            if nums[right_index as usize] == 0 {
                left_index = flip_index + 1;
                flip_index = right_index;
            }
            let cur_sum = right_index - left_index + 1;

            max_sum = cmp::max(cur_sum, max_sum);
        }

        max_sum
    }
}