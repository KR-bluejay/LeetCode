impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let matrix_size = n as usize;
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; matrix_size]; matrix_size];

        for query in queries {
            let (top_row_id, top_col_id) = (query[0] as usize, query[1] as usize);
            let (bottom_row_id, bottom_col_id) = (query[2] as usize, query[3] as usize);

            matrix[top_row_id][top_col_id] += 1;

            if bottom_col_id + 1 < matrix_size {
                matrix[top_row_id][bottom_col_id + 1] -= 1;
            }

            if bottom_row_id + 1 < matrix_size {
                matrix[bottom_row_id + 1][top_col_id] -= 1;
            }

            if bottom_row_id + 1 < matrix_size && bottom_col_id + 1 < matrix_size {
                matrix[bottom_row_id + 1][bottom_col_id + 1 ] += 1;
            }
        }

        for row_id in 0 .. matrix_size {
            for col_id in 0 .. matrix_size {
                if 0 < row_id {
                    matrix[row_id][col_id] += matrix[row_id - 1][col_id];
                }

                if 0 < col_id {
                    matrix[row_id][col_id] += matrix[row_id][col_id - 1];
                }

                if 0 < row_id && 0 < col_id {
                    matrix[row_id][col_id] -= matrix[row_id - 1][col_id - 1];
                }
            }
        }

        matrix
    }
}