impl Solution {
    fn explore_path(
        row_id: usize, 
        column_id: usize, 
        triangle: &Vec<Vec<i32>>, 
        cache: &mut Vec<Vec<i32>>
    ) -> i32 {
        if row_id >= triangle.len() || column_id >= triangle[row_id].len() {
            return 0;
        }


        if cache[row_id][column_id] != i32::MIN {
            return cache[row_id][column_id];
        }

        cache[row_id][column_id] = triangle[row_id][column_id] + Self::explore_path(row_id + 1, column_id, triangle, cache).min(Self::explore_path(row_id + 1, column_id + 1, triangle, cache));


        cache[row_id][column_id]
    }
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut cache: Vec<Vec<i32>> = vec![vec![i32::MIN; triangle[triangle.len() - 1].len()]; triangle.len()];
        Self::explore_path(0, 0, &triangle, &mut cache)
    }
}