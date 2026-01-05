impl Solution {
    pub fn max_matrix_sum(mut matrix: Vec<Vec<i32>>) -> i64 {
        let mut neg_count = 0;
        let mut neg_value = i64::MAX;

        let mut result = 0;

        for row_id in 0 .. matrix.len() {
            for col_id in 0 .. matrix[0].len() {
                let matrix_abs = matrix[row_id][col_id].abs() as i64;

                if matrix[row_id][col_id] <= 0 {
                    neg_count += 1;
                }
                neg_value = neg_value.min(matrix_abs);
                result += matrix_abs;
            }
        }

        if neg_count % 2 == 1 {
            result -= neg_value * 2;
        }

        result
    }
}