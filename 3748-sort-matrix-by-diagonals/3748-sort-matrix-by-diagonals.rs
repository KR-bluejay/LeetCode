
impl Solution {
    fn sort_diagonal(
        grid: &mut Vec<Vec<i32>>, 
        row_id: usize, 
        col_id: usize, 
        is_asc: bool
    ) {
        let row_len = grid.len();
        let col_len = grid[0].len();
        let diagonal_len = (row_len - row_id).min(col_len - col_id);

        let mut diagonal: Vec<i32> = (0 .. diagonal_len)
            .map(|id| grid[row_id + id][col_id + id])
            .collect();
        
        if is_asc {
            diagonal.sort_unstable();
        } else {
            diagonal.sort_unstable_by(|a, b| b.cmp(a));
        }


        for (k, v) in diagonal.into_iter().enumerate() {
            grid[row_id + k][col_id + k] = v;
        }
    }
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for i in 1 .. grid.len() {
            Self::sort_diagonal(&mut grid, 0, i, true);
        }
        for i in 0 .. grid.len() {
            Self::sort_diagonal(&mut grid, i, 0, false);
        }

        grid
    }
}