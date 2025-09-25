impl Solution {
    fn shortest_path(
        triangle: &Vec<Vec<i32>>,
        cache: &mut Vec<Vec<i32>>,
        row_id: usize,
        col_id: usize,
    ) -> i32 {
        if triangle.len() <= row_id 
        || triangle[row_id].len() <= col_id {
            return 0;
        }

        if cache[row_id][col_id] != i32::MAX {
            return cache[row_id][col_id];
        }
        let path_a = Self::shortest_path(triangle, cache, row_id + 1, col_id);
        let path_b = Self::shortest_path(triangle, cache, row_id + 1, col_id + 1);


        cache[row_id][col_id] = triangle[row_id][col_id] + path_a.min(path_b);

        cache[row_id][col_id]
    }
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut cache: Vec<Vec<i32>> = vec![vec![i32::MAX; triangle[triangle.len() - 1].len()]; triangle.len()];

        Self::shortest_path(&triangle, &mut cache, 0, 0)
    }
}