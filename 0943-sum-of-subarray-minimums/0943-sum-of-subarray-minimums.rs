use std::ops::Range;
use std::collections::{ HashMap };

impl Solution {

    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut min_sum: i64 = 0;
        let mut sub_stack: Vec<usize> = Vec::with_capacity(arr.len());
        let mut left_to_right: Vec<i32> = vec![-1; arr.len()];
        let mut right_to_left: Vec<usize> = vec![arr.len(); arr.len()];

        for (num_id, num_val) in arr.iter().enumerate() {
            while let Some(stack_id) = sub_stack.pop() {
                let stack_num = arr[stack_id];
                if stack_num < *num_val {
                    sub_stack.push(stack_id);
                    
                    break;
                }

                left_to_right[stack_id] = num_id as i32 - 1;
            }
            sub_stack.push(num_id);
        }

        while let Some(stack_num) = sub_stack.pop() {
            left_to_right[stack_num] = (arr.len() - 1) as i32;
        }

        
        for (num_id, num_val) in arr.iter().enumerate().rev() {
            while let Some(stack_id) = sub_stack.pop() {
                let stack_num = arr[stack_id];

                if stack_num <= *num_val {
                    sub_stack.push(stack_id);
                    
                    break;
                }

                right_to_left[stack_id] = num_id + 1;
            }
            sub_stack.push(num_id);
        }

        while let Some(stack_num) = sub_stack.pop() {
            right_to_left[stack_num] = 0;
        }

        

        for (sub_id, sub_val) in arr.iter().enumerate() {
            let start = if right_to_left[sub_id] == arr.len() {
                sub_id
            } else {
                right_to_left[sub_id] as usize
            };

            let end = if left_to_right[sub_id] == -1 {
                sub_id
            } else {
                left_to_right[sub_id] as usize
            };
            let range_count = (end - sub_id + 1) * (sub_id - start + 1);

            min_sum += (*sub_val as i64 * range_count as i64) % 1000000007;
            min_sum %= 1000000007;
        }



        min_sum as i32
    }
}