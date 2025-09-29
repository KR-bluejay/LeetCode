impl Solution {
    fn get_min_score(
        values: &Vec<i32>, 
        scores: &mut Vec<Vec<i32>>, 
        start_id: usize, 
        end_id: usize
    ) -> i32 {
        if scores[start_id][end_id] != i32::MAX {
            return scores[start_id][end_id];
        }

        if start_id + 1 >= end_id {
            return 0;
        }

        if start_id + 2 == end_id {
            scores[start_id][end_id] = values[start_id] * values[start_id + 1] * values[end_id];
            
            return scores[start_id][end_id];
        }


        for i in start_id + 1 .. end_id {
            let left = Self::get_min_score(values, scores, start_id, i);
            let right = Self::get_min_score(values, scores, i, end_id);
            let remaining = values[start_id] * values[i] * values[end_id];

            scores[start_id][end_id] = scores[start_id][end_id].min(left + right + remaining);
        }


        scores[start_id][end_id]
    }
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let mut scores: Vec<Vec<i32>> = vec![vec![i32::MAX; values.len()]; values.len()];


        Self::get_min_score(&values, &mut scores, 0, values.len() - 1)
    }
}