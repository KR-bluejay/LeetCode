use std::collections::VecDeque;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut op_count: i32 = target[0];
        let mut prev_num: i32 = target[0];

        for &cur_num in target.iter().skip(1) {
            if prev_num < cur_num {
                op_count += cur_num - prev_num;
            }
            prev_num = cur_num;
        }
        
        op_count
    }
}