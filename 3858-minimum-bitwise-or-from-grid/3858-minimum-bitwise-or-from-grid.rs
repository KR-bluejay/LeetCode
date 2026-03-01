impl Solution {
    pub fn minimum_or(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for id in (0 ..= 17).rev() {
            let bit_mask = result | ((1 << id) - 1);
            let mut possible = true;

            for row_id in 0 .. grid.len() {
                let mut row_satisfied = false;

                for col_id in 0 .. grid[0].len() {
                    if (grid[row_id][col_id] | bit_mask) == bit_mask {
                        row_satisfied = true;
                        break;
                    }
                }

                if !row_satisfied {
                    possible = false;
                    break;
                }
            }

            if !possible {
                result |= 1 << id;
            }
        }

        result
    }
}