use std::collections::BTreeSet;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut num_set: BTreeSet<i32> = BTreeSet::new();

        for row_id in 1 .. grid.len() - 1 {
            for col_id in 1 .. grid[0].len() - 1 {
                num_set.clear();
                num_set.extend([
                    grid[row_id - 1][col_id - 1],
                    grid[row_id - 1][col_id],
                    grid[row_id - 1][col_id + 1],
                    grid[row_id][col_id - 1],
                    grid[row_id][col_id],
                    grid[row_id][col_id + 1],
                    grid[row_id + 1][col_id - 1],
                    grid[row_id + 1][col_id],
                    grid[row_id + 1][col_id + 1]
                ]);

                if num_set.len() < 9 || num_set.contains(&0) || num_set.range(10..).count() > 0 {
                    continue;
                }
                
                let res = (grid[row_id - 1][col_id - 1] 
                + grid[row_id - 1][col_id] 
                + grid[row_id - 1][col_id + 1]);

                if res != (grid[row_id - 1][col_id - 1] 
                + grid[row_id][col_id - 1] 
                + grid[row_id + 1][col_id - 1]) {
                    continue;
                }
                
                if res != (grid[row_id - 1][col_id] 
                + grid[row_id][col_id] 
                + grid[row_id + 1][col_id]) {
                    continue;
                }
                
                if res != (grid[row_id - 1][col_id + 1] 
                + grid[row_id][col_id + 1] 
                + grid[row_id + 1][col_id + 1]) {
                    continue;
                }
                
                if res != 
                (grid[row_id][col_id - 1]
                + grid[row_id][col_id]
                + grid[row_id][col_id + 1]) {
                    continue;
                }
                
                if res != (grid[row_id + 1][col_id - 1]
                + grid[row_id + 1][col_id]
                + grid[row_id + 1][col_id + 1]) {
                    continue;
                }
                
                if res != (grid[row_id - 1][col_id - 1]
                + grid[row_id][col_id]
                + grid[row_id + 1][col_id + 1]) {
                    continue;
                }
                
                if res != (grid[row_id - 1][col_id + 1]
                + grid[row_id][col_id]
                + grid[row_id + 1][col_id - 1]) {
                    continue;
                }

                result += 1;
            }
        }

        result
    }
}