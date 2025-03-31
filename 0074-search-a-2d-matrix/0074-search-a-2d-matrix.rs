impl Solution {
    fn get_2d_index(id: usize, m: usize) -> (usize, usize) {
        (id/m, id%m)
    }
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let max_id = matrix.len() * matrix[0].len() - 1;

        let mut start = 0;
        let mut end = max_id.clone();

        while 0 <= start && end <= max_id && start <= end {
            let mid = (start + end) / 2;
            let (m, n) = Self::get_2d_index(mid, matrix[0].len());

            println!("{m} {n} {mid}");

            if matrix[m][n] == target {
                return true;
            }

            if matrix[m][n] > target {
                end = mid - 1;
                continue;
            }

            start = mid + 1;
        }

        false
    }
}