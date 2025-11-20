use std::collections::HashSet;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|lhs, rhs| lhs[1].cmp(&rhs[1]).then_with(|| lhs[0].cmp(&rhs[0])));
    
        let mut interval_nums: Vec<i32> = Vec::with_capacity(intervals.len() * 2);

        interval_nums.push(intervals[0][1] - 1);
        interval_nums.push(intervals[0][1]);

        for interval in intervals.into_iter().skip(1) {
            let start_num = interval[0];
            let end_num = interval[1];
            let last_id = interval_nums.len() - 1;

            let is_first_contain = start_num <= interval_nums[last_id - 1];
            let is_second_contain = start_num <= interval_nums[last_id];

            if is_first_contain && is_second_contain {
                continue;
            }

            if is_first_contain || is_second_contain {
                if interval_nums[last_id] == end_num {
                    interval_nums.push(end_num - 1);
                } else {
                    interval_nums.push(end_num);
                }
            } else if !(is_first_contain && is_second_contain) {
                interval_nums.push(end_num - 1);
                interval_nums.push(end_num);
            }
        }


        interval_nums.len() as i32
    }
}