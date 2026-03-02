impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let mut pos = vec![-1; grid.len()];
        let mut result = 0;

        for row_id in 0 .. grid.len() {
            if let Some(col_id) = grid[row_id].iter().rposition(|&n| n == 1) {
                pos[row_id] = col_id as i32;
            }
        }

        for left_id in 0 .. grid.len() {
            let mut k = -1;

            for right_id in left_id .. grid.len() {
                if pos[right_id] <= left_id as i32 {
                    result += (right_id - left_id) as i32;
                    k = right_id as i32;

                    break;
                }
            }

            if k == -1 {
                return -1;
            }

            let k = k as usize;

            for right_id in (left_id + 1 ..= k).rev() {
                pos.swap(right_id, right_id - 1);
            }
        }


        result
    }
}