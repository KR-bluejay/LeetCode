impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {

        for i in (0 .. grid.len() - 1).rev() {
            let size = grid[0].len() - 1;
            grid[i][size] += grid[i + 1][size];
        }
        for i in (0 .. grid[0].len() - 1).rev() {
            let size = grid.len() - 1;
            grid[size][i] += grid[size][i + 1];
        }
        
        for i in (0 .. grid.len() - 1).rev() {
            for j in (0 .. grid[0].len() - 1).rev() {
                if i == j {
                    grid[i][j] += grid[i + 1][j].min(grid[i][j + 1]);
                } else if i + 1 < grid.len() && j + 1 < grid[i].len() {
                    grid[i][j] += grid[i + 1][j].min(grid[i][j + 1]);
                }
            }
        }
        grid[0][0]
    }
}