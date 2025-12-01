use std::collections::{VecDeque};

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut matrix_history: Vec<Vec<bool>> = vec![vec![false; matrix[0].len()]; matrix.len()];

        let lower_pos: [(i32, i32); 2] = [(0, -1), (-1, 0)];
        let upper_pos: [(i32, i32); 2] = [(0, 1), (1, 0)];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(matrix.len());

        queue.push_back((matrix.len() / 2, matrix[0].len() / 2));

        while let Some((row_id, col_id)) = queue.pop_front(){
            if matrix_history[row_id][col_id] {
                continue;
            }

            if matrix[row_id][col_id] == target {
                return true;
            }
            matrix_history[row_id][col_id] = true;

            let next_pos = if matrix[row_id][col_id] > target {
                lower_pos
            } else {
                upper_pos
            };
            let mut next_row_id = row_id;
            let mut next_col_id = col_id;

            let mut diff = i32::MAX;

            for (pos_row, pos_col) in next_pos.iter() {
                let n_row_id = (row_id as i32 + pos_row) as usize;
                let n_col_id = (col_id as i32 + pos_col) as usize;

                if matrix.len() <= n_row_id 
                || matrix[0].len() <= n_col_id 
                || matrix_history[n_row_id][n_col_id] {
                    continue;
                }

                if matrix[n_row_id][n_col_id] == target {
                    return true;
                }

                queue.push_back((n_row_id, n_col_id));


                // if (matrix[n_row_id][n_col_id] - target).abs() <= diff {
                    // queue.push_back((n_row_id, n_col_id));
                    // diff = (matrix[n_row_id][n_col_id] - target).abs();
                    // println!("{row_id} {col_id} {next_row_id} {next_col_id} {diff} {}", matrix[n_row_id][n_col_id]);
                // }
            }
        }

        false
    }
}