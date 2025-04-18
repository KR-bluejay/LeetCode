impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let max_row_id = obstacle_grid.len() - 1;
        let max_column_id = obstacle_grid[max_row_id].len() - 1;

        for i in 0 .. obstacle_grid.len() {
            for j in 0 .. obstacle_grid[i].len() {
                if obstacle_grid[i][j] == 1 {
                    if (i == max_row_id && j == max_column_id) || (i == 0  && j == 0) {
                        return 0;
                    }

                    obstacle_grid[i][j] = -1;
                }
            }
        }


        obstacle_grid[max_row_id][max_column_id] = 1;

        for i in (0 .. obstacle_grid.len()).rev() {
            for j in (0 .. obstacle_grid[i].len()).rev() {
                if obstacle_grid[i][j] == -1 || obstacle_grid[i][j] == 0 {
                    continue;
                }

                if i > 0 && obstacle_grid[i - 1][j] != -1 {
                    obstacle_grid[i - 1][j] += obstacle_grid[i][j];
                }

                if j > 0 && obstacle_grid[i][j - 1] != -1 {
                    obstacle_grid[i][j - 1] += obstacle_grid[i][j];
                }
            }
        } 

        obstacle_grid[0][0]
    }
}