impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MODULO: u32 = 1_000_000_007;
        let (max_row_id, max_col_id) = (grid.len() - 1, grid[0].len() - 1);
        let k = k as usize;

        let mut prev_row: Vec<Vec<u32>> = vec![vec![0; k]; max_col_id + 1];
        let mut cur_row: Vec<Vec<u32>> = vec![vec![0; k]; max_col_id + 1];

        for row_id in 0 ..= max_row_id {
            for col_id in 0 ..= max_col_id {
                let cur_val = (grid[row_id][col_id] as usize % k);
                
                if row_id == 0 && col_id == 0 {
                    cur_row[0][cur_val] = 1;

                    continue;
                }

                for i in 0 .. k {
                    let prev_val = (i + k - cur_val) % k;

                    if row_id > 0 {
                        cur_row[col_id][i] += prev_row[col_id][prev_val];
                    }
                    if col_id > 0 {
                        cur_row[col_id][i] += cur_row[col_id - 1][prev_val];
                    }

                    cur_row[col_id][i] %= MODULO;
                }
            }

            if row_id != max_row_id {
                std::mem::swap(&mut prev_row, &mut cur_row);
                cur_row.iter_mut().for_each(|v| v.fill(0));
            }
        }

        cur_row[max_col_id][0] as i32
    }
}