impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let grid_len = grid.len();

        for init_id in (0 .. grid_len) {
            let mut row_id = init_id;
            let mut col_id = 0;
            let mut temp = Vec::with_capacity(grid_len - init_id);

            while (row_id < grid_len && col_id < grid_len) {
                temp.push(grid[row_id][col_id]);
                row_id += 1;
                col_id += 1;
            }
            temp.sort_by(|a, b| b.cmp(a));

            
            let mut row_id = init_id;
            let mut col_id = 0;
            let mut temp_id = 0;
            
            for temp_id in 0 .. temp.len() {
                grid[row_id][col_id] = temp[temp_id];
                row_id += 1;
                col_id += 1;
            }

        }

        for init_id in (1 .. grid_len) {
            let mut row_id = 0;
            let mut col_id = init_id;
            let mut temp = Vec::with_capacity(grid_len - init_id);

            while (row_id < grid_len && col_id < grid_len) {
                temp.push(grid[row_id][col_id]);

                row_id += 1;
                col_id += 1;
            }
            temp.sort();


            
            let mut row_id = 0;
            let mut col_id = init_id;

            for temp_id in 0 .. temp.len() {
                grid[row_id][col_id] = temp[temp_id];
                row_id += 1;
                col_id += 1;
            }
        }
        grid
    }
}