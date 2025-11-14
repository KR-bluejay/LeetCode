impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let matrix_size = n as usize;
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; matrix_size]; matrix_size];

        for query in queries.iter() {
            let (top_row_id, top_col_id) = (query[0] as usize, query[1] as usize);
            let (bottom_row_id, bottom_col_id) = (query[2] as usize, query[3] as usize);

            for row_id in top_row_id ..= bottom_row_id {
                for col_id in top_col_id ..= bottom_col_id {
                    matrix[row_id][col_id] += 1;
                }
            }
        }

        matrix
    }
}