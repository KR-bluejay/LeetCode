use std::collections::VecDeque;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut op_count: i32 = target[0];
        let mut cur_num: i32 = target[0];

        for &item in target.iter().skip(1) {
            cur_num = cur_num.min(item);

            if cur_num < item {
                op_count += item - cur_num;
                cur_num = item;
            }
        }
        
        op_count
    }
}