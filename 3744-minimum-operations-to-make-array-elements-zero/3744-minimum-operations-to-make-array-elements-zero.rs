use std::collections::BTreeMap;

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut op_count: i64 = 0;


        for query in queries {
            let mut left = query[0] as i32;
            let mut right = query[1] as i32;

            let mut left_float = query[0] as f64;
            let mut right_float = query[1] as f64;

            let mut left_log = left_float.log(4.0).floor() as i64;
            let mut right_log = right_float.log(4.0).floor() as i64;


            if left_log == right_log {
                op_count += ((right - left + 1) as f64 * (left_log + 1) as f64 / 2.0).ceil() as i64;

                continue;
            }


            let next_left = 4_i32.pow(left_log as u32 + 1) - 1;
            let mut cur_op_count = 0;
            cur_op_count += (next_left - left + 1) as i64 *  (left_log + 1);

            left = next_left + 1;
            left_float = left as f64;
            left_log = left_float.log(4.0).floor() as i64;

            while left <= right {
                if left * 4 > right {
                    cur_op_count += (right_float - left_float + 1.0) as i64 * (left_log + 1);
                    op_count += (cur_op_count as f64 / 2.0).ceil() as i64;
                    
                    break;
                }

                cur_op_count += (left * 4 - left) as i64 * (left_log + 1);

                left *= 4;
                left_float = left as f64;
                left_log = left_float.log(4.0).floor() as i64;
            }
        }

        op_count
    }
}