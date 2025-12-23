impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by(|lhs, rhs| lhs[0].cmp(&rhs[0]));
        
        let mut suffix_scores: Vec<i32> = vec![0; events.len() + 1];
        let mut result = 0;

        for id in (0 .. events.len()).rev() {
            suffix_scores[id] = suffix_scores[id + 1].max(events[id][2]);
        }

        for event in events.iter() {
            let start_time = event[0];
            let end_time = event[1];
            let score = event[2];
            let next_id = events.partition_point(|e| e[0] <= end_time);

            result = result.max(score + suffix_scores[next_id]);
        }

        result
    }
}