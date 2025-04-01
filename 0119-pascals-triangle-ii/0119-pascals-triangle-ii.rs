impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let triangle_size = (row_index + 1) as usize;
        let mut triangle: Vec<Vec<i32>> = vec![vec![1; triangle_size]; triangle_size];

        for i in 2 .. triangle.len() {
            for j in 1 .. i {
                triangle[i][j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
            }
        }

        triangle[row_index as usize].clone()
    }
}