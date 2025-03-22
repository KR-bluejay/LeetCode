impl Solution {
    fn explore_island(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        visited[i][j] = true;

        
        if i + 1 < grid.len() && grid[i + 1][j] == '1' && !visited[i + 1][j] {
            Self::explore_island(grid, visited, i + 1, j);
        }

        if 0 < i && grid[i - 1][j] == '1' && !visited[i - 1][j] {
            Self::explore_island(grid, visited, i - 1, j);
        }

        if j + 1 < grid[0].len() && grid[i][j + 1] == '1' && !visited[i][j + 1] {
            Self::explore_island(grid, visited, i, j + 1);
        }

        if 0 < j && grid[i][j - 1] == '1' &&!visited[i][j - 1] {
            Self::explore_island(grid, visited, i, j - 1);
        }

    }
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut island_count = 0;


        for i in 0 .. grid.len() {
            for j in 0 .. grid[i].len() {
                if grid[i][j] == '1' && !visited[i][j] {
                    Self::explore_island(&grid, &mut visited, i, j);

                    island_count += 1;
                }
            }
        }

        island_count
    }
}