impl Solution {
    pub fn min_operations(mut grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut opt_count: i32 = 0;

        if grid.len() == 1 && grid[0].len() == 1 {
            return opt_count;
        }

        let mut grid: Vec<i32> = grid.into_iter().flatten().collect();
        
        grid.sort();

        let ref_num = grid[grid.len() / 2];

        for grid_item in grid.iter() {
            let diff_num = (grid_item - ref_num).abs();
            
            if diff_num % x != 0 {
                return -1
            }

            opt_count += diff_num / x;
        }



        opt_count
    }
}