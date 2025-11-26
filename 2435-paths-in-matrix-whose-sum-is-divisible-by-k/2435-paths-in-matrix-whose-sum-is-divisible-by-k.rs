impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MODULO: i32 = 1000000007;
        let k = k as usize;
        let (m, n) = (grid.len(), grid[0].len());

        // 이전 행과 현재 행만 유지
        let mut prev = vec![vec![0; k]; n];
        let mut curr = vec![vec![0; k]; n];

        for row_id in 0..m {
            for col_id in 0..n {
                let cur_val = (grid[row_id][col_id] as usize) % k;

                if row_id == 0 && col_id == 0 {
                    curr[0][cur_val] = 1;
                    continue;
                }

                // 현재 셀 초기화
                curr[col_id].fill(0);

                for i in 0..k {
                    let prev_val = (i + k - cur_val) % k;
                    
                    if row_id > 0 {
                        curr[col_id][i] = (curr[col_id][i] + prev[col_id][prev_val]) % MODULO;
                    }
                    
                    if col_id > 0 {
                        curr[col_id][i] = (curr[col_id][i] + curr[col_id - 1][prev_val]) % MODULO;
                    }
                }
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        prev[n - 1][0]
    }
}