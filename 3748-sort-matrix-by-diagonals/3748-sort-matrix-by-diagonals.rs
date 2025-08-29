impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let grid_len = grid.len();

        for start_row in (0 .. grid_len) {
            let (mut row_id, mut col_id) = (start_row, 0);
            let mut diagonal = Vec::with_capacity(grid_len - start_row);

            while (row_id < grid_len && col_id < grid_len) {
                diagonal.push(grid[row_id][col_id]);
                row_id += 1;
                col_id += 1;
            }
            diagonal.sort_by(|a, b| b.cmp(a));

            
            let (mut row_id, mut col_id) = (start_row, 0);

            for diagonal_item in diagonal.iter() {
                grid[row_id][col_id] = *diagonal_item;
                
                row_id += 1;
                col_id += 1;
            }
        }

        for start_col in (1 .. grid_len) {
            let (mut row_id, mut col_id) = (0, start_col);
            let mut diagonal = Vec::with_capacity(grid_len - start_col);

            while (row_id < grid_len && col_id < grid_len) {
                diagonal.push(grid[row_id][col_id]);

                row_id += 1;
                col_id += 1;
            }
            diagonal.sort();


            
            let (mut row_id, mut col_id) = (0, start_col);
            
            for diagonal_item in diagonal.iter() {
                grid[row_id][col_id] = *diagonal_item;
                
                row_id += 1;
                col_id += 1;
            }
        }
        grid
    }
}