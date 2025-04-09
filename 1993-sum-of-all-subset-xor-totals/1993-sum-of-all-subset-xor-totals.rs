use std::collections::{HashSet};

impl Solution {
    fn backtrack(num_id: usize, cur_num: &mut Vec<i32>, nums: &Vec<i32>) -> i32 {
        if num_id == nums.len() {
            return 0;
        }

        let mut xor_sum = 0;

        for i in num_id .. nums.len() {
            let mut temp = 0;

            cur_num.push(nums[i]);

            for j in 0 .. cur_num.len() {
                temp ^= cur_num[j];
            }
            xor_sum += temp;
            xor_sum += Self::backtrack(i + 1, cur_num, nums);
            
            cur_num.pop();
        }

        xor_sum
    }
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut cur_num: Vec<i32> = Vec::new();
        Self::backtrack(0, &mut cur_num, &nums)
    }
}