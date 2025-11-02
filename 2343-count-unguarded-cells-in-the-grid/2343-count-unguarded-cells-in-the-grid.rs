use std::collections::HashSet;

impl Solution {
    pub fn count_unguarded(
        m: i32, 
        n: i32, 
        guards: Vec<Vec<i32>>, 
        walls: Vec<Vec<i32>>
    ) -> i32 {
        let max_row_id = m as usize - 1;
        let max_col_id = n as usize - 1;
        let mut unguard_count: i32 = 0;

        let mut guarded: Vec<Vec<u8>>
            = vec![vec![0; max_col_id + 1]; max_row_id + 1];

        let guard_set: HashSet<(usize, usize)> = guards.into_iter()
            .map(|v| (v[0] as usize, v[1] as usize)).collect();

        let wall_set: HashSet<(usize, usize)> = walls.into_iter()
            .map(|v| (v[0] as usize, v[1] as usize)).collect();
        // 0: None, 1: Wall, 2: Guard
        // Top -> Bottom
        for row_id in 0 ..= max_row_id {
            let mut left_id = 0;
            let mut right_id = max_col_id;

            let mut left_guard = false;
            let mut right_guard = false;

            while left_id <= max_col_id {
                if wall_set.contains(&(row_id, left_id)) {
                    guarded[row_id][left_id] = 1;
                    left_guard = false;
                } else if guard_set.contains(&(row_id, left_id)) {
                    guarded[row_id][left_id] = 2;
                    left_guard = true;
                } else if left_guard {
                    guarded[row_id][left_id] = 2;
                }

                if wall_set.contains(&(row_id, right_id)) {
                    guarded[row_id][right_id] = 1;
                    right_guard = false;
                } else if guard_set.contains(&(row_id, right_id)) {
                    guarded[row_id][right_id] = 2;
                    right_guard = true;
                } else if right_guard {
                    guarded[row_id][right_id] = 2;
                }

                left_id += 1;
                right_id -= 1;
            }
        }
        
        // 0: None, 1: Wall, 2: Guard
        // Top -> Bottom
        for col_id in 0 ..= max_col_id {
            let mut top_id = 0;
            let mut bottom_id = max_row_id;

            let mut top_guard = false;
            let mut bottom_guard = false;

            while top_id <= max_row_id {
                if wall_set.contains(&(top_id, col_id)) {
                    guarded[top_id][col_id] = 1;
                    top_guard = false;
                } else if guard_set.contains(&(top_id, col_id)) {
                    guarded[top_id][col_id] = 2;
                    top_guard = true;
                } else if top_guard {
                    guarded[top_id][col_id] = 2;
                }

                if wall_set.contains(&(bottom_id, col_id)) {
                    guarded[bottom_id][col_id] = 1;
                    bottom_guard = false;
                } else if guard_set.contains(&(bottom_id, col_id)) {
                    guarded[bottom_id][col_id] = 2;
                    bottom_guard = true;
                } else if bottom_guard {
                    guarded[bottom_id][col_id] = 2;
                }

                top_id += 1;
                bottom_id -= 1;
            }
        }

        for row_id in 0 ..= max_row_id {
            for col_id in 0 ..= max_col_id {
                if guarded[row_id][col_id] == 0 {
                    unguard_count += 1;
                }
            }
        }
        unguard_count
    }
}