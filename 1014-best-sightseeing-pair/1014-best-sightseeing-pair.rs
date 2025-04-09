impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut max_score = 0;

        for i in 0 .. values.len() - 1 {
            for j in i + 1 .. values.len() {
                max_score = max_score.max(values[i] + values[j] + i as i32 - j as i32)
            }
        }
        max_score
    }
}