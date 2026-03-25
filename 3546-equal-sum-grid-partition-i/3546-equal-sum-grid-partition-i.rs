use std::cmp::Ordering;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let total_sum: i64 = grid.iter()
            .flat_map(|row| row.iter())
            .map(|&n| n as i64)
            .sum();
        let mut prefix: i64 = 0;

        for row_id in 0 .. grid.len() {
            prefix += grid[row_id].iter().map(|&n| n as i64).sum::<i64>();

            match (prefix * 2).cmp(&total_sum) {
                Ordering::Less => {},
                Ordering::Equal => return true,
                Ordering::Greater => break,
            }
        }

        prefix = 0;

        for col_id in 0 .. grid[0].len() {
            prefix += grid.iter().map(|row| row[col_id] as i64).sum::<i64>();
            
            match (prefix * 2).cmp(&total_sum) {
                Ordering::Less => {},
                Ordering::Equal => return true,
                Ordering::Greater => return false,
            }
        }

        false
    }
}