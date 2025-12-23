impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|e| e[0]);
        
        let n = events.len();
        let mut suffix_max = vec![0; n + 1];
        for i in (0..n).rev() {
            suffix_max[i] = suffix_max[i + 1].max(events[i][2]);
        }

        let mut result = 0;

        for event in &events {
            let start = event[0];
            let end = event[1];
            let val = event[2];

            result = result.max(val);

            let idx = events.partition_point(|e| e[0] <= end);

            if idx < n {
                result = result.max(val + suffix_max[idx]);
            }
        }

        result
    }
}