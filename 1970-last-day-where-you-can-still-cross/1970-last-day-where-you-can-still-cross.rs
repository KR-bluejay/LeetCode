use std::collections::VecDeque;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let row_len = row as usize;
        let col_len = col as usize;

        let mut matrix = vec![vec![cells.len(); col_len]; row_len];
        let mut start_day = 1;
        let mut end_day = cells.len();

        let mut result = 0;

        for (day_id, cell) in cells.into_iter().enumerate() {
            matrix[cell[0] as usize - 1][cell[1] as usize - 1] = day_id + 1;
        }



        let mut visit = vec![vec![false; col_len]; row_len];
        let mut block_heap: VecDeque<(usize, usize)> = VecDeque::with_capacity(row_len * col_len);

        while start_day < end_day {
            let mid_day = start_day + (end_day - start_day) / 2;
            visit = vec![vec![false; col_len]; row_len];
            
            for col_id in 0 .. col_len {
                if mid_day < matrix[0][col_id] {
                    visit[0][col_id] = true;
                    block_heap.push_back((0, col_id))
                }
            }

            while let Some((row_id, col_id)) = block_heap.pop_front() {
                if row_id + 1 == row_len {
                    result = result.max(mid_day);
                    
                    break;
                }

                for ((next_row_id, next_col_id)) in [
                    (row_id + 1, col_id), 
                    (row_id - 1, col_id), 
                    (row_id, col_id + 1), 
                    (row_id, col_id - 1)
                ] {
                    if row_len <= next_row_id 
                    || col_len <= next_col_id 
                    || visit[next_row_id][next_col_id] 
                    || mid_day >= matrix[next_row_id][next_col_id] {
                        continue;
                    }
                    visit[next_row_id][next_col_id] = true;
                    
                    block_heap.push_back((next_row_id, next_col_id));
                }
            }

            block_heap.clear();

            if result == mid_day {
                start_day = mid_day + 1;
            } else {
                end_day = mid_day;
            }
        }


        result as i32
    }
}