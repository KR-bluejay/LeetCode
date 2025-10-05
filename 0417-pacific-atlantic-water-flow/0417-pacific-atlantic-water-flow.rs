impl Solution {
    fn dfs(
        heights: &Vec<Vec<i32>>, 
        visit: &mut Vec<Vec<bool>>,
        row_id: usize, 
        col_id: usize
    ) {
        let directions: [(i32, i32); 4] = [(1,0), (0, 1), (0, -1), (-1, 0)];


        if heights.len() <= row_id || heights[0].len() <= col_id || visit[row_id][col_id] {
            return;
        }
        visit[row_id][col_id] = true;


        for (dx, dy) in directions.iter() {
            let nx = (col_id as i32 + dx) as usize;
            let ny = (row_id as i32 + dy) as usize;

            if heights.len() <= ny || heights[0].len() <= nx || heights[ny][nx] < heights[row_id][col_id] {
                continue;
            }

            Self::dfs(heights, visit, ny, nx);
        }
    }
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut pacific_visit: Vec<Vec<bool>> = vec![vec![false; heights[0].len()]; heights.len()];
        let mut atlantic_visit: Vec<Vec<bool>> = vec![vec![false; heights[0].len()]; heights.len()];
        let mut results: Vec<Vec<i32>> = Vec::with_capacity(heights.len());

        for i in 0 .. heights.len() {
            Self::dfs(&heights, &mut pacific_visit, i, 0);
            Self::dfs(&heights, &mut atlantic_visit, i, heights[i].len() - 1);
        }

        for i in 0 .. heights[0].len() {
            Self::dfs(&heights, &mut pacific_visit, 0, i);
            Self::dfs(&heights, &mut atlantic_visit, heights.len() - 1, i);
        }

        for i in 0 .. heights.len() {
            for j in 0 .. heights[i].len() {
                if pacific_visit[i][j] && atlantic_visit[i][j] {
                    results.push(vec![i as i32, j as i32]);
                }
            }
        }

        results
    }
}