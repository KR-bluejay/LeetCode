use std::cmp::{ Ordering };
use std::collections::{ HashMap };

impl Solution {
    fn get_gcd(mut left: i32, mut right: i32) -> i32 {
        if right == 0 {
            left
        } else {
            Self::get_gcd(right, left % right)
        }
    }
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return nums;
        }

        let mut outputs: Vec<i32> = Vec::with_capacity(nums.len());
        let mut num_id = 1;

        outputs.push(nums[0]);

        while 0 <= num_id && num_id < nums.len() {
            let last_num = outputs[outputs.len() - 1];
            let gcd_num = Self::get_gcd(last_num.clone(), nums[num_id].clone());

            if gcd_num > 1 {
                if let Some(last_element) = outputs.last_mut() {
                    *last_element = (nums[num_id] as i128 * last_num as i128 / gcd_num as i128) as i32;
                }
            } else {
                outputs.push(nums[num_id]);
            }
            num_id += 1;
        }
        let mut retryable = true;

        while retryable {
            let mut output_id = outputs.len() - 1;
            retryable = false;

            while 0 < output_id && output_id < outputs.len() {
                let prev_num = outputs[output_id - 1];
                let cur_num = outputs[output_id];
    
                let gcd_num = Self::get_gcd(prev_num.clone(), cur_num.clone());
    
                if gcd_num > 1 {
                    outputs[output_id - 1] = (cur_num as i128 * prev_num as i128 / gcd_num as i128) as i32;
                    outputs[output_id] = -1;
                    retryable = true;
                }
                output_id -= 1;
            }
    
            outputs = outputs.into_iter().filter(|v| *v != -1).collect();
        }

        outputs
    }
}