impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort();
        
        let mut idx = meetings.len() - 1;


        while 0 < idx {
            let mut prev_start = meetings[idx-1][0];
            let mut prev_end = meetings[idx-1][1];

            let mut cur_start = meetings[idx][0];
            let mut cur_end = meetings[idx][1];

            if prev_end < cur_start - 1 {
                idx -= 1;
                continue;
            }
            
            if prev_end <= cur_end {
                meetings[idx-1][1] = cur_end;
            }

            meetings.remove(idx);

            if idx >= meetings.len() {
                idx -= 1;
            }
        }


        let mut working_count = meetings[0][0] - 1;

        for i in 1 .. meetings.len() {
            let prev_end = meetings[i - 1][1];
            let cur_start = meetings[i][0];

            let cur_diff = cur_start - prev_end - 1;


            if cur_diff > 0 {
                working_count += cur_diff;
            }
        }

        let diff = days - meetings[meetings.len() - 1][1];

        if  diff > 0 {
            working_count += diff;
        }

        working_count
    }
}