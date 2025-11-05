impl Solution {
    #[inline(always)]
    pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut max_board_len = fruits.len() - 1;


        for i in 0 .. fruits.len() {
            result += fruits[i][i];
            
            fruits[i][i] = 0;
        }

        let mut board: Vec<Vec<i32>> = vec![vec![-1; fruits.len()]; fruits.len()];
        board[0][max_board_len] = fruits[0][max_board_len];

        for row_id in 0 .. max_board_len {
            let next_row_id = row_id + 1;

            for col_id in max_board_len / 2 ..= max_board_len {
                if board[row_id][col_id] == -1 {
                    continue;
                }

                let cur_score = board[row_id][col_id];
                let next_col_ids = [col_id - 1, col_id, col_id + 1];

                for &next_col_id in next_col_ids.iter() {
                    if max_board_len < next_row_id || max_board_len < next_col_id {
                        continue;
                    }
                    board[next_row_id][next_col_id] 
                        = board[next_row_id][next_col_id]
                            .max(cur_score + fruits[next_row_id][next_col_id]);
                }
            }
        }
        result += board[max_board_len][max_board_len];
        board = vec![vec![-1; fruits.len()]; fruits.len()];

        board[max_board_len][0] = fruits[max_board_len][0];
        

        for col_id in 0 .. max_board_len {
            let next_col_id = col_id + 1;

            for row_id in max_board_len / 2 ..= max_board_len {
                if board[row_id][col_id] == -1 {
                    continue;
                }

                let cur_score = board[row_id][col_id];
                let next_row_ids: [usize; 3] = [row_id - 1, row_id, row_id + 1];

                for &next_row_id in next_row_ids.iter() {
                    if max_board_len < next_row_id || max_board_len < next_col_id {
                        continue;
                    }

                    board[next_row_id][next_col_id] 
                        = board[next_row_id][next_col_id]
                            .max(cur_score + fruits[next_row_id][next_col_id]);
                }
            }
        }
        result + board[max_board_len][max_board_len]
    }
}