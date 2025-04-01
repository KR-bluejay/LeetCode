impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let triangle_size = (row_index + 1) as usize;
        let mut triangle_row: Vec<i32> = Vec::with_capacity(triangle_size);

        triangle_row.push(1);

        for i in 1 .. triangle_size {
            for j in (1 .. i).rev() {
                triangle_row[j] = triangle_row[j] + triangle_row[j - 1];
            }
            triangle_row.push(1);
        }
        triangle_row
    }
}