use std::ops::Range;
use std::collections::{ HashMap };

impl Solution {

    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut min_sum: i64 = 0;
        let mut left_sub_stack: Vec<usize> = Vec::with_capacity(arr.len());
        let mut right_sub_stack: Vec<usize> = Vec::with_capacity(arr.len());
        let mut sub_map: HashMap<usize, Range<usize>> = arr.iter().enumerate().map(|(i, v)| {
            (i, Range {
                start: i,
                end: i,
            })
        }).collect();

        for num_id in 0 .. arr.len() {
            let num_val = arr[num_id];
            let rev_num_id = arr.len() - 1 - num_id;
            let rev_num_val = arr[rev_num_id];

            while let Some(stack_id) = left_sub_stack.pop() {
                let stack_num = arr[stack_id];
                if stack_num < num_val {
                    left_sub_stack.push(stack_id);
                    
                    break;
                }

                sub_map.entry(stack_id).and_modify(|v| v.end = num_id - 1);
            }

            while let Some(stack_id) = right_sub_stack.pop() {
                let stack_num = arr[stack_id];

                if stack_num <= rev_num_val {
                    right_sub_stack.push(stack_id);
                    
                    break;
                }

                sub_map.entry(stack_id).and_modify(|v| v.start = rev_num_id + 1);
            }

            left_sub_stack.push(num_id);
            right_sub_stack.push(rev_num_id);
        }

        while let Some(stack_num) = left_sub_stack.pop() {
            sub_map.entry(stack_num).and_modify(|v| v.end = arr.len() - 1);
        }


        while let Some(stack_num) = right_sub_stack.pop() {
            sub_map.entry(stack_num).and_modify(|v| v.start = 0);
        }

        for (sub_id, sub_range) in sub_map.iter() {
            let Range {start, end} = sub_range;
            let sub_val = arr[*sub_id];

            let range_count = (end - sub_id + 1) * (sub_id - start + 1);

            min_sum += (sub_val as i64 * range_count as i64) % 1000000007;
            min_sum %= 1000000007;
        }



        min_sum as i32
    }
}