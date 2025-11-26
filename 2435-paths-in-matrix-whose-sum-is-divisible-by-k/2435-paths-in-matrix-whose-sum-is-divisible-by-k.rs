impl Solution {
    fn find_paths(
        grid: &Vec<Vec<i32>>,
        cache: &mut Vec<Vec<Vec<i32>>>,
        row_id: usize,
        col_id: usize,
        k: i32,
        prev_remainder: i32,
    ) -> i32 {
        const MODULO: i32 = 1000000007;

        if grid.len() <= row_id || grid[0].len() <= col_id {
            return 0;
        }
        let cur_remainder = (prev_remainder + grid[row_id][col_id]) % k;

        if cache[row_id][col_id][cur_remainder as usize] != -1 {
            return cache[row_id][col_id][cur_remainder as usize];
        }


        if row_id + 1 == grid.len() && col_id + 1 == grid[0].len() {
            cache[row_id][col_id][cur_remainder as usize] = (cur_remainder == 0) as i32;

            return cache[row_id][col_id][cur_remainder as usize];
        }

        let mut path_count = if row_id + 1 < grid.len() {
            Self::find_paths(
                grid, 
                cache, 
                row_id + 1,
                col_id, 
                k, 
                cur_remainder
            ) % MODULO
        } else {
            0
        };

        path_count += if col_id + 1 < grid[0].len() {
            Self::find_paths(
                grid, 
                cache, 
                row_id,
                col_id + 1, 
                k, 
                cur_remainder,
            ) % MODULO
        } else {
            0
        };

        cache[row_id][col_id][cur_remainder as usize] = path_count % MODULO;
        cache[row_id][col_id][cur_remainder as usize]
    }
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut cache: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; k as usize]; grid[0].len()]; grid.len()];

        Self::find_paths(&grid, &mut cache, 0, 0, k, 0)
    }
}