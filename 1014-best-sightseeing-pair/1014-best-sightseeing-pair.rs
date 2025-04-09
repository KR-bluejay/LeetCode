impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut max_score = 0;
        let mut max_left_score = 0;

        for (score_id, &score_value) in values.iter().enumerate() {
            let score_id_i32 = score_id as i32;
            let cur_score = score_value - score_id_i32;

            max_score = max_score.max(max_left_score + cur_score);
            max_left_score = max_left_score.max(score_value + score_id_i32);
        }
        max_score
    }
}