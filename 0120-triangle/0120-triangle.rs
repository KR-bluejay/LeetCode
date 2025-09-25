impl Solution {
    fn short_path(
        triangle: &Vec<Vec<i32>>, 
        row_id: usize, 
        col_id: usize, 
        prev_path: i32, 
        min_path: &mut i32,
        visited: &mut Vec<Vec<i32>>
    ) {
        if row_id >= triangle.len() || triangle[row_id].len() <= col_id || visited[row_id][col_id] <= prev_path +  triangle[row_id][col_id] {
            return;
        }

        let cur_path = prev_path + triangle[row_id][col_id];
        visited[row_id][col_id] = cur_path;

        if row_id == triangle.len() - 1 {
            *min_path = cur_path.min(*min_path);
            
            return;
        }

        // Self::short_path(triangle, row_id + 1, col_id - 1, cur_path, min_path);
        Self::short_path(triangle, row_id + 1, col_id, cur_path, min_path, visited);
        Self::short_path(triangle, row_id + 1, col_id + 1, cur_path, min_path, visited);
    }
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut min_path = i32::MAX;
        let mut visited: Vec<Vec<i32>> = vec![vec![i32::MAX; triangle[triangle.len() - 1].len()]; triangle.len()];

        Self::short_path(&triangle, 0, 0, 0, &mut min_path, &mut visited);

        min_path
    }
}