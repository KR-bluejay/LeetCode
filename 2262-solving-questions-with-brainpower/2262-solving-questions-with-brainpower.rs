use std::cmp;

impl Solution {
    fn dp(
        question_id: usize, 
        questions: &Vec<Vec<i32>>, 
        scores: &mut Vec<i64>, 
    ) -> i64 {
        if question_id >= questions.len() {
            return 0;
        }
        
        if scores[question_id] != -1 {
            return scores[question_id];
        }

        // Skip
        let skip_score = Self::dp(question_id + 1, questions, scores);

        // Solve
        let question_point = questions[question_id][0] as i64;
        let brain_power = questions[question_id][1] as usize;
        let solve_score = question_point + Self::dp(question_id + brain_power + 1, questions, scores);
        
        scores[question_id] = cmp::max(solve_score, skip_score);

        scores[question_id]
    }
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut scores: Vec<i64> = vec![-1; questions.len()];

        Self::dp(0, &questions, &mut scores)
    }
}