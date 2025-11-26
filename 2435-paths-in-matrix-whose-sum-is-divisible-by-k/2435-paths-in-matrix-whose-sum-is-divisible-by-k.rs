impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MODULO: i32 = 1000000007;
        let k = k as usize;

        let mut cache: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; k]; grid[0].len()]; grid.len()];
        let mut total_path_count = 0;

        for row_id in 0 .. grid.len() {
            for col_id in 0 .. grid[0].len() {
                let cur_val = (grid[row_id][col_id] as usize % k);

                if row_id == 0 && col_id == 0 {
                    cache[row_id][col_id][cur_val] = 1;

                    continue;
                }

                for i in 0 .. k {
                    let prev_val = ((i + k) - cur_val) % k;
    
                    if row_id > 0 {
                        cache[row_id][col_id][i] += cache[row_id - 1][col_id][prev_val];
                    }
    
                    if col_id > 0 {
                        cache[row_id][col_id][i] += cache[row_id][col_id - 1][prev_val];
                    }
        
                    cache[row_id][col_id][i] %= MODULO;
                }
            }
        }


        
        cache[grid.len() - 1][grid[0].len() - 1][0]
    }
}