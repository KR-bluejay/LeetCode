impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|left, right| left[1].cmp(&right[1]).then_with(|| right[0].cmp(&left[0])));

        let mut inter_count = 2;
        let mut second_last_num = intervals[0][1] - 1;
        let mut last_num = intervals[0][1];
        

        for interval in intervals.into_iter().skip(1) {
            let start = interval[0];
            let end = interval[1];


            if last_num < start {
                inter_count += 2;
                second_last_num = end - 1;
                last_num = end;
            } else if second_last_num < start {
                inter_count += 1;
                second_last_num = last_num;
                last_num = end;
            }
        }

        inter_count
    }
}